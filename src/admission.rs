// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Credential-free admission shared by clients using one caller-selected egress scope.

use crate::{Error, Result};
use std::{
    collections::VecDeque,
    fmt,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{sync::Notify, time::Instant};
use tower_layer::Layer;
use tower_service::Service;

pub(crate) const REQUESTS_PER_SECOND: u32 = 120;
const STREAM_LIMIT: usize = 20;
const ADMISSION_WINDOW: Duration = Duration::from_secs(1);

/// Local admission for clients whose traffic shares one caller-selected egress scope.
///
/// Clones share REST request capacity, provider cooldown, stream leases and
/// connector-start pacing. The owner contains no credentials or account mutation state.
#[derive(Clone)]
pub struct Admission {
    pub(crate) inner: Arc<AdmissionInner>,
}

impl fmt::Debug for Admission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Admission").finish_non_exhaustive()
    }
}

pub(crate) struct AdmissionInner {
    pub(crate) rate: Mutex<Rate>,
    policy_changed: Notify,
    pub(crate) stream_slots: Mutex<StreamSlots>,
    connection_gate: Arc<ConnectionGate>,
}

pub(crate) struct Rate {
    pub(crate) exit_reserve: u32,
    pub(crate) admitted: VecDeque<Instant>,
    pub(crate) cooldown_until: Option<Instant>,
}

pub(crate) struct StreamSlots {
    pub(crate) active: usize,
}

impl Default for Admission {
    fn default() -> Self {
        Self::new()
    }
}

impl Admission {
    /// Create an independent owner with no exit reserve.
    #[must_use]
    pub fn new() -> Self {
        Self::create(0)
    }

    /// Create an independent owner with an initial exit reserve.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `requests` is at least 120.
    pub fn with_exit_reserve(requests: u32) -> Result<Self> {
        if requests >= REQUESTS_PER_SECOND {
            return Err(Error::InvalidInput(
                "exit reserve must be less than 120 requests".into(),
            ));
        }
        Ok(Self::create(requests))
    }

    fn create(requests: u32) -> Self {
        Self {
            inner: Arc::new(AdmissionInner {
                rate: Mutex::new(Rate {
                    exit_reserve: requests,
                    admitted: VecDeque::new(),
                    cooldown_until: None,
                }),
                policy_changed: Notify::new(),
                stream_slots: Mutex::new(StreamSlots { active: 0 }),
                connection_gate: Arc::new(ConnectionGate {
                    starts: Mutex::new(ConnectionStarts::Empty),
                    #[cfg(test)]
                    wait_observer: Mutex::new(None),
                }),
            }),
        }
    }

    /// Replace the reserve without refunding spent capacity or shortening cooldown.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] for a reserve of at least 120 or a poisoned lock.
    pub fn set_exit_reserve(&self, requests: u32) -> Result<()> {
        if requests >= REQUESTS_PER_SECOND {
            return Err(Error::InvalidInput(
                "exit reserve must be less than 120 requests".into(),
            ));
        }
        let mut rate = self
            .inner
            .rate
            .lock()
            .map_err(|_| Error::InvalidInput("rate lock poisoned".into()))?;
        let changed = rate.exit_reserve != requests;
        rate.exit_reserve = requests;
        drop(rate);
        if changed {
            self.inner.policy_changed.notify_waiters();
        }
        Ok(())
    }

    pub(crate) fn exit_reserve(&self) -> Result<u32> {
        self.inner
            .rate
            .lock()
            .map(|rate| rate.exit_reserve)
            .map_err(|_| Error::InvalidInput("rate lock poisoned".into()))
    }

    pub(crate) async fn admit(&self, mutation: bool, exit: bool) -> Result<()> {
        loop {
            let changed = self.inner.policy_changed.notified();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let wait = {
                let mut rate = self
                    .inner
                    .rate
                    .lock()
                    .map_err(|_| Error::InvalidInput("rate lock poisoned".into()))?;
                let capacity = if exit {
                    REQUESTS_PER_SECOND
                } else {
                    REQUESTS_PER_SECOND - rate.exit_reserve
                };
                let now = Instant::now();
                while rate
                    .admitted
                    .front()
                    .is_some_and(|at| now.duration_since(*at) >= ADMISSION_WINDOW)
                {
                    rate.admitted.pop_front();
                }
                if let Some(until) = rate.cooldown_until.filter(|until| *until > now) {
                    until.duration_since(now)
                } else if u32::try_from(rate.admitted.len())
                    .map_err(|_| Error::Invariant("rate admission count exceeds u32"))?
                    < capacity
                {
                    rate.admitted.push_back(now);
                    return Ok(());
                } else {
                    let oldest = rate
                        .admitted
                        .front()
                        .ok_or(Error::Invariant("full rate window has no admission"))?;
                    ADMISSION_WINDOW.saturating_sub(now.duration_since(*oldest))
                }
            };
            if mutation {
                return Err(Error::RateLimited);
            }
            tokio::select! {
                () = &mut changed => {},
                () = tokio::time::sleep(wait) => {},
            }
        }
    }

    pub(crate) fn install_cooldown(&self, cooldown: Duration) {
        let mut rate = self
            .inner
            .rate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let until = Instant::now() + cooldown;
        rate.cooldown_until = Some(
            rate.cooldown_until
                .map_or(until, |existing| existing.max(until)),
        );
    }

    pub(crate) fn lease_stream(&self) -> Result<StreamLease> {
        let mut slots = self
            .inner
            .stream_slots
            .lock()
            .map_err(|_| Error::InvalidInput("stream slots lock poisoned".into()))?;
        if slots.active >= STREAM_LIMIT {
            return Err(Error::RateLimited);
        }
        slots.active += 1;
        Ok(StreamLease {
            owner: self.clone(),
        })
    }

    pub(crate) fn connection_layer(&self) -> ConnectionLayer {
        ConnectionLayer {
            gate: Arc::clone(&self.inner.connection_gate),
        }
    }
}

pub(crate) struct StreamLease {
    pub(crate) owner: Admission,
}

impl Drop for StreamLease {
    fn drop(&mut self) {
        let mut slots = self
            .owner
            .inner
            .stream_slots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        slots.active = slots.active.saturating_sub(1);
    }
}

#[derive(Clone)]
pub(crate) struct ConnectionLayer {
    gate: Arc<ConnectionGate>,
}

impl<S> Layer<S> for ConnectionLayer {
    type Service = ConnectionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConnectionService {
            inner,
            gate: Arc::clone(&self.gate),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ConnectionService<S> {
    inner: S,
    gate: Arc<ConnectionGate>,
}

struct ConnectionGate {
    starts: Mutex<ConnectionStarts>,
    #[cfg(test)]
    wait_observer: Mutex<Option<tokio::sync::mpsc::UnboundedSender<Instant>>>,
}

enum ConnectionStarts {
    Empty,
    One(Instant),
    Two { oldest: Instant, newest: Instant },
}

impl ConnectionStarts {
    fn admit_or_wait(&mut self, now: Instant) -> Option<Instant> {
        match self {
            Self::Empty => {
                *self = Self::One(now);
                None
            }
            Self::One(oldest) if now.duration_since(*oldest) >= ADMISSION_WINDOW => {
                *oldest = now;
                None
            }
            Self::One(oldest) => {
                *self = Self::Two {
                    oldest: *oldest,
                    newest: now,
                };
                None
            }
            Self::Two { newest, .. } if now.duration_since(*newest) >= ADMISSION_WINDOW => {
                *self = Self::One(now);
                None
            }
            Self::Two { oldest, newest } if now.duration_since(*oldest) >= ADMISSION_WINDOW => {
                *oldest = *newest;
                *newest = now;
                None
            }
            Self::Two { oldest, .. } => Some(*oldest + ADMISSION_WINDOW),
        }
    }
}

impl ConnectionGate {
    async fn admit_start(&self) {
        loop {
            let wait_until = {
                let mut starts = self
                    .starts
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                starts.admit_or_wait(Instant::now())
            };
            if let Some(until) = wait_until {
                #[cfg(test)]
                if until > Instant::now() {
                    let observer = self
                        .wait_observer
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    if let Some(observer) = observer.as_ref() {
                        let _ = observer.send(until);
                    }
                }
                tokio::time::sleep_until(until).await;
            } else {
                return;
            }
        }
    }
}

impl<S, Request> Service<Request> for ConnectionService<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let connection = self.inner.call(request);
        let gate = Arc::clone(&self.gate);
        Box::pin(async move {
            gate.admit_start().await;
            connection.await
        })
    }
}

#[cfg(test)]
// The fake connector supplies only the transport boundary and its completion signal.
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{AccountID, Client, Environment};
    use std::convert::Infallible;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        sync::{mpsc, oneshot},
    };
    use url::Url;

    struct HeldConnector {
        id: u8,
        begun: mpsc::UnboundedSender<u8>,
        release: Option<oneshot::Receiver<()>>,
    }

    impl Service<()> for HeldConnector {
        type Response = ();
        type Error = Infallible;
        type Future = Pin<Box<dyn Future<Output = std::result::Result<(), Infallible>> + Send>>;

        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _request: ()) -> Self::Future {
            let begun = self.begun.clone();
            let release = self.release.take().expect("one call");
            let id = self.id;
            Box::pin(async move {
                begun.send(id).expect("observer");
                let _ = release.await;
                Ok(())
            })
        }
    }

    async fn loopback() -> (
        Url,
        mpsc::UnboundedReceiver<Instant>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let url = Url::parse(&format!(
            "http://{}/",
            listener.local_addr().expect("address")
        ))
        .expect("URL");
        let (accepted, observations) = mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            while let Ok((socket, _)) = listener.accept().await {
                accepted.send(Instant::now()).expect("accept receiver");
                tokio::spawn(serve_socket(socket));
            }
        });
        (url, observations, task)
    }

    async fn serve_socket(mut socket: TcpStream) {
        let mut pending = Vec::new();
        loop {
            let mut buffer = [0_u8; 4096];
            let read = socket.read(&mut buffer).await.expect("read");
            if read == 0 {
                return;
            }
            pending.extend_from_slice(&buffer[..read]);
            let Some(end) = pending.windows(4).position(|bytes| bytes == b"\r\n\r\n") else {
                continue;
            };
            let stream = pending[..end]
                .windows(b"/transactions/stream".len())
                .any(|part| part == b"/transactions/stream");
            pending.drain(..end + 4);
            if stream {
                socket
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nTransfer-Encoding: chunked\r\n\r\n",
                    )
                    .await
                    .expect("stream header");
                std::future::pending::<()>().await;
            }
            let body = br#"{"accounts":[]}"#;
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
                body.len()
            );
            socket.write_all(header.as_bytes()).await.expect("header");
            socket.write_all(body).await.expect("body");
        }
    }

    fn keep_clock_frozen() -> tokio::task::JoinHandle<()> {
        tokio::spawn(async {
            loop {
                tokio::task::yield_now().await;
            }
        })
    }

    #[tokio::test(start_paused = true)]
    async fn built_rest_and_stream_clients_share_two_connector_starts_per_second() {
        let clock_guard = keep_clock_frozen();
        let (url, mut accepts, server) = loopback().await;
        let owner = Admission::new();
        let (wait_sender, mut waits) = mpsc::unbounded_channel();
        *owner
            .inner
            .connection_gate
            .wait_observer
            .lock()
            .expect("wait observer") = Some(wait_sender);
        let first_client = Client::builder(Environment::Practice, "token-a")
            .admission(owner.clone())
            .endpoints(url.clone(), url.clone())
            .build()
            .expect("first client");
        let second_client = Client::builder(Environment::Practice, "token-b")
            .admission(owner)
            .endpoints(url.clone(), url)
            .build()
            .expect("second client");
        let (stop_watchdog, watchdog_stopped) = std::sync::mpsc::channel();
        let (expired, mut watchdog_expired) = oneshot::channel();
        let watchdog = std::thread::spawn(move || {
            if matches!(
                watchdog_stopped.recv_timeout(Duration::from_secs(5)),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout)
            ) {
                let _ = expired.send(());
            }
        });

        first_client.list_accounts().await.expect("first REST");
        let first = accepts.recv().await.expect("first accept");
        first_client
            .list_accounts()
            .await
            .expect("reused REST socket");
        assert!(accepts.try_recv().is_err());
        assert!(waits.try_recv().is_err());

        second_client
            .list_accounts()
            .await
            .expect("second REST response");
        let second = accepts.recv().await.expect("second accept");
        assert_eq!(second, first);
        assert!(waits.try_recv().is_err());

        let account = AccountID::new("101-001-1-001").expect("account");
        let mut stream_task =
            tokio::spawn(async move { first_client.stream_transactions(&account).await });
        let stream_deadline = tokio::select! {
            observed = waits.recv() => observed,
            _ = &mut stream_task => None,
            _ = &mut watchdog_expired => None,
        }
        .expect("stream reached the connector start budget");
        assert_eq!(
            stream_deadline.duration_since(first),
            Duration::from_secs(1)
        );
        tokio::time::advance(Duration::from_millis(999)).await;
        assert!(accepts.try_recv().is_err());
        tokio::time::advance(Duration::from_millis(1)).await;
        let stream = stream_task
            .await
            .expect("stream task")
            .expect("stream response");
        let third = accepts.recv().await.expect("stream accept");
        assert_eq!(third, stream_deadline);

        drop(stream);
        stop_watchdog.send(()).expect("stop watchdog");
        watchdog.join().expect("watchdog");
        clock_guard.abort();
        server.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn two_connectors_start_while_held_and_third_waits_for_a_rolling_second() {
        let owner = Admission::new();
        let clock_guard = keep_clock_frozen();
        let (begun, mut observed) = mpsc::unbounded_channel();
        let (release_first, first_done) = oneshot::channel();
        let (release_second, second_done) = oneshot::channel();
        let (release_third, third_done) = oneshot::channel();
        let mut first = owner.connection_layer().layer(HeldConnector {
            id: 1,
            begun: begun.clone(),
            release: Some(first_done),
        });
        let mut second = owner.connection_layer().layer(HeldConnector {
            id: 2,
            begun: begun.clone(),
            release: Some(second_done),
        });
        let mut third = owner.connection_layer().layer(HeldConnector {
            id: 3,
            begun,
            release: Some(third_done),
        });
        let started = Instant::now();
        let first_task = tokio::spawn(async move { first.call(()).await });
        assert_eq!(observed.recv().await, Some(1));
        let second_task = tokio::spawn(async move { second.call(()).await });
        assert_eq!(observed.recv().await, Some(2));
        assert_eq!(Instant::now(), started);
        let third_task = tokio::spawn(async move { third.call(()).await });
        tokio::time::advance(Duration::from_millis(999)).await;
        assert!(observed.try_recv().is_err());
        tokio::time::advance(Duration::from_millis(1)).await;
        assert_eq!(observed.recv().await, Some(3));
        assert_eq!(Instant::now(), started + Duration::from_secs(1));
        release_first.send(()).expect("release first");
        release_second.send(()).expect("release second");
        release_third.send(()).expect("release third");
        first_task
            .await
            .expect("first task")
            .expect("first connect");
        second_task
            .await
            .expect("second task")
            .expect("second connect");
        third_task
            .await
            .expect("third task")
            .expect("third connect");
        clock_guard.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn cancelled_started_connector_keeps_budget_but_queued_cancellation_spends_none() {
        let owner = Admission::new();
        let clock_guard = keep_clock_frozen();
        let (begun, mut observed) = mpsc::unbounded_channel();
        let (_release_first, first_done) = oneshot::channel();
        let (_release_second, second_done) = oneshot::channel();
        let (_release_third, third_done) = oneshot::channel();
        let (release_fourth, fourth_done) = oneshot::channel();
        let mut first = owner.connection_layer().layer(HeldConnector {
            id: 1,
            begun: begun.clone(),
            release: Some(first_done),
        });
        let mut second = owner.connection_layer().layer(HeldConnector {
            id: 2,
            begun: begun.clone(),
            release: Some(second_done),
        });
        let mut third = owner.connection_layer().layer(HeldConnector {
            id: 3,
            begun: begun.clone(),
            release: Some(third_done),
        });
        let mut fourth = owner.connection_layer().layer(HeldConnector {
            id: 4,
            begun,
            release: Some(fourth_done),
        });
        let first_task = tokio::spawn(async move { first.call(()).await });
        assert_eq!(observed.recv().await, Some(1));
        first_task.abort();
        let _ = first_task.await;
        let second_task = tokio::spawn(async move { second.call(()).await });
        assert_eq!(observed.recv().await, Some(2));
        let third_task = tokio::spawn(async move { third.call(()).await });
        tokio::time::advance(Duration::from_millis(999)).await;
        assert!(observed.try_recv().is_err());
        third_task.abort();
        let _ = third_task.await;
        let fourth_task = tokio::spawn(async move { fourth.call(()).await });
        assert!(observed.try_recv().is_err());
        tokio::time::advance(Duration::from_millis(1)).await;
        assert_eq!(observed.recv().await, Some(4));
        release_fourth.send(()).expect("release fourth");
        fourth_task
            .await
            .expect("fourth task")
            .expect("fourth connect");
        second_task.abort();
        let _ = second_task.await;
        clock_guard.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn shared_rate_and_cooldown_keep_exit_reserve_spent_across_clones() {
        let owner = Admission::with_exit_reserve(119).expect("reserve");
        owner.admit(true, false).await.expect("ordinary capacity");
        assert!(matches!(
            owner.clone().admit(true, false).await,
            Err(Error::RateLimited)
        ));
        owner
            .admit(true, true)
            .await
            .expect("reserved exit capacity");
        for _ in 0..118 {
            owner
                .admit(true, true)
                .await
                .expect("remaining exit capacity");
        }
        owner.set_exit_reserve(0).expect("release reserve");
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        owner.install_cooldown(Duration::from_secs(2));
        assert!(matches!(
            owner.clone().admit(true, true).await,
            Err(Error::RateLimited)
        ));
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_secs(2)).await;
        owner.admit(true, false).await.expect("cooldown expired");
    }

    #[tokio::test(start_paused = true)]
    async fn shared_owner_admits_one_hundred_twenty_rest_requests_and_refuses_the_next_mutation() {
        let owner = Admission::new();
        for _ in 0..120 {
            owner
                .clone()
                .admit(true, false)
                .await
                .expect("documented REST capacity");
        }
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn rolling_window_does_not_refill_a_late_burst_at_the_first_requests_expiry() {
        let owner = Admission::new();
        owner.admit(true, false).await.expect("first request");
        tokio::time::advance(Duration::from_millis(999)).await;
        for _ in 0..119 {
            owner.admit(true, false).await.expect("late burst");
        }
        tokio::time::advance(Duration::from_millis(1)).await;
        owner.admit(true, false).await.expect("one expired request");
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_millis(998)).await;
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        tokio::time::advance(Duration::from_millis(1)).await;
        owner.admit(true, false).await.expect("late burst expired");
    }

    #[tokio::test(start_paused = true)]
    async fn reserve_of_one_hundred_nineteen_leaves_one_ordinary_request_and_all_exits() {
        let owner = Admission::with_exit_reserve(119).expect("largest valid reserve");
        owner.admit(true, false).await.expect("ordinary request");
        assert!(matches!(
            owner.admit(true, false).await,
            Err(Error::RateLimited)
        ));
        for _ in 0..119 {
            owner.admit(true, true).await.expect("reserved exit");
        }
        assert!(matches!(
            owner.admit(true, true).await,
            Err(Error::RateLimited)
        ));
        assert!(matches!(
            Admission::with_exit_reserve(120),
            Err(Error::InvalidInput(_))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn twenty_stream_leases_start_together_and_the_next_is_unsent() {
        let owner = Admission::new();
        let mut leases = Vec::new();
        for _ in 0..20 {
            leases.push(owner.lease_stream().expect("active stream lease"));
        }
        assert!(matches!(owner.lease_stream(), Err(Error::RateLimited)));
        drop(leases.pop());
        leases.push(owner.lease_stream().expect("released lease"));
    }
}

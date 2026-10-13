//! Shared transport admission through the public client and loopback sockets.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use oanda_client::{
    AccountID, Admission, Client, Environment, Error, OperationError, OrderSpecifier,
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::Instant,
};
use url::Url;

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

async fn stall_first_loopback() -> (
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
        let mut first = true;
        while let Ok((socket, _)) = listener.accept().await {
            accepted.send(Instant::now()).expect("accept receiver");
            if first {
                tokio::spawn(async move {
                    let _socket = socket;
                    std::future::pending::<()>().await;
                });
                first = false;
            } else {
                tokio::spawn(serve_socket(socket));
            }
        }
    });
    (url, observations, task)
}

async fn rate_limit_loopback() -> (Url, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut request = Vec::new();
        while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            let mut buffer = [0_u8; 4096];
            let read = socket.read(&mut buffer).await.expect("read");
            assert!(read > 0, "request closed");
            request.extend_from_slice(&buffer[..read]);
        }
        socket
            .write_all(b"HTTP/1.1 429 Too Many Requests\r\nRetry-After: 2\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
            .expect("write rejection");
    });
    (url, task)
}

async fn serve_socket(mut socket: TcpStream) {
    let mut pending = Vec::new();
    loop {
        let mut buf = [0_u8; 4096];
        let read = socket.read(&mut buf).await.expect("read");
        if read == 0 {
            return;
        }
        pending.extend_from_slice(&buf[..read]);
        let Some(end) = pending.windows(4).position(|bytes| bytes == b"\r\n\r\n") else {
            continue;
        };
        let header = String::from_utf8_lossy(&pending[..end]);
        let stream = header.contains("/transactions/stream");
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
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
            body.len()
        );
        socket.write_all(response.as_bytes()).await.expect("header");
        socket.write_all(body).await.expect("body");
    }
}

fn client(url: &Url, token: &str, owner: Admission) -> Client {
    Client::builder(Environment::Practice, token)
        .admission(owner)
        .endpoints(url.clone(), url.clone())
        .build()
        .expect("client")
}

fn account() -> AccountID {
    AccountID::new("101-001-1-001").expect("account")
}

async fn advance_gap() {
    tokio::time::pause();
    tokio::time::advance(Duration::from_millis(500)).await;
    tokio::time::resume();
}

fn keep_clock_frozen() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        loop {
            tokio::task::yield_now().await;
        }
    })
}

#[tokio::test]
async fn different_tokens_share_twenty_stream_leases_and_the_next_is_unsent() {
    let (url, mut accepts, server) = loopback().await;
    let owner = Admission::new();
    let clients = [
        client(&url, "token-a", owner.clone()),
        client(&url, "token-b", owner),
    ];
    let mut streams = Vec::new();
    for index in 0..20 {
        if index > 0 {
            advance_gap().await;
        }
        streams.push(
            clients[index % 2]
                .stream_transactions(&account())
                .await
                .expect("stream"),
        );
        accepts.recv().await.expect("accepted stream");
    }
    advance_gap().await;
    assert!(matches!(
        clients[1].stream_transactions(&account()).await,
        Err(Error::RateLimited)
    ));
    assert!(accepts.try_recv().is_err());
    drop(streams.remove(0));
    advance_gap().await;
    let replacement = clients[1]
        .stream_transactions(&account())
        .await
        .expect("released slot");
    accepts.recv().await.expect("replacement accepted");
    assert!(accepts.try_recv().is_err());
    drop(replacement);
    drop(streams);
    server.abort();
}

#[tokio::test(start_paused = true)]
async fn independent_egress_owners_do_not_share_connector_cooldown() {
    let clock_guard = keep_clock_frozen();
    let (url, mut accepts, server) = loopback().await;
    let first = client(&url, "token-a", Admission::new());
    let second = client(&url, "token-b", Admission::new());
    let start = Instant::now();
    let (left, right) = tokio::join!(first.list_accounts(), second.list_accounts());
    left.expect("first account list");
    right.expect("second account list");
    let left_at = accepts.recv().await.expect("first accept");
    let right_at = accepts.recv().await.expect("second accept");
    assert_eq!(left_at, start);
    assert_eq!(right_at, start);
    assert_eq!(Instant::now(), start);
    clock_guard.abort();
    server.abort();
}

#[tokio::test]
async fn cancelled_stream_open_releases_shared_slot_for_another_credential() {
    let (url, mut accepts, server) = stall_first_loopback().await;
    let owner = Admission::new();
    let first = client(&url, "token-a", owner.clone());
    let second = client(&url, "token-b", owner);
    let account_id = account();
    let opening = tokio::spawn(async move { first.stream_transactions(&account_id).await });
    accepts.recv().await.expect("first accepted");
    opening.abort();
    let _ = opening.await;
    let mut held = Vec::new();
    for _ in 0..19 {
        advance_gap().await;
        held.push(
            second
                .stream_transactions(&account())
                .await
                .expect("another credential stream"),
        );
        accepts.recv().await.expect("held stream accepted");
    }
    advance_gap().await;
    let replacement = second
        .stream_transactions(&account())
        .await
        .expect("replacement stream");
    accepts.recv().await.expect("replacement accepted");
    drop(replacement);
    drop(held);
    server.abort();
}

#[tokio::test]
async fn provider_cooldown_from_one_token_refuses_another_tokens_mutation_without_a_send() {
    let (url, server) = rate_limit_loopback().await;
    let owner = Admission::new();
    let first = client(&url, "token-a", owner.clone());
    let second = client(&url, "token-b", owner);
    let order = OrderSpecifier::new("fixture-order").expect("order");
    let first_result = first.cancel_order(&account(), &order, None).await;
    assert!(first_result.is_err());
    server.await.expect("one provider rejection");
    assert!(matches!(
        second.cancel_order(&account(), &order, None).await,
        Err(OperationError::Client(Error::RateLimited))
    ));
}

#[test]
fn explicit_builder_reserve_must_agree_with_shared_owner_without_resetting_it() {
    let owner = Admission::with_exit_reserve(7).expect("reserve");
    let preserved = Client::builder(Environment::Practice, "token-a")
        .admission(owner.clone())
        .build();
    assert!(preserved.is_ok());
    let matching = Client::builder(Environment::Practice, "token-b")
        .admission(owner.clone())
        .exit_reserve(7)
        .build();
    assert!(matching.is_ok());
    let conflict = Client::builder(Environment::Practice, "token-c")
        .admission(owner.clone())
        .exit_reserve(8)
        .build();
    assert!(matches!(conflict, Err(Error::InvalidInput(_))));
    assert!(
        Client::builder(Environment::Practice, "token-d")
            .admission(owner)
            .exit_reserve(7)
            .build()
            .is_ok()
    );
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Loopback helpers shared by the integration tests.
//!
//! Each test file compiles this module into its own crate and uses a subset of it.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]
#![allow(
    dead_code,
    reason = "each integration test compiles and uses its own subset of this module"
)]

use oanda_client::{Admission, Client, Environment};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::Instant,
};
use url::Url;

/// Serve one canned response on loopback and return the request it received.
///
/// An empty `response` closes the connection without writing anything.
pub async fn server(response: Vec<u8>) -> (Url, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let mut pending = Vec::new();
        let bytes = read_request(&mut socket, &mut pending)
            .await
            .expect("request");
        if !response.is_empty() {
            socket.write_all(&response).await.expect("write");
            socket.flush().await.expect("flush");
        }
        String::from_utf8_lossy(&bytes).into_owned()
    });
    (url, task)
}

/// A `Connection: close` HTTP/1.1 response carrying `body`.
pub fn http(status: &str, content_type: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

/// A Practice client with a fixture token, pointed at one loopback server.
pub fn fixture_client(url: Url) -> Client {
    Client::builder(Environment::Practice, "fixture-secret")
        .endpoints(url.clone(), url)
        .timeout(Duration::from_secs(2))
        .build()
        .expect("fixture client")
}

/// A real streaming response whose body stays open until its client closes it.
pub struct HeldStream {
    /// Its loopback endpoint.
    pub url: Url,
    /// The server's peer-close observation, retaining its owned task.
    pub closed: tokio::task::JoinHandle<()>,
}

/// Writes exactly one body chunk, then observes closure without sending another byte.
pub async fn held_stream(prefix: &[u8]) -> HeldStream {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("stream bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let prefix = prefix.to_vec();
    let closed = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("stream accept");
        read_request(&mut socket, &mut Vec::new())
            .await
            .expect("stream request");
        let mut chunk = [0_u8; 4096];
        socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nTransfer-Encoding: chunked\r\n\r\n").await.expect("stream headers");
        if !prefix.is_empty() {
            socket
                .write_all(format!("{:x}\r\n", prefix.len()).as_bytes())
                .await
                .expect("chunk header");
            socket.write_all(&prefix).await.expect("body prefix");
            socket.write_all(b"\r\n").await.expect("chunk end");
        }
        socket.flush().await.expect("prefix flush");
        let read = socket.read(&mut chunk).await;
        assert!(
            matches!(read, Ok(0)) || read.is_err(),
            "client sent another request on a retired streaming connection"
        );
    });
    HeldStream { url, closed }
}

/// Read one complete HTTP fixture request, retaining pipelined bytes for the next call.
/// A clean close between requests is optional; closing a partial request is a test failure.
pub async fn read_request(socket: &mut TcpStream, pending: &mut Vec<u8>) -> Option<Vec<u8>> {
    loop {
        if let Some(end) = pending.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
            let header_end = end + 4;
            let head = std::str::from_utf8(&pending[..header_end]).expect("HTTP fixture headers");
            let length = head
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().expect("content length"))
                })
                .unwrap_or(0);
            let total = header_end + length;
            if pending.len() >= total {
                return Some(pending.drain(..total).collect());
            }
        }
        let mut buffer = [0_u8; 4096];
        let read = socket.read(&mut buffer).await.expect("request read");
        if read == 0 {
            assert!(
                pending.is_empty(),
                "request closed before its full body arrived"
            );
            return None;
        }
        pending.extend_from_slice(&buffer[..read]);
    }
}

/// Serve one canned response per connection, returning every complete request.
pub async fn servers(responses: Vec<Vec<u8>>) -> (Url, tokio::task::JoinHandle<Vec<String>>) {
    response_server(responses, false).await
}

/// Serve all canned responses on one persistent connection.
pub async fn keepalive_server(
    responses: Vec<Vec<u8>>,
) -> (Url, tokio::task::JoinHandle<Vec<String>>) {
    response_server(responses, true).await
}

async fn response_server(
    responses: Vec<Vec<u8>>,
    keepalive: bool,
) -> (Url, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = loopback_url(&listener);
    let task = tokio::spawn(async move {
        let mut connection = None;
        let mut pending = Vec::new();
        let mut requests = Vec::new();
        for response in responses {
            if connection.is_none() {
                connection = Some(listener.accept().await.expect("accept").0);
            }
            let socket = connection.as_mut().expect("accepted connection");
            let request = read_request(socket, &mut pending)
                .await
                .expect("request before response");
            socket.write_all(&response).await.expect("response write");
            socket.flush().await.expect("response flush");
            requests.push(String::from_utf8(request).expect("fixture request UTF-8"));
            if !keepalive {
                connection = None;
            }
        }
        requests
    });
    (url, task)
}

fn loopback_url(listener: &TcpListener) -> Url {
    Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("loopback URL")
}

pub async fn loopback() -> (
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

pub async fn counting_rest_loopback() -> (
    Url,
    mpsc::UnboundedReceiver<String>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let (requests, observed) = mpsc::unbounded_channel();
    let task = tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let requests = requests.clone();
            tokio::spawn(async move {
                let mut pending = Vec::new();
                while let Some(request) = read_request(&mut socket, &mut pending).await {
                    let head = String::from_utf8(request).expect("fixture request");
                    let path = head.lines().next().expect("request line").to_owned();
                    requests.send(path).expect("request observer");
                    let body = br#"{"accounts":[]}"#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
                        body.len()
                    );
                    socket.write_all(response.as_bytes()).await.expect("header");
                    socket.write_all(body).await.expect("body");
                }
            });
        }
    });
    (url, observed, task)
}

pub async fn stall_first_loopback() -> (
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

pub async fn rate_limit_loopback() -> (Url, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        read_request(&mut socket, &mut Vec::new())
            .await
            .expect("rate-limit request");
        socket
            .write_all(b"HTTP/1.1 429 Too Many Requests\r\nRetry-After: 2\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
            .expect("write rejection");
    });
    (url, task)
}

pub async fn serve_socket(mut socket: TcpStream) {
    let mut pending = Vec::new();
    while let Some(request) = read_request(&mut socket, &mut pending).await {
        let header = String::from_utf8(request).expect("fixture request");
        let stream = header.contains("/transactions/stream");
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

/// Keep paused Tokio time stationary while loopback I/O completes.
pub fn keep_clock_frozen() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        loop {
            tokio::task::yield_now().await;
        }
    })
}

/// A fixture client with caller-selected credentials and shared admission.
pub fn fixture_client_with(url: &Url, token: &str, owner: Admission) -> Client {
    Client::builder(Environment::Practice, token)
        .admission(owner)
        .endpoints(url.clone(), url.clone())
        .build()
        .expect("fixture admission client")
}

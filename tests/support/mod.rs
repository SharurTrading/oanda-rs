// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Loopback helpers shared by the integration tests.
//!
//! Each test file compiles this module into its own crate and uses a subset of it.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic, dead_code)]

use oanda_client::{Client, Environment};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
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
        let mut bytes = Vec::new();
        let mut buf = [0_u8; 4096];
        loop {
            let read = socket.read(&mut buf).await.expect("read");
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&buf[..read]);
            if let Some(header_end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                let header_end = header_end + 4;
                let header = String::from_utf8_lossy(&bytes[..header_end]);
                let length = header
                    .lines()
                    .find_map(|l| {
                        l.strip_prefix("content-length: ")
                            .or_else(|| l.strip_prefix("Content-Length: "))
                    })
                    .and_then(|v| v.trim().parse::<usize>().ok())
                    .unwrap_or(0);
                if bytes.len() >= header_end + length {
                    break;
                }
            }
        }
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

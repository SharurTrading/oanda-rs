//! Loopback helpers for held-open streaming responses.
use super::server::read_request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use url::Url;

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

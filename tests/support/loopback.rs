//! Behavior-observing loopback servers for admission and stream tests.
use super::server::read_request;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::Instant,
};
use url::Url;

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

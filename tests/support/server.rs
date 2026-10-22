//! Canned-response loopback HTTP servers.
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
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

pub(super) fn loopback_url(listener: &TcpListener) -> Url {
    Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("loopback URL")
}

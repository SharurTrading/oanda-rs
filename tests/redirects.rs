// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Redirect outcomes and response diagnostics observed through loopback servers.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]

use oanda_client::{
    AccountID, Error, OperationError, RequestID, ResponseDiagnostics, Supplied,
    account::ConfigureAccountBody,
};
use std::net::TcpListener;

mod support;
use support::{fixture_client, http, server, servers};

const TOKEN: &str = "fixture-secret";

fn account() -> AccountID {
    AccountID::new("101-001-1-001").expect("account")
}

fn rename() -> ConfigureAccountBody {
    ConfigureAccountBody {
        alias: Some("renamed".into()),
        margin_rate: None,
    }
}

/// A `Connection: close` answer with extra headers and an empty body.
fn answer(status: &str, headers: &[&str]) -> Vec<u8> {
    let mut response = format!("HTTP/1.1 {status}\r\n");
    for header in headers {
        response.push_str(header);
        response.push_str("\r\n");
    }
    response.push_str("Content-Length: 0\r\nConnection: close\r\n\r\n");
    response.into_bytes()
}

/// A listener whose pending connections can be inspected without waiting.
fn redirect_target() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("target bind");
    listener.set_nonblocking(true).expect("nonblocking target");
    let address = listener.local_addr().expect("target address").to_string();
    (listener, address)
}

/// Every connection attempt completes before the client returns, so an empty
/// accept queue after the call proves the target was never contacted.
fn assert_never_contacted(listener: &TcpListener) {
    match listener.accept() {
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
        Ok((_, peer)) => panic!("the client connected to the redirect target from {peer}"),
        Err(error) => panic!("target accept failed: {error}"),
    }
}

fn decoded(value: &str) -> Supplied<String> {
    Supplied::Decoded(value.to_owned())
}

fn assert_target(location: &Supplied<oanda_client::RedirectTarget>, expected: &str) {
    match location {
        Supplied::Decoded(target) => assert_eq!(target.to_string(), expected),
        other => panic!("expected a decoded target {expected}: {other:?}"),
    }
}

#[tokio::test]
async fn a_redirected_read_is_typed_with_its_reduced_target_and_request_id() {
    let (target, address) = redirect_target();
    let location = format!("Location: http://user:pass@{address}/v3/elsewhere?token=abc#frag");
    let (url, task) = server(answer(
        "307 Temporary Redirect",
        &[&location, "RequestID: 42", "Server: cloudflare"],
    ))
    .await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("redirect");
    assert_never_contacted(&target);
    let OperationError::Client(Error::Redirected {
        status,
        ref location,
        ref request_id,
        ref diagnostics,
    }) = error
    else {
        panic!("expected a typed redirect: {error:?}")
    };
    assert_eq!(status, 307);
    assert_target(location, &format!("http://{address}/v3/elsewhere"));
    assert_eq!(
        *request_id,
        Supplied::Decoded(RequestID::new("42").expect("id"))
    );
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    let rendered = format!("{error} {error:?}");
    for hidden in ["user", "pass", "token=abc", "frag", TOKEN] {
        assert!(
            !rendered.contains(hidden),
            "{hidden} leaked into {rendered}"
        );
    }
    let request = task.await.expect("server");
    assert!(request.contains(&format!("Bearer {TOKEN}")));
}

#[tokio::test]
async fn a_redirected_mutation_is_not_executed_and_does_not_fence_the_account() {
    let (target, address) = redirect_target();
    let location = format!("Location: http://{address}/v3/accounts");
    let rejection = http(
        "400 Bad Request",
        "application/json",
        r#"{"errorMessage":"invalid alias"}"#,
    );
    let (url, task) = servers(vec![
        answer("307 Temporary Redirect", &[&location, "RequestID: 7"]),
        rejection,
    ])
    .await;
    let client = fixture_client(url);
    let first = client
        .configure_account(&account(), &rename())
        .await
        .expect_err("redirect");
    assert_never_contacted(&target);
    let OperationError::Client(Error::Redirected {
        status: 307,
        ref location,
        ref request_id,
        ..
    }) = first
    else {
        panic!("expected a typed redirect, not an ambiguous mutation: {first:?}")
    };
    assert_target(location, &format!("http://{address}/v3/accounts"));
    assert_eq!(
        *request_id,
        Supplied::Decoded(RequestID::new("7").expect("id"))
    );
    let second = client
        .configure_account(&account(), &rename())
        .await
        .expect_err("rejection");
    assert!(
        matches!(second, OperationError::Rejected { status: 400, .. }),
        "the account must not be fenced after a redirect: {second:?}"
    );
    assert_eq!(task.await.expect("server").len(), 2);
}

#[tokio::test]
async fn a_redirected_stream_open_is_typed_and_never_followed() {
    let (target, address) = redirect_target();
    let location = format!("Location: http://{address}/v3/accounts/1/transactions/stream?x=1");
    let (url, task) = server(answer(
        "302 Found",
        &[
            &location,
            "RequestID: 9",
            "CF-RAY: 8f00000000000000-SYD",
            "cf-mitigated: challenge",
        ],
    ))
    .await;
    let client = fixture_client(url);
    let Err(error) = client.stream_transactions(&account()).await else {
        panic!("expected a redirect")
    };
    assert_never_contacted(&target);
    let Error::Redirected {
        status,
        ref location,
        ref request_id,
        ref diagnostics,
    } = error
    else {
        panic!("expected a typed redirect: {error:?}")
    };
    assert_eq!(status, 302);
    assert_target(
        location,
        &format!("http://{address}/v3/accounts/1/transactions/stream"),
    );
    assert_eq!(
        *request_id,
        Supplied::Decoded(RequestID::new("9").expect("id"))
    );
    assert_eq!(diagnostics.ray_id, decoded("8f00000000000000-SYD"));
    assert_eq!(diagnostics.mitigated, decoded("challenge"));
    assert_eq!(diagnostics.server, Supplied::Absent);
    assert!(!format!("{error} {error:?}").contains("x=1"));
    task.await.expect("server");
}

#[tokio::test]
async fn a_redirect_without_a_location_reports_its_absence() {
    let (url, task) = server(answer("301 Moved Permanently", &[])).await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("redirect");
    let OperationError::Client(Error::Redirected {
        status: 301,
        location: Supplied::Absent,
        request_id: Supplied::Absent,
        ref diagnostics,
    }) = error
    else {
        panic!("expected a redirect without a target: {error:?}")
    };
    assert_eq!(**diagnostics, ResponseDiagnostics::default());
    assert!(error.to_string().contains("without a Location"));
    task.await.expect("server");
}

#[tokio::test]
async fn a_read_rejection_carries_the_edge_diagnostics() {
    let (url, task) = server(answer(
        "520 Unknown Error",
        &[
            "Server: cloudflare",
            "CF-RAY: 8f00000000000001-SYD",
            "Set-Cookie: __cf_bm=cookie-value",
        ],
    ))
    .await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("rejection");
    let OperationError::Rejected {
        status: 520,
        ref diagnostics,
        ..
    } = error
    else {
        panic!("expected a rejection: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.ray_id, decoded("8f00000000000001-SYD"));
    assert_eq!(diagnostics.mitigated, Supplied::Absent);
    assert!(!format!("{error} {error:?}").contains("cookie-value"));
    task.await.expect("server");
}

#[tokio::test]
async fn a_rejection_without_edge_headers_has_absent_diagnostics() {
    let (url, task) = server(http(
        "404 Not Found",
        "application/json",
        r#"{"errorMessage":"no account"}"#,
    ))
    .await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("rejection");
    let OperationError::Rejected {
        status: 404,
        ref diagnostics,
        ..
    } = error
    else {
        panic!("expected a rejection: {error:?}")
    };
    assert_eq!(**diagnostics, ResponseDiagnostics::default());
    task.await.expect("server");
}

#[tokio::test]
async fn an_unreadable_rejection_body_keeps_the_edge_diagnostics() {
    let (url, task) = server(
        b"HTTP/1.1 403 Forbidden\r\nServer: cloudflare\r\ncf-mitigated: challenge\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort"
            .to_vec(),
    )
    .await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("rejection");
    let OperationError::RejectedUnreadable {
        status: 403,
        ref diagnostics,
        ..
    } = error
    else {
        panic!("expected an unreadable rejection: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.mitigated, decoded("challenge"));
    assert_eq!(diagnostics.ray_id, Supplied::Absent);
    task.await.expect("server");
}

#[tokio::test]
async fn a_refused_stream_open_carries_the_edge_diagnostics() {
    let (url, task) = server(answer(
        "403 Forbidden",
        &["Server: cloudflare", "cf-mitigated: challenge"],
    ))
    .await;
    let client = fixture_client(url);
    let Err(error) = client.stream_transactions(&account()).await else {
        panic!("expected a refusal")
    };
    let Error::Provider {
        status: 403,
        ref diagnostics,
        ..
    } = error
    else {
        panic!("expected a provider refusal: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.mitigated, decoded("challenge"));
    assert_eq!(diagnostics.ray_id, Supplied::Absent);
    task.await.expect("server");
}

#[tokio::test]
async fn a_refused_stream_open_without_edge_headers_has_absent_diagnostics() {
    let (url, task) = server(answer("401 Unauthorized", &[])).await;
    let client = fixture_client(url);
    let Err(error) = client.stream_transactions(&account()).await else {
        panic!("expected a refusal")
    };
    let Error::Provider {
        status: 401,
        ref diagnostics,
        ..
    } = error
    else {
        panic!("expected a provider refusal: {error:?}")
    };
    assert_eq!(**diagnostics, ResponseDiagnostics::default());
    task.await.expect("server");
}

#[tokio::test]
async fn a_mutation_answered_by_the_edge_stays_ambiguous_and_carries_its_diagnostics() {
    let (url, task) = server(answer(
        "520 Unknown Error",
        &["Server: cloudflare", "CF-RAY: 8f00000000000002-SYD"],
    ))
    .await;
    let client = fixture_client(url);
    let first = client
        .configure_account(&account(), &rename())
        .await
        .expect_err("ambiguous");
    let OperationError::Client(Error::AmbiguousMutation {
        ref account,
        status,
        ref diagnostics,
    }) = first
    else {
        panic!("expected an ambiguous mutation: {first:?}")
    };
    assert_eq!(account.as_str(), "101-001-1-001");
    assert_eq!(status, Some(520));
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.ray_id, decoded("8f00000000000002-SYD"));
    assert_eq!(diagnostics.mitigated, Supplied::Absent);
    let second = client
        .configure_account(&self::account(), &rename())
        .await
        .expect_err("fenced");
    assert!(
        matches!(
            second,
            OperationError::Client(Error::ReconciliationRequired { .. })
        ),
        "an ambiguous mutation still fences the account: {second:?}"
    );
    task.await.expect("server");
}

#[tokio::test]
async fn an_undecodable_mutation_success_keeps_its_status_and_diagnostics() {
    let (url, task) = server(
        b"HTTP/1.1 200 OK\r\nServer: cloudflare\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot json!"
            .to_vec(),
    )
    .await;
    let client = fixture_client(url);
    let error = client
        .configure_account(&account(), &rename())
        .await
        .expect_err("ambiguous");
    let OperationError::Client(Error::AmbiguousMutation {
        status: Some(200),
        ref diagnostics,
        ..
    }) = error
    else {
        panic!("expected an ambiguous mutation with its status: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    task.await.expect("server");
}

#[tokio::test]
async fn a_mutation_without_a_response_is_ambiguous_without_a_status() {
    let (url, task) = server(Vec::new()).await;
    let client = fixture_client(url);
    let error = client
        .configure_account(&account(), &rename())
        .await
        .expect_err("ambiguous");
    let OperationError::Client(Error::AmbiguousMutation {
        status: None,
        ref diagnostics,
        ..
    }) = error
    else {
        panic!("expected an ambiguous mutation without a status: {error:?}")
    };
    assert_eq!(**diagnostics, ResponseDiagnostics::default());
    task.await.expect("server");
}

#[tokio::test]
async fn a_rate_limited_answer_carries_the_edge_diagnostics() {
    let (url, task) = server(answer(
        "429 Too Many Requests",
        &[
            "Server: cloudflare",
            "CF-RAY: 8f00000000000003-SYD",
            "Retry-After: 1",
        ],
    ))
    .await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("rate limited");
    let OperationError::Client(Error::ProviderRateLimited { ref diagnostics }) = error else {
        panic!("expected a provider rate limit: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.ray_id, decoded("8f00000000000003-SYD"));
    assert_eq!(diagnostics.mitigated, Supplied::Absent);
    task.await.expect("server");
}

#[tokio::test]
async fn a_rate_limited_answer_without_edge_headers_has_absent_diagnostics() {
    let (url, task) = server(answer("429 Too Many Requests", &[])).await;
    let client = fixture_client(url);
    let error = client
        .account_summary(&account())
        .await
        .expect_err("rate limited");
    let OperationError::Client(Error::ProviderRateLimited { ref diagnostics }) = error else {
        panic!("expected a provider rate limit: {error:?}")
    };
    assert_eq!(**diagnostics, ResponseDiagnostics::default());
    task.await.expect("server");
}

#[tokio::test]
async fn a_refused_stream_open_with_a_truncated_body_keeps_its_status_and_diagnostics() {
    let (url, task) = server(
        b"HTTP/1.1 503 Service Unavailable\r\nServer: cloudflare\r\nCF-RAY: 8f00000000000004-SYD\r\nRequestID: 11\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort"
            .to_vec(),
    )
    .await;
    let client = fixture_client(url);
    let Err(error) = client.stream_transactions(&account()).await else {
        panic!("expected a refusal")
    };
    let Error::ProviderUnreadable {
        status,
        ref request_id,
        ref diagnostics,
        ref source,
    } = error
    else {
        panic!("expected an unreadable provider refusal: {error:?}")
    };
    assert_eq!(status, 503);
    assert_eq!(
        *request_id,
        Supplied::Decoded(RequestID::new("11").expect("id"))
    );
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.ray_id, decoded("8f00000000000004-SYD"));
    assert_eq!(diagnostics.mitigated, Supplied::Absent);
    assert!(matches!(**source, Error::Transport(_)), "{source:?}");
    task.await.expect("server");
}

#[tokio::test]
async fn a_refused_stream_open_with_an_oversized_body_keeps_its_status_and_diagnostics() {
    let body = "x".repeat(64);
    let (url, task) = server(
        format!(
            "HTTP/1.1 403 Forbidden\r\nServer: cloudflare\r\ncf-mitigated: challenge\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .into_bytes(),
    )
    .await;
    let client = oanda_client::Client::builder(oanda_client::Environment::Practice, TOKEN)
        .endpoints(url.clone(), url)
        .max_response_bytes(16)
        .build()
        .expect("client");
    let Err(error) = client.stream_transactions(&account()).await else {
        panic!("expected a refusal")
    };
    let Error::ProviderUnreadable {
        status: 403,
        request_id: Supplied::Absent,
        ref diagnostics,
        ref source,
    } = error
    else {
        panic!("expected an unreadable provider refusal: {error:?}")
    };
    assert_eq!(diagnostics.server, decoded("cloudflare"));
    assert_eq!(diagnostics.mitigated, decoded("challenge"));
    assert_eq!(diagnostics.ray_id, Supplied::Absent);
    assert!(matches!(**source, Error::ResponseTooLarge), "{source:?}");
    task.await.expect("server");
}

//! Deterministic public-client and loopback transport tests.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use oanda_client::pricing::{PricingQuery, StreamPricingQuery};
use oanda_client::trade::SetTradeDependentOrdersBody;
use oanda_client::{
    AccountID, Client, Environment, Error, InstrumentName, OperationError, Patch, PriceStreamEvent,
    Supplied,
    models::{ClientPrice, Order, Transaction},
};
use rust_decimal::Decimal;
use std::{str::FromStr, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use url::Url;

async fn server(response: Vec<u8>) -> (Url, tokio::task::JoinHandle<String>) {
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

/// Serve one canned response per connection, returning every request received.
async fn servers(responses: Vec<Vec<u8>>) -> (Url, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let task = tokio::spawn(async move {
        let mut requests = Vec::new();
        for response in responses {
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
            socket.write_all(&response).await.expect("write");
            socket.flush().await.expect("flush");
            requests.push(String::from_utf8_lossy(&bytes).into_owned());
        }
        requests
    });
    (url, task)
}

fn http_json(status: &str, value: &str) -> Vec<u8> {
    format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{value}", value.len()).into_bytes()
}

fn fixture_client(url: Url) -> Client {
    Client::builder(Environment::Practice, "fixture-secret")
        .endpoints(url.clone(), url)
        .timeout(Duration::from_secs(2))
        .build()
        .expect("fixture client")
}

#[tokio::test]
async fn account_query_uses_provider_path_and_redacts_token() {
    let (url, task) = server(http_json("200 OK", r#"{"accounts":[]}"#)).await;
    let client = fixture_client(url);
    assert!(!format!("{client:?}").contains("fixture-secret"));
    let response = client.list_accounts().await.expect("accounts");
    assert_eq!(response.body.accounts.expect("accounts field").len(), 0);
    let request = task.await.expect("server");
    assert!(request.starts_with("GET /v3/accounts HTTP/1.1"));
    assert!(
        request.contains("Authorization: Bearer fixture-secret")
            || request.contains("authorization: Bearer fixture-secret")
    );
}

#[tokio::test]
async fn pricing_query_uses_csv_and_exact_decimal() {
    let (url, task) = server(http_json("200 OK", r#"{"prices":[{"type":"PRICE","instrument":"EUR_USD","closeoutBid":"1.000000000000000001","bids":[{"price":"1.0","liquidity":500000}],"asks":[{"price":"1.1","liquidity":0.1234567890123456789012345678}]}] }"#)).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let instrument = InstrumentName::new("EUR_USD").expect("instrument");
    let query = PricingQuery {
        instruments: vec![instrument],
        since: None,
        include_units_available: None,
        include_home_conversions: None,
    };
    let response = client.pricing(&account, &query).await.expect("pricing");
    assert_eq!(
        response.prices[0].closeout_bid,
        Some(Decimal::from_str("1.000000000000000001").expect("decimal"))
    );
    assert_eq!(
        response.prices[0].bids.as_ref().expect("bids")[0].liquidity,
        Some(Decimal::from(500_000))
    );
    assert_eq!(
        response.prices[0].asks.as_ref().expect("asks")[0].liquidity,
        Some(Decimal::from_str("0.1234567890123456789012345678").expect("decimal"))
    );
    let request = task.await.expect("server");
    assert!(request.contains("/pricing?instruments=EUR_USD"));
}

#[tokio::test]
async fn ambiguous_mutation_fences_only_its_account() {
    let (url, task) = server(Vec::new()).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let body = oanda_client::account::ConfigureAccountBody {
        alias: Some("renamed".into()),
        margin_rate: None,
    };
    let first = client
        .configure_account(&account, &body)
        .await
        .expect_err("connection closed");
    assert!(matches!(
        first,
        OperationError::Client(Error::AmbiguousMutation { .. })
    ));
    let second = client
        .configure_account(&account, &body)
        .await
        .expect_err("fenced");
    assert!(matches!(
        second,
        OperationError::Client(Error::ReconciliationRequired { .. })
    ));
    client
        .acknowledge_reconciliation(&account)
        .expect("acknowledge");
    task.await.expect("server");
}

#[tokio::test]
async fn pricing_stream_handles_fragmented_lines_and_heartbeats() {
    let price = br#"{"type":"PRICE","instrument":"EUR_USD","closeoutBid":"1.23456","bids":[{"price":"1.23456","liquidity":500000}]}"#;
    let heartbeat = br#"{"type":"HEARTBEAT","time":"2016-09-20T15:05:50.163791738Z"}"#;
    let chunks = vec![
        &price[..18],
        &price[18..],
        b"\n",
        heartbeat.as_slice(),
        b"\n",
    ];
    let mut response = b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n".to_vec();
    for chunk in chunks {
        response.extend_from_slice(format!("{:X}\r\n", chunk.len()).as_bytes());
        response.extend_from_slice(chunk);
        response.extend_from_slice(b"\r\n");
    }
    response.extend_from_slice(b"0\r\n\r\n");
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let query = StreamPricingQuery {
        instruments: vec![InstrumentName::new("EUR_USD").expect("instrument")],
        snapshot: None,
        include_home_conversions: None,
    };
    let mut stream = client
        .stream_pricing(&account, &query)
        .await
        .expect("stream");
    assert!(matches!(
        stream.next_event().await,
        Some(Ok(PriceStreamEvent::Price(_)))
    ));
    assert!(matches!(
        stream.next_event().await,
        Some(Ok(PriceStreamEvent::Heartbeat(_)))
    ));
    assert!(matches!(
        stream.next_event().await,
        Some(Err(Error::Transport(_)))
    ));
    assert!(stream.next_event().await.is_none());
    task.await.expect("server");
}

#[test]
fn dependent_order_patch_distinguishes_omission_and_clear() {
    let body = SetTradeDependentOrdersBody {
        take_profit: Patch::Clear,
        stop_loss: Patch::Unchanged,
        trailing_stop_loss: Patch::Unchanged,
        guaranteed_stop_loss: Patch::Unchanged,
    };
    let value = serde_json::to_value(body).expect("serialize");
    assert_eq!(value, serde_json::json!({"takeProfit":null}));
}

#[test]
fn tagged_models_preserve_concrete_and_future_variants() {
    let order: Order = serde_json::from_value(serde_json::json!({"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"})).expect("market order");
    assert!(matches!(order, Order::MarketOrder(_)));
    let transaction: Transaction =
        serde_json::from_value(serde_json::json!({"type":"FUTURE_TRANSACTION","id":"44"}))
            .expect("future transaction");
    assert!(matches!(transaction, Transaction::Unknown { .. }));
    let price: ClientPrice =
        serde_json::from_value(serde_json::json!({"closeoutAsk":"1.000000000000000001"}))
            .expect("price");
    assert_eq!(
        price.closeout_ask,
        Some(Decimal::from_str("1.000000000000000001").expect("decimal"))
    );
}

#[tokio::test]
async fn typed_rejection_retains_provider_transaction() {
    let body = r#"{"clientConfigureRejectTransaction":{"type":"CLIENT_CONFIGURE_REJECT","id":"9"},"errorCode":"INVALID_VALUE","errorMessage":"margin rate rejected"}"#;
    let (url, task) = server(http_json("400 Bad Request", body)).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let request = oanda_client::account::ConfigureAccountBody {
        alias: None,
        margin_rate: Some(Decimal::from_str("0.02").expect("decimal")),
    };
    let error = client
        .configure_account(&account, &request)
        .await
        .expect_err("rejected");
    match error {
        OperationError::Rejected {
            status,
            code,
            message,
            body,
        } => {
            assert_eq!(status, 400);
            assert_eq!(code.as_deref(), Some("INVALID_VALUE"));
            assert_eq!(message.as_deref(), Some("margin rate rejected"));
            let body = body.into_decoded().expect("decoded rejection body");
            assert!(body.client_configure_reject_transaction.is_some());
        }
        other => panic!("unexpected error: {other:?}"),
    }
    task.await.expect("server");
}

#[tokio::test]
async fn transaction_stream_decodes_heartbeat_and_variant() {
    let body = concat!(
        "{\"type\":\"HEARTBEAT\",\"lastTransactionID\":\"1\",\"time\":\"2016-09-20T15:05:50Z\"}\n",
        "{\"type\":\"MARKET_ORDER\",\"id\":\"2\",\"instrument\":\"EUR_USD\",\"units\":\"1\",\"timeInForce\":\"FOK\",\"positionFill\":\"DEFAULT\"}\n",
    );
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).into_bytes();
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let mut stream = client.stream_transactions(&account).await.expect("stream");
    assert!(matches!(
        stream.next_event().await,
        Some(Ok(oanda_client::TransactionStreamEvent::Heartbeat(_)))
    ));
    assert!(matches!(
        stream.next_event().await,
        Some(Ok(oanda_client::TransactionStreamEvent::Transaction(_)))
    ));
    task.await.expect("server");
}

#[test]
fn timestamps_accept_rfc3339_and_fractional_unix_without_float() {
    let rfc: oanda_client::Timestamp = "2016-09-20T15:05:50.163791738Z".parse().expect("RFC3339");
    let unix: oanda_client::Timestamp = "1474383950.163791738".parse().expect("Unix");
    assert_eq!(rfc, unix);
}

#[tokio::test]
async fn pagination_link_follows_only_same_origin() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("url");
    let next = format!("{url}v3/accounts?page=2");
    let task = tokio::spawn(async move {
        let mut paths = Vec::new();
        for index in 0..2 {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut bytes = [0_u8; 4096];
            let read = socket.read(&mut bytes).await.expect("read");
            paths.push(
                String::from_utf8_lossy(&bytes[..read])
                    .lines()
                    .next()
                    .expect("line")
                    .to_owned(),
            );
            let body = r#"{"accounts":[]}"#;
            let link = if index == 0 {
                format!("Link: <{next}>; rel=\"next\"\r\n")
            } else {
                String::new()
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n{link}Connection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.expect("write");
        }
        paths
    });
    let client = fixture_client(url);
    let first = client.list_accounts().await.expect("first");
    let page = first.next_page.into_decoded().expect("next page");
    let second: oanda_client::ApiResponse<oanda_client::account::ListAccountsResponse> =
        client.fetch_page(&page).await.expect("second");
    assert!(second.next_page.is_absent());
    let paths = task.await.expect("server");
    assert_eq!(
        paths,
        [
            "GET /v3/accounts HTTP/1.1",
            "GET /v3/accounts?page=2 HTTP/1.1"
        ]
    );
    let foreign = Url::parse("https://example.com/v3/accounts").expect("url");
    let result: Result<oanda_client::ApiResponse<oanda_client::account::ListAccountsResponse>, _> =
        client.fetch_page(&foreign).await;
    assert!(matches!(
        result,
        Err(OperationError::Client(Error::InvalidInput(_)))
    ));
}

/// Serve one JSON success body with arbitrary extra raw header bytes, so a
/// header value this client cannot read as text is expressible in a fixture.
fn raw_headers_json(status: &str, headers: &[u8], value: &str) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n",
        value.len()
    )
    .into_bytes();
    response.extend_from_slice(headers);
    response.extend_from_slice(b"Connection: close\r\n\r\n");
    response.extend_from_slice(value.as_bytes());
    response
}

#[tokio::test]
async fn an_unreadable_request_id_is_reported_as_sent_not_absent() {
    // The header value carries bytes that are not valid ASCII/UTF-8, so OANDA
    // supplied an identifier this client cannot read. Before the Supplied
    // contract this decoded to None, the same value as no header at all.
    let response = raw_headers_json("200 OK", b"RequestID: \xFF\xFE\r\n", r#"{"accounts":[]}"#);
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let page = client.list_accounts().await.expect("request succeeds");
    assert_eq!(page.request_id, Supplied::Undecoded);
    assert!(page.next_page.is_absent());
    task.await.expect("server");
}

#[tokio::test]
async fn an_absent_request_id_is_absent() {
    let (url, task) = server(http_json("200 OK", r#"{"accounts":[]}"#)).await;
    let client = fixture_client(url);
    let page = client.list_accounts().await.expect("request succeeds");
    assert!(page.request_id.is_absent());
    task.await.expect("server");
}

#[tokio::test]
async fn an_unreadable_link_header_is_not_read_as_no_next_page() {
    // A Link header with unreadable bytes offered something this client cannot
    // surface. Before the Supplied contract the whole request failed on the
    // pagination hint even though the body decoded; it must never be reported
    // as "OANDA offered no next page".
    let response = raw_headers_json(
        "200 OK",
        b"Link: <\xFF/v3/accounts?page=2>; rel=\"next\"\r\n",
        r#"{"accounts":[]}"#,
    );
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let page = client.list_accounts().await.expect("body still decodes");
    assert_eq!(page.next_page, Supplied::Undecoded);
    task.await.expect("server");
}

#[tokio::test]
async fn a_malformed_next_link_is_undecoded_not_an_error() {
    // rel="next" with no closing bracket: a next page was offered and does not
    // match the documented shape, so the body succeeds and the offer is
    // reported as unreadable rather than failing the decoded body with it.
    let response = raw_headers_json(
        "200 OK",
        b"Link: </v3/accounts?page=2; rel=\"next\"\r\n",
        r#"{"accounts":[]}"#,
    );
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let page = client.list_accounts().await.expect("body still decodes");
    assert_eq!(page.next_page, Supplied::Undecoded);
    task.await.expect("server");
}

#[tokio::test]
async fn a_link_header_offering_no_next_page_is_absent() {
    // A readable Link that relates to something else is a decoded fact that no
    // next page was offered, and a readable RequestID decodes.
    let response = raw_headers_json(
        "200 OK",
        b"RequestID: 42\r\nLink: </v3/accounts>; rel=\"prev\"\r\n",
        r#"{"accounts":[]}"#,
    );
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let page = client.list_accounts().await.expect("request succeeds");
    assert_eq!(page.request_id, Supplied::Decoded("42".to_owned()));
    assert!(page.next_page.is_absent());
    task.await.expect("server");
}

#[tokio::test]
async fn invalid_query_and_mutation_are_rejected_before_network() {
    let client = fixture_client(Url::parse("http://127.0.0.1:1/").expect("url"));
    let account = AccountID::new("101-001-1-001").expect("account");
    let query = PricingQuery {
        instruments: vec![],
        since: None,
        include_units_available: None,
        include_home_conversions: None,
    };
    assert!(matches!(
        client.pricing(&account, &query).await,
        Err(OperationError::Client(Error::InvalidInput(_)))
    ));
    let body = oanda_client::account::ConfigureAccountBody {
        alias: None,
        margin_rate: Some(Decimal::from_str("2").expect("decimal")),
    };
    assert!(matches!(
        client.configure_account(&account, &body).await,
        Err(OperationError::Client(Error::InvalidInput(_)))
    ));
}

#[tokio::test]
async fn malformed_stream_record_ends_generation() {
    let body = b"not-json\n";
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let mut stream = client.stream_transactions(&account).await.expect("stream");
    assert!(matches!(
        stream.next_event().await,
        Some(Err(Error::Decode(_)))
    ));
    assert!(stream.next_event().await.is_none());
    task.await.expect("server");
}

#[tokio::test]
async fn oversized_stream_record_is_bounded_and_ends_generation() {
    let body = vec![b'X'; 1024 * 1024 + 1];
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(&body);
    let (url, task) = server(response).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let mut stream = client.stream_transactions(&account).await.expect("stream");
    assert!(matches!(
        stream.next_event().await,
        Some(Err(Error::ResponseTooLarge))
    ));
    assert!(stream.next_event().await.is_none());
    task.await.expect("server");
}

#[test]
fn remote_endpoint_override_cannot_redirect_credentials() {
    let rest = Url::parse("https://example.com/").expect("url");
    let stream = Url::parse("https://example.com/").expect("url");
    assert!(matches!(
        Client::builder(Environment::Practice, "fixture-secret")
            .endpoints(rest, stream)
            .build(),
        Err(Error::InvalidInput(_))
    ));
}

fn configure_body() -> oanda_client::account::ConfigureAccountBody {
    oanda_client::account::ConfigureAccountBody {
        alias: Some("renamed".into()),
        margin_rate: None,
    }
}

#[tokio::test]
async fn a_mistyped_reason_key_erases_only_itself() {
    // A struct-level read would fail on the first mistyped field and erase
    // both keys; each key is read on its own, so the readable sibling survives
    // exactly as it does on a refused stream.
    let (url, task) = server(http_json(
        "400 Bad Request",
        r#"{"errorCode":5,"errorMessage":"margin rate rejected"}"#,
    ))
    .await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let error = client
        .configure_account(&account, &configure_body())
        .await
        .expect_err("rejected");
    match error {
        OperationError::Rejected { code, message, .. } => {
            assert!(code.is_none(), "a numeric code is not a readable one");
            assert_eq!(message.as_deref(), Some("margin rate rejected"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    task.await.expect("server");
}

#[tokio::test]
async fn an_unreadable_rejection_body_is_reported_as_evidence_not_as_a_reason() {
    let (url, task) = servers(vec![
        http_json("400 Bad Request", "not-json"),
        http_json(
            "400 Bad Request",
            r#"{"errorCode":"FIXTURE","errorMessage":"second"}"#,
        ),
    ])
    .await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let first = client
        .configure_account(&account, &configure_body())
        .await
        .expect_err("rejected");
    match first {
        OperationError::Rejected {
            status,
            code,
            message,
            body,
        } => {
            assert_eq!(status, 400);
            // No code, no message, and above all no reason this client did not
            // read presented as one OANDA supplied.
            assert!(code.is_none());
            assert!(message.is_none());
            assert!(matches!(body, Supplied::Undecoded), "a body did arrive");
        }
        other => panic!("unexpected error: {other:?}"),
    }
    // The rejection is definitive, so the account must not be fenced. A second
    // mutation has to reach the provider rather than stop at ReconciliationRequired.
    let second = client
        .configure_account(&account, &configure_body())
        .await
        .expect_err("rejected");
    match second {
        OperationError::Rejected {
            status: 400,
            code: Some(code),
            message: Some(message),
            body,
        } => {
            assert_eq!(code, "FIXTURE");
            assert_eq!(message, "second");
            assert!(
                matches!(body, Supplied::Decoded(_)),
                "the second body decoded"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
    let requests = task.await.expect("server");
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn a_rejection_with_no_body_is_absent_rather_than_guessed() {
    let (url, task) = server(
        b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec(),
    )
    .await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let error = client
        .configure_account(&account, &configure_body())
        .await
        .expect_err("rejected");
    match error {
        OperationError::Rejected {
            status: 400,
            code,
            message,
            body,
        } => {
            assert!(code.is_none());
            assert!(message.is_none());
            assert!(matches!(body, Supplied::Absent), "no body arrived");
        }
        other => panic!("unexpected error: {other:?}"),
    }
    task.await.expect("server");
}

#[tokio::test]
async fn a_readable_stream_rejection_keeps_its_reason() {
    let reason = r#"{"errorCode":"AUTH","errorMessage":"bad token"}"#;
    let (url, task) = server(http_json("401 Unauthorized", reason)).await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let Err(error) = client.stream_transactions(&account).await else {
        panic!("expected a provider rejection")
    };
    match error {
        Error::Provider {
            status,
            code,
            message,
        } => {
            assert_eq!(status, 401);
            assert_eq!(code.as_deref(), Some("AUTH"));
            assert_eq!(message.as_deref(), Some("bad token"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    task.await.expect("server");
}

#[tokio::test]
async fn an_unreadable_stream_rejection_reports_no_reason() {
    let (url, task) = server(
        b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 21\r\nConnection: close\r\n\r\ntoken rejected at edge".to_vec(),
    )
    .await;
    let client = fixture_client(url);
    let account = AccountID::new("101-001-1-001").expect("account");
    let Err(error) = client.stream_transactions(&account).await else {
        panic!("expected a provider rejection")
    };
    match error {
        Error::Provider {
            status,
            code,
            message,
        } => {
            assert_eq!(status, 401);
            assert!(code.is_none(), "no code was readable: {code:?}");
            assert!(
                message.is_none(),
                "no provider message was readable: {message:?}"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
    task.await.expect("server");
}

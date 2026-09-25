//! Local fixture coverage for all 30 documented REST operations.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::too_many_lines,
    unused_variables
)]
use oanda_client::{
    AccountID, Client, Environment, InstrumentName, OperationError, OrderSpecifier, TradeSpecifier,
    TransactionID,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{fmt::Debug, future::Future};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use url::Url;

fn fixture<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("valid fixture")
}
async fn check<T, E, F, Fut>(method: &str, path: &str, expected_body: Option<Value>, call: F)
where
    T: DeserializeOwned + Debug,
    E: Debug,
    F: Fn(Client) -> Fut,
    Fut: Future<Output = Result<oanda_client::ApiResponse<T>, OperationError<E>>>,
{
    for success in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let url = Url::parse(&format!(
            "http://{}/",
            listener.local_addr().expect("address")
        ))
        .expect("url");
        let success_body = if path.ends_with("/pricing") {
            r#"{"prices":[]}"#
        } else {
            "{}"
        };
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut request = Vec::new();
            let mut buf = [0_u8; 4096];
            loop {
                let count = socket.read(&mut buf).await.expect("read");
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buf[..count]);
                if let Some(pos) = request.windows(4).position(|x| x == b"\r\n\r\n") {
                    let head = String::from_utf8_lossy(&request[..pos + 4]);
                    let len = head
                        .lines()
                        .find_map(|l| {
                            l.to_ascii_lowercase()
                                .strip_prefix("content-length: ")
                                .and_then(|x| x.trim().parse::<usize>().ok())
                        })
                        .unwrap_or(0);
                    if request.len() >= pos + 4 + len {
                        break;
                    }
                }
            }
            let body = if success {
                success_body
            } else {
                r#"{"errorCode":"FIXTURE_REJECT","errorMessage":"rejected by fixture"}"#
            };
            let status = if success { "200 OK" } else { "400 Bad Request" };
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.expect("write");
            request
        });
        let client = Client::builder(Environment::Practice, "fixture-secret")
            .endpoints(url.clone(), url)
            .build()
            .expect("client");
        let result = call(client).await;
        if success {
            assert!(result.is_ok(), "{method} {path}: {result:?}");
        } else {
            assert!(
                matches!(
                    result,
                    Err(OperationError::Rejected {
                        status: 400,
                        body: Some(_),
                        ..
                    })
                ),
                "{method} {path}: {result:?}"
            );
        }
        let bytes = server.await.expect("server");
        let split = bytes
            .windows(4)
            .position(|x| x == b"\r\n\r\n")
            .expect("headers");
        let head = String::from_utf8_lossy(&bytes[..split]);
        let target = head.lines().next().expect("request line");
        assert!(target.starts_with(&format!("{method} {path}")), "{target}");
        let target_url = Url::parse(&format!(
            "http://fixture{}",
            target.split_whitespace().nth(1).expect("target")
        ))
        .expect("target URL");
        assert_eq!(target_url.path(), path, "{target}");
        let pairs: Vec<_> = target_url.query_pairs().collect();
        let expected_query: &[(&str, &str)] = match path {
            "/v3/accounts/101-001-1-001/instruments" | "/v3/accounts/101-001-1-001/pricing" => {
                &[("instruments", "EUR_USD")]
            }
            "/v3/accounts/101-001-1-001/changes" => &[("sinceTransactionID", "1")],
            "/v3/accounts/101-001-1-001/orders" if method == "GET" => &[("count", "1")],
            "/v3/accounts/101-001-1-001/trades" => &[("count", "1")],
            "/v3/accounts/101-001-1-001/transactions" => &[("pageSize", "1")],
            "/v3/accounts/101-001-1-001/transactions/idrange" => &[("from", "1"), ("to", "2")],
            "/v3/accounts/101-001-1-001/transactions/sinceid" => &[("id", "1")],
            "/v3/accounts/101-001-1-001/candles/latest" => {
                &[("candleSpecifications", "EUR_USD:M5:B")]
            }
            "/v3/accounts/101-001-1-001/instruments/EUR_USD/candles" => {
                &[("count", "1"), ("price", "M")]
            }
            _ => &[],
        };
        assert_eq!(
            pairs.len(),
            expected_query.len(),
            "{method} {path}: {target}"
        );
        for (key, value) in expected_query {
            assert!(
                pairs.iter().any(|(k, v)| k == key && v == value),
                "{method} {path}: {target}"
            );
        }
        assert!(
            head.to_ascii_lowercase()
                .contains("authorization: bearer fixture-secret")
        );
        assert!(
            head.to_ascii_lowercase()
                .contains("accept-datetime-format: rfc3339")
        );
        if let Some(ref expected_body) = expected_body {
            let actual: Value = serde_json::from_slice(&bytes[split + 4..]).expect("body JSON");
            assert_eq!(&actual, expected_body, "{method} {path}");
        }
    }
}

#[tokio::test]
async fn all_documented_rest_operations_have_wire_fixtures() {
    // list_accounts
    check("GET", "/v3/accounts", None, |client| async move {
        let account_id = AccountID::new("101-001-1-001").expect("account");
        let order_specifier = OrderSpecifier::new("10").expect("order");
        let trade_specifier = TradeSpecifier::new("20").expect("trade");
        let transaction_id = TransactionID::new("30").expect("transaction");
        let instrument = InstrumentName::new("EUR_USD").expect("instrument");
        client.list_accounts().await
    })
    .await;
    // account_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.account_details(&account_id).await
        },
    )
    .await;
    // account_summary
    check(
        "GET",
        "/v3/accounts/101-001-1-001/summary",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.account_summary(&account_id).await
        },
    )
    .await;
    // account_instruments
    check(
        "GET",
        "/v3/accounts/101-001-1-001/instruments",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::account::AccountInstrumentsQuery =
                fixture(json!({"instruments":["EUR_USD"]}));
            client.account_instruments(&account_id, Some(&query)).await
        },
    )
    .await;
    // configure_account
    check(
        "PATCH",
        "/v3/accounts/101-001-1-001/configuration",
        Some(json!({"alias":"fixture"})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::account::ConfigureAccountBody =
                fixture(json!({"alias":"fixture"}));
            client.configure_account(&account_id, &body).await
        },
    )
    .await;
    // account_changes
    check(
        "GET",
        "/v3/accounts/101-001-1-001/changes",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::account::AccountChangesQuery =
                fixture(json!({"sinceTransactionID":"1"}));
            client.account_changes(&account_id, Some(&query)).await
        },
    )
    .await;
    // create_order
    check("POST", "/v3/accounts/101-001-1-001/orders", Some(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}})), |client| async move {
        let account_id = AccountID::new("101-001-1-001").expect("account");
        let order_specifier = OrderSpecifier::new("10").expect("order");
        let trade_specifier = TradeSpecifier::new("20").expect("trade");
        let transaction_id = TransactionID::new("30").expect("transaction");
        let instrument = InstrumentName::new("EUR_USD").expect("instrument");
        let body: oanda_client::order::CreateOrderBody = fixture(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}}));
        client.create_order(&account_id, &body).await
    }).await;
    // list_orders
    check(
        "GET",
        "/v3/accounts/101-001-1-001/orders",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::order::ListOrdersQuery = fixture(json!({"count":1}));
            client.list_orders(&account_id, Some(&query)).await
        },
    )
    .await;
    // list_pending_orders
    check(
        "GET",
        "/v3/accounts/101-001-1-001/pendingOrders",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.list_pending_orders(&account_id).await
        },
    )
    .await;
    // order_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/orders/10",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.order_details(&account_id, &order_specifier).await
        },
    )
    .await;
    // replace_order
    check("PUT", "/v3/accounts/101-001-1-001/orders/10", Some(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}})), |client| async move {
        let account_id = AccountID::new("101-001-1-001").expect("account");
        let order_specifier = OrderSpecifier::new("10").expect("order");
        let trade_specifier = TradeSpecifier::new("20").expect("trade");
        let transaction_id = TransactionID::new("30").expect("transaction");
        let instrument = InstrumentName::new("EUR_USD").expect("instrument");
        let body: oanda_client::order::ReplaceOrderBody = fixture(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}}));
        client.replace_order(&account_id, &order_specifier, &body, None).await
    }).await;
    // cancel_order
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/orders/10/cancel",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client
                .cancel_order(&account_id, &order_specifier, None)
                .await
        },
    )
    .await;
    // update_order_client_extensions
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/orders/10/clientExtensions",
        Some(json!({"clientExtensions":{"id":"fixture"}})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::order::UpdateOrderClientExtensionsBody =
                fixture(json!({"clientExtensions":{"id":"fixture"}}));
            client
                .update_order_client_extensions(&account_id, &order_specifier, &body)
                .await
        },
    )
    .await;
    // list_trades
    check(
        "GET",
        "/v3/accounts/101-001-1-001/trades",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::trade::ListTradesQuery = fixture(json!({"count":1}));
            client.list_trades(&account_id, Some(&query)).await
        },
    )
    .await;
    // list_open_trades
    check(
        "GET",
        "/v3/accounts/101-001-1-001/openTrades",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.list_open_trades(&account_id).await
        },
    )
    .await;
    // trade_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/trades/20",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.trade_details(&account_id, &trade_specifier).await
        },
    )
    .await;
    // close_trade
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/trades/20/close",
        Some(json!({"units":"ALL"})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::trade::CloseTradeBody = fixture(json!({"units":"ALL"}));
            client
                .close_trade(&account_id, &trade_specifier, &body)
                .await
        },
    )
    .await;
    // update_trade_client_extensions
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/trades/20/clientExtensions",
        Some(json!({"clientExtensions":{"id":"fixture"}})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::trade::UpdateTradeClientExtensionsBody =
                fixture(json!({"clientExtensions":{"id":"fixture"}}));
            client
                .update_trade_client_extensions(&account_id, &trade_specifier, &body)
                .await
        },
    )
    .await;
    // set_trade_dependent_orders
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/trades/20/orders",
        Some(json!({"takeProfit":null})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::trade::SetTradeDependentOrdersBody =
                fixture(json!({"takeProfit":null}));
            client
                .set_trade_dependent_orders(&account_id, &trade_specifier, &body)
                .await
        },
    )
    .await;
    // list_positions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/positions",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.list_positions(&account_id).await
        },
    )
    .await;
    // list_open_positions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/openPositions",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.list_open_positions(&account_id).await
        },
    )
    .await;
    // position_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/positions/EUR_USD",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client.position_details(&account_id, &instrument).await
        },
    )
    .await;
    // close_position
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/positions/EUR_USD/close",
        Some(json!({"longUnits":"ALL"})),
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let body: oanda_client::position::ClosePositionBody =
                fixture(json!({"longUnits":"ALL"}));
            client.close_position(&account_id, &instrument, &body).await
        },
    )
    .await;
    // list_transactions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::transaction::ListTransactionsQuery =
                fixture(json!({"pageSize":1}));
            client.list_transactions(&account_id, Some(&query)).await
        },
    )
    .await;
    // transaction_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions/30",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            client
                .transaction_details(&account_id, &transaction_id)
                .await
        },
    )
    .await;
    // transactions_by_id_range
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions/idrange",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::transaction::TransactionsByIdRangeQuery =
                fixture(json!({"from":"1","to":"2"}));
            client.transactions_by_id_range(&account_id, &query).await
        },
    )
    .await;
    // transactions_since_id
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions/sinceid",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::transaction::TransactionsSinceIdQuery =
                fixture(json!({"id":"1"}));
            client.transactions_since_id(&account_id, &query).await
        },
    )
    .await;
    // latest_candles
    check(
        "GET",
        "/v3/accounts/101-001-1-001/candles/latest",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::pricing::LatestCandlesQuery =
                fixture(json!({"candleSpecifications":["EUR_USD:M5:B"]}));
            client.latest_candles(&account_id, &query).await
        },
    )
    .await;
    // pricing
    check(
        "GET",
        "/v3/accounts/101-001-1-001/pricing",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::pricing::PricingQuery =
                fixture(json!({"instruments":["EUR_USD"]}));
            client.pricing(&account_id, &query).await
        },
    )
    .await;
    // instrument_candles
    check(
        "GET",
        "/v3/accounts/101-001-1-001/instruments/EUR_USD/candles",
        None,
        |client| async move {
            let account_id = AccountID::new("101-001-1-001").expect("account");
            let order_specifier = OrderSpecifier::new("10").expect("order");
            let trade_specifier = TradeSpecifier::new("20").expect("trade");
            let transaction_id = TransactionID::new("30").expect("transaction");
            let instrument = InstrumentName::new("EUR_USD").expect("instrument");
            let query: oanda_client::pricing::InstrumentCandlesQuery =
                fixture(json!({"count":1,"price":"M"}));
            client
                .instrument_candles(&account_id, &instrument, Some(&query))
                .await
        },
    )
    .await;
}

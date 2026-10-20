// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Local fixture coverage for all 30 documented REST operations.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]
use oanda_client::{
    AccountID, Client, InstrumentName, OperationError, OrderSpecifier, Supplied, TradeSpecifier,
    TransactionID,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{fmt::Debug, future::Future};
mod support;
use support::{fixture_client, http, server};
use url::Url;

fn fixture<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("valid fixture")
}
#[allow(
    clippy::too_many_lines,
    reason = "wire assertions share the request and typed operation result"
)]
async fn check<T, E, F, Fut>(method: &str, path: &str, expected_body: Option<Value>, call: F)
where
    T: DeserializeOwned + Debug,
    E: Debug,
    F: Fn(Client) -> Fut,
    Fut: Future<Output = Result<oanda_client::ApiResponse<T>, OperationError<E>>>,
{
    for (success, missing_message) in [(false, false), (true, false), (false, true)] {
        // Every one of the nine mutation-specific rejection schemas requires
        // errorMessage. A malformed supplied body must remain distinct from absence.
        if missing_message && method == "GET" {
            continue;
        }
        let success_body = if path.ends_with("/pricing") {
            r#"{"prices":[]}"#
        } else {
            "{}"
        };
        let body = if success {
            success_body
        } else if missing_message {
            r#"{"errorCode":"FIXTURE_REJECT"}"#
        } else {
            r#"{"errorCode":"FIXTURE_REJECT","errorMessage":"rejected by fixture"}"#
        };
        let status = if success { "200 OK" } else { "400 Bad Request" };
        let (url, server) = server(http(status, "application/json", body)).await;
        let client = fixture_client(url);
        let result = call(client).await;
        if success {
            assert!(result.is_ok(), "{method} {path}: {result:?}");
        } else if missing_message {
            assert!(
                matches!(
                    result,
                    Err(OperationError::Rejected {
                        status: 400,
                        body: Supplied::Undecoded,
                        ..
                    })
                ),
                "missing required rejection message for {method} {path}: {result:?}"
            );
        } else {
            assert!(
                matches!(
                    result,
                    Err(OperationError::Rejected {
                        status: 400,
                        body: Supplied::Decoded(_),
                        ..
                    })
                ),
                "{method} {path}: {result:?}"
            );
        }
        let request = server.await.expect("server");
        let bytes = request.as_bytes();
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

#[allow(
    clippy::too_many_lines,
    reason = "one fixture per operation makes the complete operation ledger reviewable"
)]
#[tokio::test]
async fn all_documented_rest_operations_have_wire_fixtures() {
    // list_accounts
    check("GET", "/v3/accounts", None, |client| async move {
        client.list_accounts().await
    })
    .await;
    // account_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001",
        None,
        |client| async move { client.account_details(&account_id()).await },
    )
    .await;
    // account_summary
    check(
        "GET",
        "/v3/accounts/101-001-1-001/summary",
        None,
        |client| async move { client.account_summary(&account_id()).await },
    )
    .await;
    // account_instruments
    check(
        "GET",
        "/v3/accounts/101-001-1-001/instruments",
        None,
        |client| async move {
            let query: oanda_client::account::AccountInstrumentsQuery =
                fixture(json!({"instruments":["EUR_USD"]}));
            client
                .account_instruments(&account_id(), Some(&query))
                .await
        },
    )
    .await;
    // configure_account
    check(
        "PATCH",
        "/v3/accounts/101-001-1-001/configuration",
        Some(json!({"alias":"fixture"})),
        |client| async move {
            let body: oanda_client::account::ConfigureAccountBody =
                fixture(json!({"alias":"fixture"}));
            client.configure_account(&account_id(), &body).await
        },
    )
    .await;
    // account_changes
    check(
        "GET",
        "/v3/accounts/101-001-1-001/changes",
        None,
        |client| async move {
            let query: oanda_client::account::AccountChangesQuery =
                fixture(json!({"sinceTransactionID":"1"}));
            client.account_changes(&account_id(), Some(&query)).await
        },
    )
    .await;
    // create_order
    check("POST", "/v3/accounts/101-001-1-001/orders", Some(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}})), |client| async move {
        let body: oanda_client::order::CreateOrderBody = fixture(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}}));
        client.create_order(&account_id(), &body).await
    }).await;
    // list_orders
    check(
        "GET",
        "/v3/accounts/101-001-1-001/orders",
        None,
        |client| async move {
            let query: oanda_client::order::ListOrdersQuery = fixture(json!({"count":1}));
            client.list_orders(&account_id(), Some(&query)).await
        },
    )
    .await;
    // list_pending_orders
    check(
        "GET",
        "/v3/accounts/101-001-1-001/pendingOrders",
        None,
        |client| async move { client.list_pending_orders(&account_id()).await },
    )
    .await;
    // order_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/orders/10",
        None,
        |client| async move {
            client
                .order_details(&account_id(), &order_specifier())
                .await
        },
    )
    .await;
    // replace_order
    check("PUT", "/v3/accounts/101-001-1-001/orders/10", Some(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}})), |client| async move {
        let body: oanda_client::order::ReplaceOrderBody = fixture(json!({"order":{"type":"MARKET","instrument":"EUR_USD","units":"1","timeInForce":"FOK","positionFill":"DEFAULT"}}));
        client.replace_order(&account_id(), &order_specifier(), &body, None).await
    }).await;
    // cancel_order
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/orders/10/cancel",
        None,
        |client| async move {
            client
                .cancel_order(&account_id(), &order_specifier(), None)
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
            let body: oanda_client::order::UpdateOrderClientExtensionsBody =
                fixture(json!({"clientExtensions":{"id":"fixture"}}));
            client
                .update_order_client_extensions(&account_id(), &order_specifier(), &body)
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
            let query: oanda_client::trade::ListTradesQuery = fixture(json!({"count":1}));
            client.list_trades(&account_id(), Some(&query)).await
        },
    )
    .await;
    // list_open_trades
    check(
        "GET",
        "/v3/accounts/101-001-1-001/openTrades",
        None,
        |client| async move { client.list_open_trades(&account_id()).await },
    )
    .await;
    // trade_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/trades/20",
        None,
        |client| async move {
            client
                .trade_details(&account_id(), &trade_specifier())
                .await
        },
    )
    .await;
    // close_trade
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/trades/20/close",
        Some(json!({"units":"ALL"})),
        |client| async move {
            let body: oanda_client::trade::CloseTradeBody = fixture(json!({"units":"ALL"}));
            client
                .close_trade(&account_id(), &trade_specifier(), &body)
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
            let body: oanda_client::trade::UpdateTradeClientExtensionsBody =
                fixture(json!({"clientExtensions":{"id":"fixture"}}));
            client
                .update_trade_client_extensions(&account_id(), &trade_specifier(), &body)
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
            let body: oanda_client::trade::SetTradeDependentOrdersBody =
                fixture(json!({"takeProfit":null}));
            client
                .set_trade_dependent_orders(&account_id(), &trade_specifier(), &body)
                .await
        },
    )
    .await;
    // list_positions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/positions",
        None,
        |client| async move { client.list_positions(&account_id()).await },
    )
    .await;
    // list_open_positions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/openPositions",
        None,
        |client| async move { client.list_open_positions(&account_id()).await },
    )
    .await;
    // position_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/positions/EUR_USD",
        None,
        |client| async move { client.position_details(&account_id(), &instrument()).await },
    )
    .await;
    // close_position
    check(
        "PUT",
        "/v3/accounts/101-001-1-001/positions/EUR_USD/close",
        Some(json!({"longUnits":"ALL"})),
        |client| async move {
            let body: oanda_client::position::ClosePositionBody =
                fixture(json!({"longUnits":"ALL"}));
            client
                .close_position(&account_id(), &instrument(), &body)
                .await
        },
    )
    .await;
    // list_transactions
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions",
        None,
        |client| async move {
            let query: oanda_client::transaction::ListTransactionsQuery =
                fixture(json!({"pageSize":1}));
            client.list_transactions(&account_id(), Some(&query)).await
        },
    )
    .await;
    // transaction_details
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions/30",
        None,
        |client| async move {
            client
                .transaction_details(&account_id(), &transaction_id())
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
            let query: oanda_client::transaction::TransactionsByIdRangeQuery =
                fixture(json!({"from":"1","to":"2"}));
            client.transactions_by_id_range(&account_id(), &query).await
        },
    )
    .await;
    // transactions_since_id
    check(
        "GET",
        "/v3/accounts/101-001-1-001/transactions/sinceid",
        None,
        |client| async move {
            let query: oanda_client::transaction::TransactionsSinceIdQuery =
                fixture(json!({"id":"1"}));
            client.transactions_since_id(&account_id(), &query).await
        },
    )
    .await;
    // latest_candles
    check(
        "GET",
        "/v3/accounts/101-001-1-001/candles/latest",
        None,
        |client| async move {
            let query: oanda_client::pricing::LatestCandlesQuery =
                fixture(json!({"candleSpecifications":["EUR_USD:M5:B"]}));
            client.latest_candles(&account_id(), &query).await
        },
    )
    .await;
    // pricing
    check(
        "GET",
        "/v3/accounts/101-001-1-001/pricing",
        None,
        |client| async move {
            let query: oanda_client::pricing::PricingQuery =
                fixture(json!({"instruments":["EUR_USD"]}));
            client.pricing(&account_id(), &query).await
        },
    )
    .await;
    // instrument_candles
    check(
        "GET",
        "/v3/accounts/101-001-1-001/instruments/EUR_USD/candles",
        None,
        |client| async move {
            let query: oanda_client::pricing::InstrumentCandlesQuery =
                fixture(json!({"count":1,"price":"M"}));
            client
                .instrument_candles(&account_id(), &instrument(), Some(&query))
                .await
        },
    )
    .await;
}

fn account_id() -> AccountID {
    AccountID::new("101-001-1-001").expect("fixture account_id")
}

fn order_specifier() -> OrderSpecifier {
    OrderSpecifier::new("10").expect("fixture order_specifier")
}

fn trade_specifier() -> TradeSpecifier {
    TradeSpecifier::new("20").expect("fixture trade_specifier")
}

fn transaction_id() -> TransactionID {
    TransactionID::new("30").expect("fixture transaction_id")
}

fn instrument() -> InstrumentName {
    InstrumentName::new("EUR_USD").expect("fixture instrument")
}

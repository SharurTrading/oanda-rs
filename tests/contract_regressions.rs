// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Observable boundary checks for audit-remediated mutation and wire contracts.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]

use oanda_client::{
    AccountID, Error, InstrumentName, OperationError, Patch, PositionCloseoutUnits,
    TradeCloseoutUnits, TradeSpecifier,
    models::{MarketOrderRequest, OrderRequest, OrderType, StopLossDetails},
    position::ClosePositionBody,
    trade::{CloseTradeBody, SetTradeDependentOrdersBody},
};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use tokio::net::TcpListener;
use url::Url;

mod support;
use support::{fixture_client, http, server};

fn account() -> AccountID {
    AccountID::new("101-001-1-001").expect("account")
}

fn trade() -> TradeSpecifier {
    TradeSpecifier::new("20").expect("trade")
}

fn dependent_orders(stop_loss: Patch<StopLossDetails>) -> SetTradeDependentOrdersBody {
    SetTradeDependentOrdersBody {
        take_profit: Patch::Unchanged,
        stop_loss,
        trailing_stop_loss: Patch::Unchanged,
        guaranteed_stop_loss: Patch::Unchanged,
    }
}

fn request_body(request: &str) -> Value {
    let (_, body) = request.split_once("\r\n\r\n").expect("HTTP body");
    serde_json::from_str(body).expect("body JSON")
}

#[test]
fn unchanged_patch_requires_an_enclosing_omission_policy() {
    let error = serde_json::to_value(Patch::<StopLossDetails>::Unchanged)
        .expect_err("standalone unchanged cannot fabricate null");
    assert!(
        error
            .to_string()
            .contains("unchanged patch must be omitted")
    );
    let body = dependent_orders(Patch::Unchanged);
    assert_eq!(
        serde_json::to_value(body).expect("omitted fields"),
        json!({})
    );
}

#[test]
fn union_variant_cannot_overwrite_a_conflicting_nested_tag() {
    let mut market: MarketOrderRequest = serde_json::from_value(json!({
        "type":"MARKET", "instrument":"EUR_USD", "units":"1", "timeInForce":"FOK", "positionFill":"DEFAULT"
    }))
    .expect("market request");
    market.r#type = Some(OrderType::Limit);
    assert!(
        serde_json::to_value(&market).is_err(),
        "a raw concrete DTO must reject a contradictory tag too"
    );
    let request = OrderRequest::MarketOrderRequest(market);
    assert!(
        serde_json::to_value(request).is_err(),
        "a conflicting LIMIT tag must fail"
    );
}

#[tokio::test]
async fn partial_stop_loss_modification_preserves_provider_omission_defaults() {
    // Omitting price and distance is a documented modification contract: retain
    // the existing value while changing another property.
    let body = dependent_orders(Patch::Set(
        serde_json::from_value(json!({
            "clientExtensions": {"tag":"retain-existing-price"}
        }))
        .expect("partial stop loss"),
    ));
    let (url, task) = server(http("200 OK", "application/json", "{}")).await;
    fixture_client(url)
        .set_trade_dependent_orders(&account(), &trade(), &body)
        .await
        .expect("valid partial modification");
    assert_eq!(
        request_body(&task.await.expect("server")),
        json!({
            "stopLoss":{"clientExtensions":{"tag":"retain-existing-price"}}
        })
    );
}

#[tokio::test]
async fn conflicting_stop_loss_price_and_distance_fail_before_network_and_preserve_admission() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let client = fixture_client(url);
    let body = dependent_orders(Patch::Set(
        serde_json::from_value(json!({
            "price":"1.1", "distance":"0.01"
        }))
        .expect("representable conflicting details"),
    ));
    assert!(matches!(
        client
            .set_trade_dependent_orders(&account(), &trade(), &body)
            .await,
        Err(OperationError::Client(Error::InvalidInput(_)))
    ));
    assert!(
        futures_util::poll!(Box::pin(listener.accept())).is_pending(),
        "invalid request was sent"
    );
    // The local failure must not fence or consume the only ordinary mutation slot.
    client.set_exit_reserve(99).expect("one ordinary slot");
    let task = tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        let (mut socket, _) = listener.accept().await.expect("valid request");
        let bytes = support::read_request(&mut socket, &mut Vec::new())
            .await
            .expect("request body");
        socket
            .write_all(&http("200 OK", "application/json", "{}"))
            .await
            .expect("response");
        String::from_utf8(bytes).expect("request UTF-8")
    });
    client
        .set_trade_dependent_orders(&account(), &trade(), &dependent_orders(Patch::Clear))
        .await
        .expect("valid request after local failure");
    assert_eq!(
        request_body(&task.await.expect("server")),
        json!({"stopLoss":null})
    );
}

#[tokio::test]
async fn omitted_position_sides_send_the_documented_all_defaults() {
    let body: ClosePositionBody = serde_json::from_value(json!({})).expect("default ALL body");
    let (url, task) = server(http("200 OK", "application/json", "{}")).await;
    fixture_client(url)
        .close_position(
            &account(),
            &InstrumentName::new("EUR_USD").expect("instrument"),
            &body,
        )
        .await
        .expect("both omitted sides default to ALL");
    assert_eq!(request_body(&task.await.expect("server")), json!({}));
}

#[tokio::test]
async fn nonpositive_closeout_quantities_are_rejected_without_a_network_send() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind");
    let url = Url::parse(&format!(
        "http://{}/",
        listener.local_addr().expect("address")
    ))
    .expect("URL");
    let client = fixture_client(url);
    for units in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
        assert!(matches!(
            client
                .close_trade(
                    &account(),
                    &trade(),
                    &CloseTradeBody {
                        units: Some(TradeCloseoutUnits::Units(units)),
                    }
                )
                .await,
            Err(OperationError::Client(Error::InvalidInput(_)))
        ));
        let position = ClosePositionBody {
            long_units: Some(PositionCloseoutUnits::Units(units)),
            long_client_extensions: None,
            short_units: Some(PositionCloseoutUnits::None),
            short_client_extensions: None,
        };
        assert!(matches!(
            client
                .close_position(
                    &account(),
                    &InstrumentName::new("EUR_USD").expect("instrument"),
                    &position
                )
                .await,
            Err(OperationError::Client(Error::InvalidInput(_)))
        ));
    }
    assert!(
        futures_util::poll!(Box::pin(listener.accept())).is_pending(),
        "invalid quantity was sent"
    );
}

#[tokio::test]
async fn truncated_definitive_rejection_does_not_fence_the_next_mutation() {
    let (url, task) = support::servers(vec![
        b"HTTP/1.1 400 Bad Request\r\nRequestID: 41\r\nContent-Length: 100\r\nConnection: close\r\n\r\ncut-short"
            .to_vec(),
        http(
            "200 OK",
            "application/json",
            r#"{"lastTransactionID":"19"}"#,
        ),
    ])
    .await;
    let client = fixture_client(url);
    let body = oanda_client::account::ConfigureAccountBody {
        alias: Some("fixture".to_owned()),
        margin_rate: None,
    };
    let error = client
        .configure_account(&account(), &body)
        .await
        .expect_err("truncated rejection");
    match error {
        OperationError::RejectedUnreadable {
            status: 400,
            source: Error::Transport(_),
            request_id,
            ..
        } => {
            assert_eq!(
                request_id,
                oanda_client::Supplied::Decoded(
                    oanda_client::RequestID::new("41").expect("request ID")
                )
            );
        }
        other => panic!("definitive rejection must retain its status and read error: {other:?}"),
    }
    let response = client
        .configure_account(&account(), &body)
        .await
        .expect("no false reconciliation fence");
    assert_eq!(
        response
            .body
            .last_transaction_id
            .as_ref()
            .map(oanda_client::TransactionID::as_str),
        Some("19")
    );
    assert_eq!(task.await.expect("both requests").len(), 2);
}

#[tokio::test]
async fn successful_mutation_keeps_accepted_body_when_pagination_is_unsafe() {
    for link in [
        "<https://example.com/v3/accounts?page=2>; rel=\"next\"",
        "</v3/accounts?page=2; rel=\"next\"",
    ] {
        let body = r#"{"lastTransactionID":"23"}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nLink: {link}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let (url, task) = server(response.into_bytes()).await;
        let request = oanda_client::account::ConfigureAccountBody {
            alias: Some("fixture".to_owned()),
            margin_rate: None,
        };
        let response = fixture_client(url)
            .configure_account(&account(), &request)
            .await
            .expect("accepted mutation evidence");
        assert_eq!(
            response
                .body
                .last_transaction_id
                .as_ref()
                .map(oanda_client::TransactionID::as_str),
            Some("23")
        );
        assert!(
            response.next_page.is_undecoded(),
            "unsafe supplied pagination must remain distinguishable from absence"
        );
        task.await.expect("server");
    }
}

#[tokio::test]
async fn direct_stop_loss_orders_reject_price_and_distance_but_admit_price_only() {
    for kind in ["STOP_LOSS", "GUARANTEED_STOP_LOSS"] {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("loopback bind");
        let url = Url::parse(&format!(
            "http://{}/",
            listener.local_addr().expect("address")
        ))
        .expect("URL");
        let client = fixture_client(url);
        let mut order = json!({"type":kind,"tradeID":"20","price":"1.1","distance":"0.01","timeInForce":"GTC","triggerCondition":"DEFAULT"});
        let conflicting = oanda_client::order::CreateOrderBody {
            order: Some(
                serde_json::from_value(order.clone()).expect("representable conflicting order"),
            ),
        };
        assert!(
            matches!(
                client.create_order(&account(), &conflicting).await,
                Err(OperationError::Client(Error::InvalidInput(_)))
            ),
            "{kind} accepted conflicting price and distance"
        );
        assert!(
            futures_util::poll!(Box::pin(listener.accept())).is_pending(),
            "{kind} invalid request was sent"
        );
        order
            .as_object_mut()
            .expect("order object")
            .remove("distance");
        let valid = oanda_client::order::CreateOrderBody {
            order: Some(serde_json::from_value(order.clone()).expect("price-only request")),
        };
        let task = tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let (mut socket, _) = listener.accept().await.expect("valid request");
            let bytes = support::read_request(&mut socket, &mut Vec::new())
                .await
                .expect("request body");
            socket
                .write_all(&http("200 OK", "application/json", "{}"))
                .await
                .expect("response");
            String::from_utf8(bytes).expect("request UTF-8")
        });
        client
            .create_order(&account(), &valid)
            .await
            .expect("price-only request after rejected conflict");
        assert_eq!(
            request_body(&task.await.expect("server")),
            json!({"order":order})
        );
    }
}

#[tokio::test]
async fn an_empty_request_id_is_present_but_not_a_valid_provider_identity() {
    let (url, task) = server(
        b"HTTP/1.1 200 OK\r\nRequestID: \r\nContent-Type: application/json\r\nContent-Length: 15\r\nConnection: close\r\n\r\n{\"accounts\":[]}".to_vec(),
    ).await;
    let response = fixture_client(url)
        .list_accounts()
        .await
        .expect("account list");
    assert_eq!(response.request_id, oanda_client::Supplied::Undecoded);
    task.await.expect("server");
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared transport admission through the public client and loopback sockets.

// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]

use oanda_client::{
    AccountID, Admission, Client, Environment, Error, OperationError, OrderSpecifier,
};
use std::time::Duration;
use tokio::time::Instant;

mod support;
use support::{counting_rest_loopback, loopback, rate_limit_loopback, stall_first_loopback};
use support::{fixture_client_with as client, keep_clock_frozen};

fn account() -> AccountID {
    AccountID::new("101-001-1-001").expect("account")
}

async fn advance_gap() {
    tokio::time::pause();
    tokio::time::advance(Duration::from_millis(500)).await;
    tokio::time::resume();
}

#[tokio::test(start_paused = true)]
async fn warmed_rest_connection_sends_one_hundred_twenty_requests_then_refuses_mutation() {
    let clock_guard = keep_clock_frozen();
    let (rest, mut requests, rest_server) = counting_rest_loopback().await;
    let (stream, _accepts, stream_server) = loopback().await;
    let client = Client::builder(Environment::Practice, "token-a")
        .endpoints(rest, stream)
        .build()
        .expect("client");
    let start = Instant::now();
    for _ in 0..120 {
        client.list_accounts().await.expect("REST response");
        assert!(
            requests
                .recv()
                .await
                .expect("wire request")
                .starts_with("GET /v3/accounts HTTP/1.1")
        );
    }
    assert_eq!(Instant::now(), start);
    let order = OrderSpecifier::new("fixture-order").expect("order");
    assert!(matches!(
        client.cancel_order(&account(), &order, None).await,
        Err(OperationError::Client(Error::RateLimited))
    ));
    assert!(requests.try_recv().is_err());
    let read = client.list_accounts();
    tokio::pin!(read);
    assert!(futures_util::poll!(read.as_mut()).is_pending());
    tokio::time::advance(Duration::from_millis(999)).await;
    assert!(requests.try_recv().is_err());
    tokio::time::advance(Duration::from_millis(1)).await;
    read.await.expect("REST capacity returned");
    assert!(
        requests
            .recv()
            .await
            .expect("new window request")
            .starts_with("GET /v3/accounts HTTP/1.1")
    );
    clock_guard.abort();
    rest_server.abort();
    stream_server.abort();
}

#[tokio::test(start_paused = true)]
async fn stream_start_does_not_consume_exhausted_rest_capacity() {
    let clock_guard = keep_clock_frozen();
    let (rest, mut requests, rest_server) = counting_rest_loopback().await;
    let (stream, mut accepts, stream_server) = loopback().await;
    let client = Client::builder(Environment::Practice, "token-a")
        .endpoints(rest, stream)
        .build()
        .expect("client");
    for _ in 0..120 {
        client.list_accounts().await.expect("REST response");
        requests.recv().await.expect("wire request");
    }
    let started = Instant::now();
    let stream = client
        .stream_transactions(&account())
        .await
        .expect("stream despite exhausted REST budget");
    assert_eq!(accepts.recv().await, Some(started));
    assert_eq!(Instant::now(), started);
    assert!(requests.try_recv().is_err());
    drop(stream);
    clock_guard.abort();
    rest_server.abort();
    stream_server.abort();
}

#[tokio::test]
async fn different_tokens_share_twenty_stream_leases_and_the_next_is_unsent() {
    let (url, mut accepts, server) = loopback().await;
    let owner = Admission::new();
    let clients = [
        client(&url, "token-a", owner.clone()),
        client(&url, "token-b", owner),
    ];
    let mut streams = Vec::new();
    for index in 0..20 {
        if index > 0 {
            advance_gap().await;
        }
        streams.push(
            clients[index % 2]
                .stream_transactions(&account())
                .await
                .expect("stream"),
        );
        accepts.recv().await.expect("accepted stream");
    }
    advance_gap().await;
    assert!(matches!(
        clients[1].stream_transactions(&account()).await,
        Err(Error::RateLimited)
    ));
    assert!(accepts.try_recv().is_err());
    drop(streams.remove(0));
    advance_gap().await;
    let replacement = clients[1]
        .stream_transactions(&account())
        .await
        .expect("released slot");
    accepts.recv().await.expect("replacement accepted");
    assert!(accepts.try_recv().is_err());
    drop(replacement);
    drop(streams);
    server.abort();
}

#[tokio::test(start_paused = true)]
async fn independent_egress_owners_do_not_share_connector_cooldown() {
    let clock_guard = keep_clock_frozen();
    let (url, mut accepts, server) = loopback().await;
    let first = client(&url, "token-a", Admission::new());
    let second = client(&url, "token-b", Admission::new());
    let start = Instant::now();
    let (left, right) = tokio::join!(first.list_accounts(), second.list_accounts());
    left.expect("first account list");
    right.expect("second account list");
    let left_at = accepts.recv().await.expect("first accept");
    let right_at = accepts.recv().await.expect("second accept");
    assert_eq!(left_at, start);
    assert_eq!(right_at, start);
    assert_eq!(Instant::now(), start);
    clock_guard.abort();
    server.abort();
}

#[tokio::test]
async fn cancelled_stream_open_releases_shared_slot_for_another_credential() {
    let (url, mut accepts, server) = stall_first_loopback().await;
    let owner = Admission::new();
    let first = client(&url, "token-a", owner.clone());
    let second = client(&url, "token-b", owner);
    let account_id = account();
    let opening = tokio::spawn(async move { first.stream_transactions(&account_id).await });
    accepts.recv().await.expect("first accepted");
    opening.abort();
    let Err(cancelled) = opening.await else {
        panic!("stream opening must be aborted");
    };
    assert!(cancelled.is_cancelled());
    let mut held = Vec::new();
    for _ in 0..19 {
        advance_gap().await;
        held.push(
            second
                .stream_transactions(&account())
                .await
                .expect("another credential stream"),
        );
        accepts.recv().await.expect("held stream accepted");
    }
    advance_gap().await;
    let replacement = second
        .stream_transactions(&account())
        .await
        .expect("replacement stream");
    accepts.recv().await.expect("replacement accepted");
    drop(replacement);
    drop(held);
    server.abort();
}

#[tokio::test]
async fn provider_cooldown_from_one_token_refuses_another_tokens_mutation_without_a_send() {
    let (url, server) = rate_limit_loopback().await;
    let owner = Admission::new();
    let first = client(&url, "token-a", owner.clone());
    let second = client(&url, "token-b", owner);
    let order = OrderSpecifier::new("fixture-order").expect("order");
    let first_result = first.cancel_order(&account(), &order, None).await;
    assert!(first_result.is_err());
    server.await.expect("one provider rejection");
    assert!(matches!(
        second.cancel_order(&account(), &order, None).await,
        Err(OperationError::Client(Error::RateLimited))
    ));
}

#[test]
fn explicit_builder_reserve_must_agree_with_shared_owner_without_resetting_it() {
    let owner = Admission::with_exit_reserve(7).expect("reserve");
    let preserved = Client::builder(Environment::Practice, "token-a")
        .admission(owner.clone())
        .build();
    assert!(preserved.is_ok());
    let matching = Client::builder(Environment::Practice, "token-b")
        .admission(owner.clone())
        .exit_reserve(7)
        .build();
    assert!(matching.is_ok());
    let conflict = Client::builder(Environment::Practice, "token-c")
        .admission(owner.clone())
        .exit_reserve(8)
        .build();
    assert!(matches!(conflict, Err(Error::InvalidInput(_))));
    assert!(
        Client::builder(Environment::Practice, "token-d")
            .admission(owner)
            .exit_reserve(7)
            .build()
            .is_ok()
    );
}

#[test]
fn largest_exit_reserve_is_accepted_by_owner_builder_and_runtime_setter() {
    let owner = Admission::with_exit_reserve(119).expect("owner reserve");
    owner.set_exit_reserve(119).expect("owner setter");
    assert!(matches!(
        owner.set_exit_reserve(120),
        Err(Error::InvalidInput(_))
    ));
    let client = Client::builder(Environment::Practice, "token-a")
        .admission(owner)
        .exit_reserve(119)
        .build()
        .expect("builder reserve");
    client.set_exit_reserve(119).expect("client setter");
    assert!(matches!(
        Admission::with_exit_reserve(120),
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        client.set_exit_reserve(120),
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        Client::builder(Environment::Practice, "token-b")
            .exit_reserve(120)
            .build(),
        Err(Error::InvalidInput(_))
    ));
}

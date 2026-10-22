//! Fixture clients for the integration tests.
use oanda_client::{Admission, Client, Environment};
use std::time::Duration;
use url::Url;

/// A Practice client with a fixture token, pointed at one loopback server.
pub fn fixture_client(url: Url) -> Client {
    Client::builder(Environment::Practice, "fixture-secret")
        .endpoints(url.clone(), url)
        .timeout(Duration::from_secs(2))
        .build()
        .expect("fixture client")
}

/// A fixture client with caller-selected credentials and shared admission.
pub fn fixture_client_with(url: &Url, token: &str, owner: Admission) -> Client {
    Client::builder(Environment::Practice, token)
        .admission(owner)
        .endpoints(url.clone(), url.clone())
        .build()
        .expect("fixture admission client")
}

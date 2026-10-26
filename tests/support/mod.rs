// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Loopback helpers shared by the integration tests.
//!
//! Each test file compiles this module into its own crate and uses a subset of it.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]
#![allow(
    dead_code,
    reason = "each integration test compiles and uses its own subset of this module"
)]
#![allow(
    unused_imports,
    reason = "each integration test uses only a subset of the re-exported helpers"
)]

mod client;
mod clock;
mod loopback;
mod server;
mod stream;

pub use client::{fixture_client, fixture_client_with};
pub use clock::keep_clock_frozen;
pub use loopback::{
    counting_rest_loopback, loopback, rate_limit_loopback, serve_socket, stall_first_loopback,
};
pub use server::{http, keepalive_server, read_request, server, servers};
pub use stream::{HeldStream, ParkedOpen, held_stream, parked_open};

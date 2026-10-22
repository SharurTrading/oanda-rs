//! Tokio time helpers for the integration tests.

/// Keep paused Tokio time stationary while loopback I/O completes.
pub fn keep_clock_frozen() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        loop {
            tokio::task::yield_now().await;
        }
    })
}

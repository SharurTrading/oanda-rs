//! Public-safe client errors.

/// Result returned by client operations.
pub type Result<T> = std::result::Result<T, Error>;

/// An endpoint error with its documented rejection body when available.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OperationError<R> {
    /// Client-side, transport, decoding, or ambiguous outcome.
    #[error(transparent)]
    Client(#[from] Error),
    /// A definitive provider rejection.
    #[error("OANDA rejected the operation with HTTP {status}: {message}")]
    Rejected {
        /// HTTP status.
        status: u16,
        /// Optional provider error code.
        code: Option<String>,
        /// Provider error message.
        message: String,
        /// Typed documented rejection fields, if the provider supplied them.
        body: Option<R>,
    },
}

/// Common OANDA error fields on endpoints without a specific rejection schema.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericRejection {
    /// Optional provider error code.
    pub error_code: Option<String>,
    /// Optional provider error message.
    pub error_message: Option<String>,
}

/// An OANDA client failure.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Local input or environment is invalid.
    #[error("invalid client input: {0}")]
    InvalidInput(String),
    /// Local query or stream capacity was exhausted.
    #[error("OANDA request rate capacity exhausted")]
    RateLimited,
    /// Provider rate limited a request.
    #[error("OANDA returned HTTP 429")]
    ProviderRateLimited,
    /// The provider rejected the request definitively.
    #[error("OANDA returned HTTP {status}: {code:?}: {message}")]
    Provider {
        /// HTTP response status.
        status: u16,
        /// Optional provider error code.
        code: Option<String>,
        /// Provider error message.
        message: String,
    },
    /// An I/O or decode outcome leaves a mutation uncertain.
    #[error("OANDA mutation outcome is ambiguous for account {account}")]
    AmbiguousMutation {
        /// Affected account identifier.
        account: String,
    },
    /// This account requires explicit reconciliation before another mutation.
    #[error("OANDA account {account} requires reconciliation")]
    ReconciliationRequired {
        /// Affected account identifier.
        account: String,
    },
    /// Another mutation for this account is still in flight.
    #[error("OANDA account {account} has a mutation in flight")]
    MutationInFlight {
        /// Affected account identifier.
        account: String,
    },
    /// Transport failed before a response was available.
    #[error("OANDA transport failed: {0}")]
    Transport(String),
    /// The response or stream record exceeds a configured bound.
    #[error("OANDA response exceeds the configured size limit")]
    ResponseTooLarge,
    /// Provider response did not match the documented shape.
    #[error("malformed OANDA response: {0}")]
    Decode(String),
}

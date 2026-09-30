//! Public-safe client errors.

/// Result returned by client operations.
pub type Result<T> = std::result::Result<T, Error>;

/// What OANDA actually supplied for an optional contract field.
///
/// A provider that sent nothing, a value this client decoded, and a value this
/// client could not decode are three different facts. Collapsing the last two
/// into absence would report evidence OANDA did send as evidence it never sent,
/// so a caller reconciling a rejected mutation could not tell a body it has
/// already accounted for from a body it is missing.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Supplied<T> {
    /// OANDA sent no value for this field. A body (or field) that arrived
    /// empty — only whitespace — is the same fact: nothing was sent to read.
    Absent,
    /// OANDA sent a value that matched the documented shape.
    Decoded(T),
    /// OANDA sent a non-empty value that did not match the documented shape.
    Undecoded,
}

impl<T> Supplied<T> {
    /// Whether OANDA sent no value at all.
    #[must_use]
    pub fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
    /// Whether OANDA sent a value this client could not decode.
    #[must_use]
    pub fn is_undecoded(&self) -> bool {
        matches!(self, Self::Undecoded)
    }
    /// The decoded value, when OANDA sent one this client could read.
    pub fn decoded(&self) -> Option<&T> {
        match self {
            Self::Decoded(value) => Some(value),
            Self::Absent | Self::Undecoded => None,
        }
    }
}

/// Render a provider reason without ever speaking for OANDA.
///
/// Both fields are `None` exactly when the provider supplied no readable
/// reason; the rendering then says so rather than asserting one.
fn describe_reason(code: Option<&str>, message: Option<&str>) -> String {
    match (code, message) {
        (Some(code), Some(message)) => format!("{code}: {message}"),
        (Some(code), None) => format!("{code} (OANDA sent no error message)"),
        (None, Some(message)) => message.to_owned(),
        (None, None) => "OANDA sent no readable reason".to_owned(),
    }
}

/// An endpoint error with its documented rejection body when available.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OperationError<R> {
    /// Client-side, transport, decoding, or ambiguous outcome.
    #[error(transparent)]
    Client(#[from] Error),
    /// A definitive provider rejection.
    #[error(
        "OANDA rejected the operation with HTTP {status}: {}",
        describe_reason(.code.as_deref(), .message.as_deref())
    )]
    Rejected {
        /// HTTP status.
        status: u16,
        /// OANDA's `errorCode`, when it supplied a readable one.
        code: Option<String>,
        /// OANDA's `errorMessage`, when it supplied a readable one.
        message: Option<String>,
        /// The endpoint's documented rejection fields, or what OANDA actually sent for them.
        body: Supplied<R>,
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
    #[error(
        "OANDA returned HTTP {status}: {}",
        describe_reason(.code.as_deref(), .message.as_deref())
    )]
    Provider {
        /// HTTP response status.
        status: u16,
        /// OANDA's `errorCode`, when it supplied a readable one.
        code: Option<String>,
        /// OANDA's `errorMessage`, when it supplied a readable one.
        message: Option<String>,
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

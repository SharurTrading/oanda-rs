// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

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

/// Render a redirect target without ever guessing one.
fn describe_target(location: &Supplied<crate::RedirectTarget>) -> String {
    match location {
        Supplied::Decoded(target) => format!("to {target}"),
        Supplied::Absent => "without a Location".to_owned(),
        Supplied::Undecoded => "with an unreadable Location".to_owned(),
    }
}

/// An endpoint error with its documented rejection body when available.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OperationError<R> {
    /// Client-side, transport, decoding, or ambiguous outcome.
    #[error(transparent)]
    Client(#[from] Error),
    /// A definitive rejection whose body could not be read within local transport limits.
    ///
    /// The observed status remains authoritative and the account is not fenced.
    #[error("OANDA rejected the operation with HTTP {status}; rejection body unreadable: {source}")]
    RejectedUnreadable {
        /// Definitive provider rejection status.
        status: u16,
        /// Provider request identity observed before the body failed.
        request_id: Supplied<crate::RequestID>,
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
        /// Body transport failure or local size bound.
        #[source]
        source: Error,
    },
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
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
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
    /// The client's internal admission state is inconsistent.
    #[error("OANDA client invariant failed: {0}")]
    Invariant(&'static str),
    /// Local input or environment is invalid.
    #[error("invalid client input: {0}")]
    InvalidInput(String),
    /// Local query or stream capacity was exhausted.
    #[error("OANDA request rate capacity exhausted")]
    RateLimited,
    /// Provider rate limited a request with HTTP 429.
    #[error("OANDA returned HTTP 429")]
    ProviderRateLimited {
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
    },
    /// HTTP 429 proves refusal, but the shared cooldown could not be installed.
    #[error("OANDA returned HTTP 429; cooldown invariant failed: {source}")]
    ProviderRateLimitedInvariant {
        /// The admission invariant failure, retained alongside the definitive status.
        #[source]
        source: Box<Error>,
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
    },
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
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
    },
    /// The answer was an HTTP redirect (300-399), which this client never follows.
    ///
    /// No request is sent to the target, so the bearer token never leaves the
    /// selected environment's hosts. A redirected mutation was not executed at the
    /// requested URL, so it does not fence the account.
    #[error("OANDA answered with HTTP {status} redirect {}; it was not followed", describe_target(.location))]
    Redirected {
        /// HTTP response status.
        status: u16,
        /// The `Location` target without userinfo, query or fragment: `Absent`
        /// when no `Location` was sent, `Undecoded` when one was sent that does
        /// not resolve to a URL with a host.
        location: Supplied<crate::RedirectTarget>,
        /// OANDA's `RequestID` response header as supplied evidence.
        request_id: Supplied<crate::RequestID>,
        /// Diagnostic headers that say which network layer answered.
        diagnostics: Box<crate::ResponseDiagnostics>,
    },
    /// An I/O or decode outcome leaves a mutation uncertain.
    #[error("OANDA mutation outcome is ambiguous for account {account}")]
    AmbiguousMutation {
        /// Affected account identifier.
        account: crate::AccountID,
        /// HTTP response status, or `None` when no response arrived.
        status: Option<u16>,
        /// Diagnostic headers that say which network layer answered. Every
        /// field is `Absent` when no response arrived.
        diagnostics: Box<crate::ResponseDiagnostics>,
    },
    /// This account requires explicit reconciliation before another mutation.
    #[error("OANDA account {account} requires reconciliation")]
    ReconciliationRequired {
        /// Affected account identifier.
        account: crate::AccountID,
    },
    /// Another mutation for this account is still in flight.
    #[error("OANDA account {account} has a mutation in flight")]
    MutationInFlight {
        /// Affected account identifier.
        account: crate::AccountID,
    },
    /// Transport failed before a response was available.
    #[error("OANDA transport failed: {0}")]
    Transport(String),
    /// The response or stream record exceeds a configured bound.
    #[error("OANDA response exceeds the configured size limit")]
    ResponseTooLarge,
    /// A finite buffered cut ended inside an unterminated stream record.
    #[error("OANDA buffered stream ends with {bytes} unterminated bytes")]
    IncompleteStreamRecord {
        /// Bytes remaining after the last complete newline-delimited record.
        bytes: usize,
    },
    /// The stream had ended before a buffered cut could be established.
    #[error("OANDA stream already ended before its buffered prefix was closed")]
    StreamAlreadyEnded,
    /// Provider response did not match the documented shape.
    #[error("malformed OANDA response: {0}")]
    Decode(String),
}

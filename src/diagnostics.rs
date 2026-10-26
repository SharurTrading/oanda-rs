// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Response headers that say which layer of the network answered a request.

use crate::Supplied;
use reqwest::header::{HeaderMap, SERVER};

/// Diagnostic response headers carried on a non-success answer.
///
/// An answer can come from OANDA's own servers or from an edge network in front
/// of them, such as a content-delivery or bot-management layer. These headers
/// let a caller tell the two apart. They are reported exactly as received and
/// nothing in this client acts on them. Cookies, `Set-Cookie` and other headers
/// are never read into this value.
///
/// Each field is `Absent` when the header was not sent and `Undecoded` when it
/// was sent with bytes that are not visible ASCII text.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseDiagnostics {
    /// The `Server` header, naming the software or network that answered.
    pub server: Supplied<String>,
    /// The `CF-RAY` header, an edge network's identifier for this request.
    pub ray_id: Supplied<String>,
    /// The `cf-mitigated` header, which an edge network sets when it answered
    /// with a challenge instead of passing the request on.
    pub mitigated: Supplied<String>,
}

impl ResponseDiagnostics {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Self {
        Self {
            server: text_header(headers, SERVER.as_str()),
            ray_id: text_header(headers, "cf-ray"),
            mitigated: text_header(headers, "cf-mitigated"),
        }
    }
}

impl Default for ResponseDiagnostics {
    /// No diagnostic header was received.
    fn default() -> Self {
        Self {
            server: Supplied::Absent,
            ray_id: Supplied::Absent,
            mitigated: Supplied::Absent,
        }
    }
}

fn text_header(headers: &HeaderMap, name: &str) -> Supplied<String> {
    match headers.get(name) {
        None => Supplied::Absent,
        Some(value) => match value.to_str() {
            Ok(text) => Supplied::Decoded(text.to_owned()),
            Err(_) => Supplied::Undecoded,
        },
    }
}

#[cfg(test)]
// Fixture failures are clearer as panics; production paths return evidence.
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderValue, SET_COOKIE};

    #[test]
    fn present_headers_are_read_as_received() {
        let mut headers = HeaderMap::new();
        headers.insert(SERVER, HeaderValue::from_static("cloudflare"));
        headers.insert("CF-RAY", HeaderValue::from_static("8f00000000000000-SYD"));
        headers.insert("cf-mitigated", HeaderValue::from_static("challenge"));
        headers.insert(SET_COOKIE, HeaderValue::from_static("__cf_bm=cookie-value"));
        let diagnostics = ResponseDiagnostics::from_headers(&headers);
        assert_eq!(
            diagnostics.server,
            Supplied::Decoded("cloudflare".to_owned())
        );
        assert_eq!(
            diagnostics.ray_id,
            Supplied::Decoded("8f00000000000000-SYD".to_owned())
        );
        assert_eq!(
            diagnostics.mitigated,
            Supplied::Decoded("challenge".to_owned())
        );
        assert!(!format!("{diagnostics:?}").contains("cookie-value"));
    }

    #[test]
    fn missing_headers_are_absent_and_unreadable_ones_undecoded() {
        let mut headers = HeaderMap::new();
        headers.insert(
            SERVER,
            HeaderValue::from_bytes(b"edge\xff").expect("opaque header"),
        );
        let diagnostics = ResponseDiagnostics::from_headers(&headers);
        assert_eq!(diagnostics.server, Supplied::Undecoded);
        assert_eq!(diagnostics.ray_id, Supplied::Absent);
        assert_eq!(diagnostics.mitigated, Supplied::Absent);
        assert_eq!(
            ResponseDiagnostics::from_headers(&HeaderMap::new()),
            ResponseDiagnostics::default()
        );
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The reduced target of an HTTP redirect this client refused to follow.

use crate::Supplied;
use reqwest::{Url, header::HeaderMap};
use std::fmt;

/// Where an HTTP 3xx answer pointed, reduced to its scheme, host, port and path.
///
/// This client never follows a redirect, so the bearer token is only ever sent to
/// the selected environment's own hosts. The target is kept so a caller can see
/// where the answer pointed, without carrying anything that could hold a secret:
/// the userinfo, query and fragment of the `Location` value are discarded when it
/// is read and are never stored, so neither `Debug` nor `Display` can render them.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RedirectTarget(Box<Parts>);

/// Boxed so the error variant carrying a target stays small.
#[derive(Clone, PartialEq, Eq, Hash)]
struct Parts {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
}

impl RedirectTarget {
    /// Read a response's `Location` header as supplied evidence.
    ///
    /// A relative reference is resolved against the request URL, as HTTP defines.
    /// No header is `Absent`; a header that is not text, does not resolve to a URL,
    /// or names no host is `Undecoded`, because a target was sent and cannot be
    /// stated.
    pub(crate) fn from_headers(request: &Url, headers: &HeaderMap) -> Supplied<Self> {
        let Some(value) = headers.get(reqwest::header::LOCATION) else {
            return Supplied::Absent;
        };
        let Ok(text) = value.to_str() else {
            return Supplied::Undecoded;
        };
        let Ok(url) = request.join(text.trim()) else {
            return Supplied::Undecoded;
        };
        let Some(host) = url.host_str() else {
            return Supplied::Undecoded;
        };
        Supplied::Decoded(Self(Box::new(Parts {
            scheme: url.scheme().to_owned(),
            host: host.to_owned(),
            port: url.port(),
            path: url.path().to_owned(),
        })))
    }

    /// The URL scheme, lowercase, such as `https`.
    #[must_use]
    pub fn scheme(&self) -> &str {
        &self.0.scheme
    }

    /// The host name or address literal; an IPv6 address keeps its brackets.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.0.host
    }

    /// The port, when the target named one other than its scheme's default.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.0.port
    }

    /// The percent-encoded path, which is `/` for a target that names none.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.0.path
    }
}

impl fmt::Debug for RedirectTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedirectTarget")
            .field("scheme", &self.0.scheme)
            .field("host", &self.0.host)
            .field("port", &self.0.port)
            .field("path", &self.0.path)
            .finish()
    }
}

impl fmt::Display for RedirectTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.0.scheme, self.0.host)?;
        if let Some(port) = self.0.port {
            write!(f, ":{port}")?;
        }
        f.write_str(&self.0.path)
    }
}

#[cfg(test)]
// Fixture failures are clearer as panics; production paths return evidence.
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderValue, LOCATION};

    fn read(location: Option<HeaderValue>) -> Supplied<RedirectTarget> {
        let request =
            Url::parse("https://api-fxpractice.oanda.com/v3/accounts?instruments=EUR_USD")
                .expect("request URL");
        let mut headers = HeaderMap::new();
        if let Some(value) = location {
            headers.insert(LOCATION, value);
        }
        RedirectTarget::from_headers(&request, &headers)
    }

    fn decoded(location: &str) -> RedirectTarget {
        match read(Some(HeaderValue::from_str(location).expect("header"))) {
            Supplied::Decoded(target) => target,
            other => panic!("expected a decoded target for {location}: {other:?}"),
        }
    }

    #[test]
    fn an_absolute_target_keeps_only_scheme_host_port_and_path() {
        let target = decoded("https://user:secret@edge.example:8443/a/b?token=abc#frag");
        assert_eq!(target.scheme(), "https");
        assert_eq!(target.host(), "edge.example");
        assert_eq!(target.port(), Some(8443));
        assert_eq!(target.path(), "/a/b");
        assert_eq!(target.to_string(), "https://edge.example:8443/a/b");
        let debug = format!("{target:?}");
        for hidden in ["user", "secret", "token", "abc", "frag", "?", "#"] {
            assert!(!debug.contains(hidden), "{hidden} leaked into {debug}");
        }
    }

    #[test]
    fn a_default_port_is_not_rendered() {
        let target = decoded("https://edge.example:443/cdn-cgi/challenge");
        assert_eq!(target.port(), None);
        assert_eq!(target.to_string(), "https://edge.example/cdn-cgi/challenge");
    }

    #[test]
    fn a_relative_target_resolves_against_the_request_without_its_query() {
        let target = decoded("/v3/elsewhere?x=1");
        assert_eq!(
            target.to_string(),
            "https://api-fxpractice.oanda.com/v3/elsewhere"
        );
    }

    #[test]
    fn a_missing_location_is_absent() {
        assert_eq!(read(None), Supplied::Absent);
    }

    #[test]
    fn an_unreadable_or_hostless_location_is_undecoded() {
        let opaque = HeaderValue::from_bytes(b"https://edge.example/\xff").expect("opaque header");
        assert_eq!(read(Some(opaque)), Supplied::Undecoded);
        let hostless = HeaderValue::from_static("mailto:ops@example.com");
        assert_eq!(read(Some(hostless)), Supplied::Undecoded);
        let invalid = HeaderValue::from_static("https://");
        assert_eq!(read(Some(invalid)), Supplied::Undecoded);
    }
}

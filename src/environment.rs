//! Paired REST and streaming environments.

use crate::{Error, Result};
use url::Url;

/// OANDA account environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// fxTrade Practice.
    Practice,
    /// fxTrade Live.
    Live,
}

impl Environment {
    pub(crate) fn urls(self) -> Result<(Url, Url)> {
        let (rest, stream) = match self {
            Self::Practice => (
                "https://api-fxpractice.oanda.com",
                "https://stream-fxpractice.oanda.com",
            ),
            Self::Live => (
                "https://api-fxtrade.oanda.com",
                "https://stream-fxtrade.oanda.com",
            ),
        };
        Ok((
            Url::parse(rest).map_err(|e| Error::InvalidInput(e.to_string()))?,
            Url::parse(stream).map_err(|e| Error::InvalidInput(e.to_string()))?,
        ))
    }
}

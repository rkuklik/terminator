use std::env;
use std::ffi::OsStr;

use crate::consts::BACKTRACE;
use crate::consts::LIB_BACKTRACE;

/// Setting for backtrace details
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum Verbosity {
    /// No backtrace will be collected
    #[default]
    Minimal,
    /// Backtrace will be filtered (influenced by registered [`FrameFilter`](crate::FrameFilter)s)
    Medium,
    /// Backtrace will be shown in full
    Full,
}

impl Verbosity {
    const fn decode(thing: &[u8]) -> Self {
        #[allow(clippy::match_same_arms)]
        match thing {
            b"0" => Verbosity::Minimal,
            b"1" => Verbosity::Medium,
            b"full" => Verbosity::Full,
            _ => Verbosity::Medium,
        }
    }

    /// Retrieves [`Verbosity`] that should be used by errors (based on environment variables)
    #[must_use]
    pub fn error() -> Option<Self> {
        env::var_os(LIB_BACKTRACE)
            .or_else(|| env::var_os(BACKTRACE))
            .as_deref()
            .map(OsStr::as_encoded_bytes)
            .map(Self::decode)
    }

    /// Retrieves [`Verbosity`] that should be used by panics (based on environment variables)
    #[must_use]
    pub fn panic() -> Option<Self> {
        env::var_os(BACKTRACE)
            .as_deref()
            .map(OsStr::as_encoded_bytes)
            .map(Self::decode)
    }

    /// Shows environment name corresponding to provided [`Verbosity`]
    #[must_use]
    #[inline]
    pub const fn env(self) -> &'static str {
        match self {
            Self::Minimal => "0",
            Self::Medium => "1",
            Self::Full => "full",
        }
    }
}

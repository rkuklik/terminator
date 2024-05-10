use std::env;
use std::ffi::OsStr;

use crate::consts::BACKTRACE;
use crate::consts::LIB_BACKTRACE;

/// Setting for backtrace details
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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
    fn decode(thing: &[u8]) -> Option<Self> {
        Some(match thing {
            b"0" => Verbosity::Minimal,
            b"1" => Verbosity::Medium,
            b"full" => Verbosity::Full,
            _ => return None,
        })
    }

    /// Retrieves [`Verbosity`] that should be used by errors (based on environment variables)
    pub fn error() -> Option<Self> {
        env::var_os(LIB_BACKTRACE)
            .or_else(|| env::var_os(BACKTRACE))
            .as_deref()
            .map(OsStr::as_encoded_bytes)
            .and_then(Self::decode)
    }

    /// Retrieves [`Verbosity`] that should be used by panics (based on environment variables)
    pub fn panic() -> Option<Self> {
        env::var_os(BACKTRACE)
            .as_deref()
            .map(OsStr::as_encoded_bytes)
            .and_then(Self::decode)
    }

    pub(crate) fn env(self) -> &'static str {
        match self {
            Self::Minimal => "0",
            Self::Medium => "1",
            Self::Full => "full",
        }
    }
}

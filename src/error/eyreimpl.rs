use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use eyre::EyreHandler;

use crate::Config;
use crate::Verbosity;

pub struct BacktraceHandler {
    backtrace: Backtrace,
}

impl BacktraceHandler {
    pub fn configured(config: &Config) -> Self {
        Self {
            backtrace: (config.error == Verbosity::Minimal)
                .then(Backtrace::disabled)
                .unwrap_or_else(Backtrace::force_capture),
        }
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl EyreHandler for BacktraceHandler {
    fn debug(&self, error: &(dyn Error + 'static), f: &mut Formatter<'_>) -> Result {
        Debug::fmt(error, f)
    }

    fn display(&self, error: &(dyn Error + 'static), f: &mut Formatter<'_>) -> Result {
        Display::fmt(error, f)
    }
}

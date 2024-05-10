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
    backtrace: Option<Backtrace>,
}

type Handler = dyn Fn(&(dyn Error + 'static)) -> Box<dyn EyreHandler> + Sync + Send + 'static;

impl Config {
    pub(crate) fn eyre_hook(&'static self) -> Box<Handler> {
        Box::new(move |_| {
            Box::new(BacktraceHandler {
                backtrace: (self.error != Verbosity::Minimal).then(Backtrace::force_capture),
            })
        })
    }
}

impl BacktraceHandler {
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
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

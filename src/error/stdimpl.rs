use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::Config;
use crate::Verbosity;
use crate::GLOBAL_SETTINGS;

struct Chain<'a> {
    next: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Chain<'a> {
    pub fn new(head: &'a (dyn Error + 'static)) -> Self {
        Chain { next: Some(head) }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn Error + 'static);
    fn next(&mut self) -> Option<Self::Item> {
        let yielded = self.next?;
        self.next = yielded.source();
        Some(yielded)
    }
}

// HACK: can't be declared directly, see three lines below. In order to not misuse
// the helper, implement traits and methods directly on this (ie. no generics).
pub type DynError = ErrorUnsizingHelper<dyn Error + 'static>;

// NOTE: `DynError` has to have generic helper (not directly error = dyn Error + 'static),
// because it is almost impossible to create instance of unsized structs directly.
pub struct ErrorUnsizingHelper<E: ?Sized> {
    backtrace: Backtrace,
    error: E,
}

impl DynError {
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl DynError {
    pub fn chain(&self) -> impl Iterator<Item = &(dyn Error + 'static)> {
        Chain::new(&self.error)
    }
}

impl Debug for DynError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.error, f)
    }
}

impl Display for DynError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.error, f)
    }
}

impl Error for DynError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<E> From<E> for Box<DynError>
where
    E: Error + 'static,
{
    fn from(value: E) -> Self {
        let backtrace = GLOBAL_SETTINGS
            .get()
            .map(Config::selected_verbosity)
            .and_then(|verbosity| (verbosity == Verbosity::Minimal).then(Backtrace::disabled))
            .unwrap_or_else(Backtrace::force_capture);
        let error = ErrorUnsizingHelper {
            backtrace,
            error: value,
        };
        Box::new(error) as Self
    }
}

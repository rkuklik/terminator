use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

use owo_colors::OwoColorize;

use crate::indent::Indent;
use crate::Config;
use crate::GLOBAL_SETTINGS;

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
struct Chain<'a> {
    next: Option<&'a (dyn Error + 'static)>,
}

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
impl<'a> Chain<'a> {
    pub fn new(head: &'a (dyn Error + 'static)) -> Self {
        Chain { next: Some(head) }
    }
}

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn Error + 'static);
    fn next(&mut self) -> Option<Self::Item> {
        let yielded = self.next?;
        self.next = yielded.source();
        Some(yielded)
    }
}

/// Why not use this in main function as `Error` value? It's so pretty :)
pub struct Terminator {
    #[cfg(not(any(feature = "anyhow", feature = "eyre")))]
    inner: Box<dyn Error>,
    #[cfg(feature = "anyhow")]
    inner: anyhow::Error,
    #[cfg(feature = "eyre")]
    inner: eyre::Report,
}

impl Debug for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            return Debug::fmt(&self.inner, f);
        }

        let config = GLOBAL_SETTINGS.get_or_init(Config::new);

        #[cfg(any(feature = "anyhow", feature = "eyre"))]
        let errors = self.inner.chain();
        #[cfg(not(any(feature = "anyhow", feature = "eyre")))]
        let errors = Chain::new(&*self.inner as &dyn Error);

        for (index, error) in errors.enumerate() {
            write!(f, "\n{:>4}: {}", index, error.style(config.theme.error))?;
        }

        #[cfg(feature = "anyhow")]
        let backtrace = self.inner.backtrace();
        #[cfg(not(feature = "anyhow"))]
        let backtrace = if config.selected_verbosity() == crate::verbosity::Verbosity::Minimal {
            std::backtrace::Backtrace::disabled()
        } else {
            std::backtrace::Backtrace::force_capture()
        };
        #[cfg(not(feature = "anyhow"))]
        let backtrace = &backtrace;

        write!(Indent::double(f), "\n\n{}", config.bundle(backtrace))
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Error: {self:?}")
    }
}

#[cfg(feature = "anyhow")]
impl<E> From<E> for Terminator
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(feature = "eyre")]
impl<E> From<E> for Terminator
where
    E: Into<eyre::Report>,
{
    fn from(value: E) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
impl<E> From<E> for Terminator
where
    E: Error + 'static,
{
    fn from(value: E) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

/// Error for when [`Config`] had already been installed
#[derive(Debug)]
pub struct InstallError;

impl Display for InstallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "`Config` was already installed globally")
    }
}

impl Error for InstallError {}

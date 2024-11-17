use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

use crate::indent::Indent;
use crate::Config;
use crate::GLOBAL_SETTINGS;

#[cfg(feature = "eyre")]
mod eyreimpl;
#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
mod stdimpl;

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
type Inner = Box<stdimpl::DynError>;
#[cfg(feature = "anyhow")]
type Inner = anyhow::Error;
#[cfg(feature = "eyre")]
type Inner = eyre::Report;

/// Why not use this in main function as `Error` value? It's so pretty :)
pub struct Terminator {
    inner: Inner,
}

impl Terminator {
    fn new(inner: Inner) -> Self {
        Self { inner }
    }

    fn chain(&self) -> impl Iterator<Item = &(dyn Error + 'static)> {
        self.inner.chain()
    }

    #[cfg(not(any(feature = "anyhow", feature = "eyre")))]
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }

    #[cfg(feature = "anyhow")]
    fn backtrace(&self) -> Option<&Backtrace> {
        let backtrace = self.inner.backtrace();
        if backtrace.status() == std::backtrace::BacktraceStatus::Captured {
            Some(backtrace)
        } else {
            None
        }
    }

    #[cfg(feature = "eyre")]
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner
            .handler()
            .downcast_ref::<eyreimpl::BacktraceHandler>()
            .and_then(eyreimpl::BacktraceHandler::backtrace)
    }
}

impl Debug for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            return Debug::fmt(&self.inner, f);
        }

        let config = GLOBAL_SETTINGS.get_or_init(Config::new);

        for (index, error) in self.chain().enumerate() {
            write!(f, "\n{:>4}: {}", index, config.theme.error.style(error))?;
        }

        write!(Indent::double(f), "\n\n{}", config.bundle(self.backtrace()))
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
    E: Into<Inner>,
{
    fn from(value: E) -> Self {
        Self::new(value.into())
    }
}

#[cfg(feature = "eyre")]
impl<E> From<E> for Terminator
where
    E: Into<Inner>,
{
    fn from(value: E) -> Self {
        Self::new(value.into())
    }
}

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
impl<E> From<E> for Terminator
where
    E: Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Self::new(value.into())
    }
}

/// Error for when [`Config`] had already been installed
#[derive(Debug)]
pub struct InstallError;

impl Display for InstallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("`Config` was already installed globally")
    }
}

impl Error for InstallError {}

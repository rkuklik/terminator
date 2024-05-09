use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;
use std::marker::PhantomData;

use owo_colors::OwoColorize;

use crate::indent::Indent;
use crate::Config;
use crate::GLOBAL_SETTINGS;

#[cfg(feature = "eyre")]
mod eyreimpl;
#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
mod stdimpl;

impl Config {
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn post_install(&'static self) -> std::result::Result<&'static Self, InstallError> {
        #[cfg(feature = "eyre")]
        {
            let constructor = eyreimpl::BacktraceHandler::configured;
            eyre::set_hook(Box::new(move |_| Box::new(constructor(self))))
                .map_err(|_| InstallError)?;
        }
        Ok(self)
    }
}

#[cfg(not(any(feature = "anyhow", feature = "eyre")))]
type Inner = Box<stdimpl::DynError>;
#[cfg(feature = "anyhow")]
type Inner = anyhow::Error;
#[cfg(feature = "eyre")]
type Inner = eyre::Report;

/// Why not use this in main function as `Error` value? It's so pretty :)
pub struct Terminator {
    inner: Inner,
    // force `Terminator` to not be `Send` and `Sync` (though this may be lifted)
    phantom: PhantomData<*const ()>,
}

impl Terminator {
    fn new(inner: Inner) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    fn chain(&self) -> impl Iterator<Item = &(dyn Error + 'static)> {
        self.inner.chain()
    }

    fn checked_capture(backtrace: &Backtrace) -> Option<&Backtrace> {
        if backtrace.status() == BacktraceStatus::Captured {
            Some(backtrace)
        } else {
            None
        }
    }

    #[cfg(not(feature = "eyre"))]
    fn backtrace(&self) -> Option<&Backtrace> {
        Self::checked_capture(self.inner.backtrace())
    }

    #[cfg(feature = "eyre")]
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.inner
            .handler()
            .downcast_ref::<eyreimpl::BacktraceHandler>()
            .map(eyreimpl::BacktraceHandler::backtrace)
            .and_then(Self::checked_capture)
    }
}

impl Debug for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            return Debug::fmt(&self.inner, f);
        }

        let config = GLOBAL_SETTINGS.get_or_init(Config::new);

        for (index, error) in self.chain().enumerate() {
            write!(f, "\n{:>4}: {}", index, error.style(config.theme.error))?;
        }

        write!(Indent::double(f), "\n\n{}", config.bundle(self.backtrace()))
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Error: {self:?}")
    }
}

#[cfg(any(feature = "anyhow", feature = "eyre"))]
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
        write!(f, "`Config` was already installed globally")
    }
}

impl Error for InstallError {}

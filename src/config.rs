use std::env;
use std::panic;

use crate::backtrace::FrameFilter;
use crate::consts::BACKTRACE;
use crate::consts::LIB_BACKTRACE;
use crate::theme::Theme;
use crate::Frame;
use crate::InstallError;
use crate::Verbosity;
use crate::GLOBAL_SETTINGS;

pub(crate) struct Bundle<'a, T> {
    pub config: &'a Config,
    pub data: T,
}

/// Configuration influencing appearance of displayed messages
#[must_use = "`Config` is useless unless used in panic hook or installed"]
pub struct Config {
    pub(crate) filters: Vec<Box<FrameFilter>>,
    pub(crate) theme: Theme,
    pub(crate) error: Verbosity,
    pub(crate) panic: Verbosity,
}

impl Config {
    pub(crate) fn bundle<T>(&self, data: T) -> Bundle<'_, T> {
        Bundle { config: self, data }
    }

    pub(crate) fn selected_verbosity(&self) -> Verbosity {
        if std::thread::panicking() {
            self.panic
        } else {
            self.error
        }
    }

    /// Creates new [`Config`] with no settings altered
    #[inline]
    pub fn blank() -> Self {
        Self {
            filters: Vec::new(),
            theme: Theme::new(),
            error: Verbosity::Minimal,
            panic: Verbosity::Minimal,
        }
    }

    /// Creates new [`Config`] with sane defaults applied
    ///
    /// This registers builtin [`FrameFilter`]s and retrieves [`Verbosity`]
    /// settings from environment.
    #[inline]
    pub fn new() -> Self {
        Self {
            filters: Frame::default_filters(),
            error: Verbosity::error().unwrap_or_default(),
            panic: Verbosity::panic().unwrap_or_default(),
            theme: Theme::new(),
        }
    }

    /// Sets [`Config`] and registers panic hook.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`Config`] is already installed.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn install(self) -> Result<&'static Self, InstallError> {
        env::set_var(BACKTRACE, self.panic.env());
        env::set_var(LIB_BACKTRACE, self.error.env());
        let config = GLOBAL_SETTINGS
            .set(self)
            .map_err(|_| InstallError)
            .map(|()| GLOBAL_SETTINGS.get().expect("`OnceLock` was just set"))?;
        panic::set_hook(Box::new(config.panic_hook()));
        #[cfg(feature = "eyre")]
        eyre::set_hook(config.eyre_hook()).map_err(|_| InstallError)?;
        Ok(config)
    }

    /// Set verbosity for panics
    #[inline]
    pub fn panic_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.panic = verbosity;
        self
    }

    /// Set verbosity for errors
    #[inline]
    pub fn error_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.error = verbosity;
        self
    }

    /// Set verbosity for both errors and panics
    #[inline]
    pub fn verbosity(mut self, verbosity: Verbosity) -> Self {
        self.error = verbosity;
        self.panic = verbosity;
        self
    }

    /// Add filter for backtrace filtering
    #[inline]
    pub fn filter(mut self, filter: Box<FrameFilter>) -> Self {
        self.filters.push(filter);
        self
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

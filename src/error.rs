use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

use owo_colors::OwoColorize;

use crate::Config;
use crate::Indent;
use crate::GLOBAL_SETTINGS;

/// It's so pretty :)
pub struct PrettyError {
    inner: anyhow::Error,
}

impl Debug for PrettyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            return Debug::fmt(&self.inner, f);
        }

        let config = GLOBAL_SETTINGS.get_or_init(Config::new);
        let errors = self.inner.chain();

        for (index, error) in errors.enumerate() {
            write!(f, "\n{:>4}: {}", index, error.style(config.theme.error))?;
        }

        write!(
            Indent::double(f),
            "\n\n{}",
            config.bundle(self.inner.backtrace())
        )
    }
}

impl Display for PrettyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Error: {self:?}")
    }
}

impl<E> From<E> for PrettyError
where
    E: Into<anyhow::Error>,
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

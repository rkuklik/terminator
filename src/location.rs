use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::panic;

use crate::config::Bundle;
use crate::consts::UNKNOWN;

/// Filename and line corresponding to source file
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Location<'a> {
    pub(crate) file: Cow<'a, str>,
    pub(crate) line: u32,
}

impl<'a> Location<'a> {
    pub(crate) fn derived(location: &'a panic::Location<'a>) -> Self {
        Self {
            file: Cow::Borrowed(location.file()),
            line: location.line(),
        }
    }

    /// Gets source file filename
    #[must_use]
    #[inline]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Gets line number
    #[must_use]
    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }
}

impl Display for Bundle<'_, &Location<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let theme = &self.config.theme;
        let location = self.data;
        let file = theme.file.style(location.file());
        let line = theme.line.style(location.line());
        write!(f, "{file}:{line}")
    }
}

impl Display for Bundle<'_, Option<&Location<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(location) = self.data {
            Display::fmt(&self.config.bundle(location), f)
        } else {
            f.write_str(UNKNOWN)
        }
    }
}

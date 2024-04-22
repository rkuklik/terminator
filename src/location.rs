use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::panic;

use owo_colors::OwoColorize;

use crate::config::Bundle;
use crate::consts::UNKNOWN;

/// Filename and line corresponding to source file
#[derive(Debug, Clone, Default)]
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

impl Display for Bundle<'_, &'_ Location<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let theme = self.config().theme;
        let Location { file, line } = *self.data();
        let file = file.style(theme.file);
        let line = line.style(theme.line);
        write!(f, "{file}:{line}")
    }
}

impl Display for Bundle<'_, Option<&'_ Location<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(location) = self.data() {
            write!(f, "{}", self.config().bundle(*location))
        } else {
            f.write_str(UNKNOWN)
        }
    }
}

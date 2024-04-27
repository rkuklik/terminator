use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use owo_colors::OwoColorize;

use crate::config::Bundle;
use crate::consts::UNKNOWN;
use crate::location::Location;

pub use filter::FrameFilter;

mod convert;
mod display;
mod filter;

/// Type to smuggle mutable vector to display impl and enable trait impls. One-time use.
struct Backtrace<'a> {
    frames: Cell<Vec<Frame<'a>>>,
}

/// Representation of single frame in backtrace
///
/// Mainly used to filter backtrace for unnecessary info.
#[derive(Debug, Clone)]
pub struct Frame<'a> {
    index: usize,
    name: Option<Cow<'a, str>>,
    location: Option<Location<'a>>,
}

impl Frame<'_> {
    /// Frame index
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Frame symbol name
    #[must_use]
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Frame location
    #[must_use]
    #[inline]
    pub fn location(&self) -> Option<&Location<'_>> {
        self.location.as_ref()
    }

    fn symbolify(&self) -> (&str, Option<&str>) {
        let Some(name) = self.name() else {
            return (UNKNOWN, None);
        };
        let condition = name.len() > 19
            && name[name.len() - 19..name.len() - 16].eq("::h")
            && name[name.len() - 16..]
                .chars()
                .all(|char| char.is_ascii_hexdigit());
        if condition {
            (&name[..(name.len() - 19)], Some(&name[(name.len() - 19)..]))
        } else {
            (name, None)
        }
    }
}

impl Display for Bundle<'_, &'_ Frame<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config();
        let frame = *self.data();
        let (name, hash) = frame.symbolify();

        #[allow(clippy::obfuscated_if_else)]
        let name = frame
            .is_dependency_code()
            .then_some(config.theme.dependency)
            .unwrap_or(config.theme.package)
            .style(name);
        let hash = hash.unwrap_or("");
        let hash = hash.style(config.theme.hash);
        let location = config.bundle(frame.location.as_ref());

        write!(f, "{:>2}: {name}{hash}\n    at {location}", frame.index)
    }
}

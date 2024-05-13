use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::config::Bundle;
use crate::consts::UNKNOWN;
use crate::location::Location;

pub use filter::FrameFilter;

mod convert;
mod display;
mod filter;

/// Type to smuggle mutable vector to display impl and enable trait impls. One-time use.
#[derive(Default)]
struct Backtrace<'a> {
    frames: Cell<Vec<Frame<'a>>>,
}

/// Representation of single frame in backtrace
///
/// Mainly used to filter backtrace for unnecessary info.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    #[cfg(feature = "backtrace")]
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

    #[cfg(not(feature = "backtrace"))]
    fn symbolify(&self) -> (&str, Option<&str>) {
        (self.name().unwrap_or(UNKNOWN), None)
    }
}

impl Display for Bundle<'_, &Frame<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config;
        let theme = &config.theme;
        let frame = self.data;
        let (name, hash) = frame.symbolify();

        #[allow(clippy::obfuscated_if_else)]
        let name = frame
            .is_dependency_code()
            .then_some(theme.dependency)
            .unwrap_or(theme.package)
            .style(name);
        let hash = hash.unwrap_or("");
        let hash = theme.hash.style(hash);
        let location = config.bundle(frame.location.as_ref());

        write!(f, "{:>2}: {name}{hash}\n    at {location}", frame.index)
    }
}

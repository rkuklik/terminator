use owo_colors::style;
use owo_colors::Style;

macro_rules! theme {
    ($(#[$meta:meta] $name:ident),* $(,)?) => {
        /// Setting for appearance of `terminator` messages
        #[derive(Debug, Clone, Default)]
        #[must_use]
        pub struct Theme {
            $(
            pub(crate) $name: Style,
            )*
        }

        impl Theme {
        $(
            #[$meta]
            #[inline]
            pub fn $name(mut self, style: Style) -> Self {
                self.$name = style;
                self
            }
        )*
        }
    };
}

theme! {
    /// Styles printed paths
    file,
    /// Styles the line number of a file
    line,
    /// Styles errors printed by `EyreHandler`
    error,
    /// Styles code that is not part of your crate
    dependency,
    /// Styles code that's in your crate
    package,
    /// Styles the hash after `dependency_code` and `crate_code`
    hash,
    /// Styles the header of a panic
    header,
    /// Styles the message of a panic
    message,
    /// Styles the "N frames hidden" message
    hidden,
}

impl Theme {
    /// Creates a blank theme
    pub fn new() -> Self {
        Self::dark()
    }

    /// Returns a theme for dark backgrounds. This is the default
    pub fn dark() -> Self {
        Self {
            file: style().purple(),
            line: style().purple(),
            error: style().bright_red(),
            dependency: style().green(),
            package: style().bright_red(),
            hash: style().bright_black(),
            header: style().red(),
            message: style().cyan(),
            hidden: style().bright_cyan(),
        }
    }

    /// Returns a theme for light backgrounds
    pub fn light() -> Self {
        Self {
            file: style().purple(),
            line: style().purple(),
            error: style().red(),
            dependency: style().green(),
            package: style().red(),
            hash: style().bright_black(),
            header: style().red(),
            message: style().blue(),
            hidden: style().blue(),
        }
    }
}

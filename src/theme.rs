use std::fmt;

macro_rules! color {
    ($($name:ident $fg:literal $bg:literal),* $(,)?) => {
        /// Color setting for text and background
        #[derive(Debug, Clone, Copy)]
        #[must_use]
        #[non_exhaustive]
        #[cfg_attr(not(doc), repr(u8))]
        pub enum Color {
            $(
            #[allow(missing_docs)]
            $name,
            )*
        }

        impl Default for Color {
            fn default() -> Self {
                Self::Default
            }
        }

        impl Color {
            const fn fg(self) -> &'static str {
                match self {
                    $(
                    Self::$name => stringify!($fg),
                    )*

                }
            }

            const fn bg(self) -> &'static str {
                match self {
                    $(
                    Self::$name => stringify!($bg),
                    )*
                }
            }
        }
    };
}

color! {
    Default 39 49,
    Black   30 40,
    Red     31 41,
    Green   32 42,
    Yellow  33 43,
    Blue    34 44,
    Magenta 35 45,
    Cyan    36 46,
    White   37 47,

    BrightBlack   90 100,
    BrightRed     91 101,
    BrightGreen   92 102,
    BrightYellow  93 103,
    BrightBlue    94 104,
    BrightMagenta 95 105,
    BrightCyan    96 106,
    BrightWhite   97 107,
}

macro_rules! effect {
    ($($name:ident $num:literal),* $(,)?) => {
        /// Effect setting for text and background
        #[derive(Debug, Clone, Copy)]
        #[must_use]
        #[non_exhaustive]
        #[cfg_attr(not(doc), repr(u8))]
        pub enum Effect {
            $(
            #[allow(missing_docs)]
            $name = $num,
            )*
        }

        impl Effect {
            const ALL: [Self; 9] = [$(Self::$name,)*];

            const fn ansi(self) -> &'static str {
                match self {
                    $(
                    Self::$name => stringify!($num),
                    )*
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
struct Colors {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl Colors {
    const fn new() -> Self {
        Self { fg: None, bg: None }
    }

    const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

effect! {
    Bold          1,
    Dimmed        2,
    Italic        3,
    Underline     4,
    Blink         5,
    BlinkFast     6,
    Reversed      7,
    Hidden        8,
    Strikethrough 9,
}

#[derive(Debug, Clone, Copy, Default)]
struct Effects {
    bytes: u16,
}

impl Effects {
    const fn new() -> Self {
        Self { bytes: 0 }
    }

    const fn set(mut self, effect: Effect) -> Self {
        self.bytes |= 1 << effect as u16;
        self
    }

    const fn unset(mut self, effect: Effect) -> Self {
        self.bytes |= 0 << effect as u16;
        self
    }

    const fn get(self, effect: Effect) -> Option<Effect> {
        if (self.bytes >> effect as u16) & 1 == 1 {
            Some(effect)
        } else {
            None
        }
    }
}

/// Appearance setting for text
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
#[non_exhaustive]
pub struct Style {
    colors: Colors,
    effects: Effects,
}

impl Style {
    /// Creates new empty style
    #[inline]
    pub const fn new() -> Self {
        Self {
            colors: Colors::new(),
            effects: Effects::new(),
        }
    }

    /// Sets foreground [`Color`]
    #[inline]
    pub const fn fg(mut self, color: Color) -> Self {
        self.colors = self.colors.fg(color);
        self
    }

    /// Sets background [`Color`]
    #[inline]
    pub const fn bg(mut self, color: Color) -> Self {
        self.colors = self.colors.bg(color);
        self
    }

    /// Adds [`Effect`]
    #[inline]
    pub const fn set(mut self, effect: Effect) -> Self {
        self.effects = self.effects.set(effect);
        self
    }

    /// Removes [`Effect`]
    #[inline]
    pub const fn unset(mut self, effect: Effect) -> Self {
        self.effects = self.effects.unset(effect);
        self
    }

    pub(crate) fn style<T: fmt::Display>(self, thing: T) -> Styled<T> {
        Styled { style: self, thing }
    }
}

pub(crate) struct Styled<T> {
    style: Style,
    thing: T,
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Styled {
            style: Style { colors, effects },
            thing,
        } = self;

        let meta = colors.bg.is_some() || colors.fg.is_some() || effects.bytes != 0;

        if meta {
            f.write_str("\x1b[")?;
            let colors = [colors.fg.map(Color::fg), colors.bg.map(Color::bg)]
                .into_iter()
                .flatten();
            let effects = Effect::ALL
                .into_iter()
                .filter_map(|effect| effects.get(effect))
                .map(Effect::ansi);
            for (index, code) in colors.chain(effects).enumerate() {
                if index != 0 {
                    f.write_str(";")?;
                }
                f.write_str(code)?;
            }
            f.write_str("m")?;
        }
        fmt::Display::fmt(&thing, f)?;
        if meta {
            f.write_str("\x1b[0m")?;
        }
        Ok(())
    }
}

macro_rules! theme {
    ($(#[$meta:meta] $name:ident),* $(,)?) => {
        /// Setting for appearance of `terminator` messages
        #[derive(Debug, Clone, Default)]
        #[must_use]
        #[non_exhaustive]
        pub struct Theme {
            $(
            #[$meta]
            pub(crate) $name: Style,
            )*
        }

        impl Theme {
        $(
            #[$meta]
            #[inline]
            pub const fn $name(mut self, style: Style) -> Self {
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
    /// Creates a blank field
    #[inline]
    pub const fn blank() -> Self {
        Self {
            file: Style::new(),
            line: Style::new(),
            error: Style::new(),
            dependency: Style::new(),
            package: Style::new(),
            hash: Style::new(),
            header: Style::new(),
            message: Style::new(),
            hidden: Style::new(),
        }
    }

    /// Creates a default theme
    #[inline]
    pub const fn new() -> Self {
        Self::dark()
    }

    /// Returns a theme for dark backgrounds. This is the default
    #[inline]
    pub const fn dark() -> Self {
        Self {
            file: Style::new().fg(Color::Magenta),
            line: Style::new().fg(Color::Magenta),
            error: Style::new().fg(Color::BrightRed),
            dependency: Style::new().fg(Color::Green),
            package: Style::new().fg(Color::BrightRed),
            hash: Style::new().fg(Color::BrightBlack),
            header: Style::new().fg(Color::Red),
            message: Style::new().fg(Color::Cyan),
            hidden: Style::new().fg(Color::BrightCyan),
        }
    }

    /// Returns a theme for light backgrounds
    #[inline]
    pub const fn light() -> Self {
        Self {
            file: Style::new().fg(Color::Magenta),
            line: Style::new().fg(Color::Magenta),
            error: Style::new().fg(Color::Red),
            dependency: Style::new().fg(Color::Green),
            package: Style::new().fg(Color::Red),
            hash: Style::new().fg(Color::BrightBlack),
            header: Style::new().fg(Color::Red),
            message: Style::new().fg(Color::Blue),
            hidden: Style::new().fg(Color::Blue),
        }
    }
}

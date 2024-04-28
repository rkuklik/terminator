use std::cell::Cell;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

use owo_colors::OwoColorize;

use crate::config::Bundle;
use crate::verbosity::Verbosity;

use super::convert::BacktraceParser;
use super::Backtrace;
use super::Frame;

struct Hidden<'a> {
    buffer: &'a RefCell<String>,
    count: usize,
}

impl Display for Bundle<'_, &Hidden<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Hidden { buffer, count } = *self.data;
        let mut buffer = buffer.borrow_mut();
        buffer.clear();
        write!(
            buffer,
            "{decorator} {count} frame{plural} hidden {decorator}",
            plural = if count == 1 { "" } else { "s" },
            decorator = "⋮",
        )?;
        write!(f, "{:^80}", buffer.style(self.config.theme.hidden))
    }
}

impl Display for Bundle<'_, &Backtrace<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config;
        let mut frames: Vec<_> = self.data.frames.replace(Vec::new());

        write!(f, "{:━^80}", " BACKTRACE ")?;
        if frames.is_empty() {
            return write!(f, "{:^80}", "<empty backtrace>");
        }

        let last = frames.last().map_or(0, Frame::index);

        if config.selected_verbosity() != Verbosity::Full {
            for filter in &config.filters {
                filter(&mut frames);
                frames.sort_unstable_by_key(Frame::index);
            }
        }

        let buffer = RefCell::new(String::with_capacity(128));
        let mut next = 0;
        for frame in frames {
            let delta = frame.index - next;
            if delta != 0 {
                write!(
                    f,
                    "\n{}",
                    config.bundle(&Hidden {
                        buffer: &buffer,
                        count: delta
                    })
                )?;
            }
            write!(f, "\n{}", config.bundle(&frame))?;
            next = frame.index + 1;
        }

        if (last + 1) != next {
            write!(
                f,
                "\n{}",
                config.bundle(&Hidden {
                    buffer: &buffer,
                    count: last - next
                })
            )?;
        }

        Ok(())
    }
}

#[cfg(feature = "backtrace")]
impl Display for Bundle<'_, &'_ backtrace::Backtrace> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let backtrace = Backtrace::from(*self.data());
        Display::fmt(&self.config().bundle(&backtrace), f)
    }
}

impl Display for Bundle<'_, &std::backtrace::Backtrace> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let backtrace = self.data.to_string();
        let frames = BacktraceParser::new(&backtrace).collect();
        let cell = Cell::new(frames);
        let backtrace = Backtrace::from(cell);
        Display::fmt(&self.config.bundle(&backtrace), f)
    }
}

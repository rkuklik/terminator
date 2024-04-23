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

impl Display for Bundle<'_, Hidden<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config();
        let Hidden { buffer, count } = self.data();
        let mut buffer = buffer.borrow_mut();
        buffer.clear();
        write!(
            buffer,
            "{decorator} {count} frame{plural} hidden {decorator}",
            plural = if *count == 1 { "" } else { "s" },
            decorator = "⋮",
        )?;
        write!(f, "{:^80}", buffer.style(config.theme.hidden))
    }
}

impl Display for Bundle<'_, &Cell<Vec<Frame<'_>>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config();
        let mut frames: Vec<_> = self.data().replace(Vec::new());

        let last = frames.last().map_or(0, Frame::index);

        if config.selected_verbosity() != Verbosity::Full {
            for filter in &config.filters {
                filter(&mut frames);
                frames.sort_unstable_by_key(Frame::index);
            }
        }

        writeln!(f, "{:━^80}", " BACKTRACE ")?;
        if frames.is_empty() {
            return write!(f, "{:^80}", "<empty backtrace>");
        }

        let buffer = RefCell::new(String::with_capacity(128));
        let mut next = 0;
        for frame in frames {
            let delta = frame.index - next;
            if delta != 0 {
                writeln!(
                    f,
                    "{}",
                    config.bundle(Hidden {
                        buffer: &buffer,
                        count: delta
                    })
                )?;
            }
            writeln!(f, "{}", config.bundle(&frame))?;
            next = frame.index + 1;
        }

        if (last + 1) != next {
            write!(
                f,
                "{}",
                config.bundle(Hidden {
                    buffer: &buffer,
                    count: last - next
                })
            )?;
        }

        Ok(())
    }
}

impl Display for Bundle<'_, &'_ backtrace::Backtrace> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let backtrace = Backtrace::from(*self.data());
        Display::fmt(&self.config().bundle(&backtrace.frames), f)
    }
}

impl Display for Bundle<'_, &'_ std::backtrace::Backtrace> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let backtrace = self.data().to_string();
        let frames = BacktraceParser::new(&backtrace).collect();
        let cell = Cell::new(frames);
        Display::fmt(&self.config().bundle(&cell), f)
    }
}

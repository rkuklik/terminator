use std::borrow::Cow;
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Lines;

use crate::consts::UNKNOWN;
use crate::location::Location;

use super::Backtrace;
use super::Frame;

struct BacktraceString(String);

impl<'a> From<Vec<Frame<'a>>> for Backtrace<'a> {
    fn from(value: Vec<Frame<'a>>) -> Self {
        Cell::new(value).into()
    }
}

impl<'a> From<Cell<Vec<Frame<'a>>>> for Backtrace<'a> {
    fn from(value: Cell<Vec<Frame<'a>>>) -> Self {
        Backtrace { frames: value }
    }
}

#[cfg(feature = "backtrace")]
impl<'a> From<&'a backtrace::Backtrace> for Backtrace<'a> {
    fn from(value: &'a backtrace::Backtrace) -> Self {
        value
            .frames()
            .iter()
            .flat_map(backtrace::BacktraceFrame::symbols)
            .enumerate()
            .map(|(index, symbol)| Frame {
                index,
                name: symbol
                    .name()
                    .as_ref()
                    .map(ToString::to_string)
                    .map(Cow::Owned),
                location: if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                    Some(Location {
                        file: match file.to_str() {
                            Some(file) => Cow::Borrowed(file),
                            None => Cow::Owned(file.display().to_string()),
                        },
                        line,
                    })
                } else {
                    None
                },
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl<'a> From<&'a BacktraceString> for Backtrace<'a> {
    fn from(value: &'a BacktraceString) -> Self {
        BacktraceParser::new(&value.0).collect::<Vec<_>>().into()
    }
}

pub(super) struct BacktraceParser<'a> {
    source: Peekable<Lines<'a>>,
}

impl<'a> BacktraceParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.lines().peekable(),
        }
    }
}

impl<'a> Iterator for BacktraceParser<'a> {
    type Item = Frame<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let (index, name) = self
            .source
            .next()?
            .trim()
            .split_once(": ")
            .map(|(index, name)| (index.parse().expect("`usize` index"), name))
            .map(|(index, name)| (index, (name != UNKNOWN).then_some(Cow::Borrowed(name))))?;
        let location = self
            .source
            .peek()
            .copied()
            .unwrap_or("")
            .split_once("at ")
            .and_then(|(_, location)| location.rsplit_once(':')?.0.rsplit_once(':'))
            .map(|(file, line)| (Cow::Borrowed(file), line.parse().expect("`usize` line")))
            .map(|(file, line)| Location { file, line });
        if location.is_some() {
            self.source.next();
        }
        Some(Self::Item {
            index,
            name,
            location,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.source.size_hint();
        (lower / 2, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_backtrace() {
        let backtrace = "\
   0: anyhow::error::<impl anyhow::Error>::msg
             at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.81/src/error.rs:83:36
   1: anyhow::__private::format_err
             at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.81/src/lib.rs:688:13
   2: aoc2023::day1::exec
             at ./aoc2023/src/day1.rs:52:17
   3: aoc2023::day1::first
             at ./aoc2023/src/day1.rs:22:5
   4: core::ops::function::FnOnce::call_once
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
   5: <F as aoc::Eval<A>>::eval
             at ./src/lib.rs:17:9
   6: aoc::main
             at ./src/main.rs:46:18
   7: core::ops::function::FnOnce::call_once
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
   8: std::sys_common::backtrace::__rust_begin_short_backtrace
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:155:18
   9: std::rt::lang_start::{{closure}}
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:166:18
  10: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:284:13
  11: std::panicking::try::do_call
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:554:40
  12: std::panicking::try
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:518:19
  13: std::panic::catch_unwind
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panic.rs:142:14
  14: std::rt::lang_start_internal::{{closure}}
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:148:48
  15: std::panicking::try::do_call
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:554:40
  16: std::panicking::try
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:518:19
  17: std::panic::catch_unwind
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panic.rs:142:14
  18: std::rt::lang_start_internal
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:148:20
  19: std::rt::lang_start
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:165:17
  20: main
  21: <unknown>
  22: __libc_start_main
  23: _start
";
        let backtrace: Vec<_> = BacktraceParser::new(backtrace).collect();
        assert_eq!(
            backtrace.len(),
            24,
            "Backtrace had wrong length {backtrace:#?}"
        );
    }

    #[test]
    fn plain_frame() {
        let frame = "  20: main";
        let mut parser = BacktraceParser::new(frame);
        assert_eq!(
            parser.next(),
            Some(Frame {
                index: 20,
                name: Some(Cow::Borrowed("main")),
                location: None
            })
        );
        assert_eq!(parser.next(), None);
        let frame = "  21: <unknown>";
        let mut parser = BacktraceParser::new(frame);
        assert_eq!(
            parser.next(),
            Some(Frame {
                index: 21,
                name: None,
                location: None
            })
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn located_frame() {
        let frame = "   6: aoc::main\n             at ./src/main.rs:46:18";
        let mut parser = BacktraceParser::new(frame);
        assert_eq!(
            parser.next(),
            Some(Frame {
                index: 6,
                name: Some(Cow::Borrowed("aoc::main")),
                location: Some(Location {
                    file: Cow::Borrowed("./src/main.rs"),
                    line: 46
                })
            })
        );
        assert_eq!(parser.next(), None);
    }
}

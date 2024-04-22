use std::borrow::Cow;

use winnow::ascii::dec_uint;
use winnow::ascii::digit1;
use winnow::ascii::line_ending;
use winnow::ascii::space0;
use winnow::ascii::till_line_ending;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::separated_pair;
use winnow::combinator::seq;
use winnow::combinator::terminated;
use winnow::token::any;
use winnow::token::take_until;
use winnow::PResult;
use winnow::Parser;

use crate::location::Location;

use super::Backtrace;
use super::Frame;

pub(super) struct StdBacktraceString(pub String);

impl From<&backtrace::Backtrace> for Backtrace<'_> {
    fn from(value: &backtrace::Backtrace) -> Self {
        let frames = value
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
                        file: Cow::Owned(file.display().to_string()),
                        line,
                    })
                } else {
                    None
                },
            })
            .collect();
        Self { frames }
    }
}

impl<'a> From<&'a StdBacktraceString> for Backtrace<'a> {
    fn from(value: &'a StdBacktraceString) -> Self {
        Self::parse.parse(&value.0).unwrap_or_default()
    }
}

impl<'a> Backtrace<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        repeat(0.., Frame::parse)
            .parse_next(input)
            .map(|frames| Self { frames })
    }
}

impl<'a> Frame<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        seq!(Frame {
            index: delimited(space0, dec_uint, ": "),
            name: till_line_ending
                .map(Cow::Borrowed)
                .map(Some),
            _: line_ending,
            location: opt(preceded((space0, "at "), Location::parse)),
        })
        .parse_next(input)
    }
}

impl<'a> Location<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        terminated(
            separated_pair(take_until(0.., ':'), any, dec_uint),
            (':', digit1, line_ending),
        )
        .map(|(file, line)| Location {
            file: Cow::Borrowed(file),
            line,
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backtrace() {
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
        let backtrace = Backtrace::parse.parse(backtrace).expect("backtrace");
        assert_eq!(backtrace.frames.len(), 24);
    }
}

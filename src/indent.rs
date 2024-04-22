use std::fmt::Result;
use std::fmt::Write;
use std::num::NonZeroUsize;

pub struct Indent<'a, F>
where
    F: Write + ?Sized,
{
    indentation: &'a str,
    writer: &'a mut F,
}

impl<'a, F> Indent<'a, F>
where
    F: Write + ?Sized,
{
    pub fn new(writer: &'a mut F, indentation: &'a str) -> Self {
        Self {
            indentation,
            writer,
        }
    }

    pub fn double(writer: &'a mut F) -> Self {
        Self::new(writer, "  ")
    }
}

impl<F> Write for Indent<'_, F>
where
    F: Write + ?Sized,
{
    fn write_str(&mut self, s: &str) -> Result {
        let Self {
            indentation,
            writer,
        } = self;
        let mut start = None;
        for index in s
            .match_indices('\n')
            .map(|(index, _)| index)
            .chain(Some(s.len()))
        {
            let begin = start.map_or(0, NonZeroUsize::get);
            let line = &s[begin..index];
            let (newline, indentation) = match start {
                None => ("", ""),
                Some(_) => ("\n", *indentation),
            };
            write!(writer, "{newline}{indentation}{line}")?;
            start = NonZeroUsize::new(index + 1);
        }
        Ok(())
    }
}

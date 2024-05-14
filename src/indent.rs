use std::fmt::Result;
use std::fmt::Write;

pub struct Indent<'a, F>
where
    F: Write + ?Sized,
{
    indentation: &'a str,
    writer: &'a mut F,
    requires: bool,
}

impl<'a, F> Indent<'a, F>
where
    F: Write + ?Sized,
{
    pub fn new(writer: &'a mut F, indentation: &'a str) -> Self {
        Self {
            indentation,
            writer,
            requires: true,
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
        for (ind, line) in s.split('\n').enumerate() {
            if ind > 0 {
                self.writer.write_char('\n')?;
                self.requires = true;
            }

            // Don't render the line unless its actually got text on it
            if line.is_empty() {
                continue;
            }

            if self.requires {
                self.writer.write_str(self.indentation)?;
                self.requires = false;
            }

            self.writer.write_str(line)?;
        }

        Ok(())
    }
}

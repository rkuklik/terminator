use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write as _;
use std::io::stderr;
use std::io::LineWriter;
use std::io::Write;
use std::panic::PanicInfo;

use owo_colors::OwoColorize;

use crate::config::Bundle;
use crate::indent::Indent;
use crate::location::Location;
use crate::Config;
use crate::GLOBAL_SETTINGS;

impl Config {
    /// Panic hook which references provided [`Config`]
    ///
    /// This can be used as panic hook only when `&self` id `'static`
    pub fn panic_hook(&self) -> impl Fn(&PanicInfo<'_>) + Sync + Send + '_ {
        |info| _ = write!(LineWriter::new(stderr()), "{}", self.bundle(info))
    }

    /// Panic hook which lazily retrieves [`Config`] from global settings.
    pub fn lazy_panic_hook() -> impl Fn(&PanicInfo<'_>) + Sync + Send + 'static {
        |info| GLOBAL_SETTINGS.get_or_init(Self::default).panic_hook()(info)
    }
}

impl Display for Bundle<'_, &'_ PanicInfo<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let config = self.config();
        let payload = self.data().payload();
        let payload = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("<non string panic payload>");

        let theme = &config.theme;
        let info = "The application panicked (crashed).".style(theme.header);
        let message = payload.style(theme.message);
        let location = self.data().location().map(Location::derived);
        let location = config.bundle(location.as_ref());

        writeln!(f, "{info}")?;
        writeln!(f, "Message:  {message}")?;
        writeln!(f, "Location: {location}")?;

        let backtrace = std::backtrace::Backtrace::force_capture();
        write!(Indent::double(f), "\n{}", config.bundle(&backtrace))?;
        writeln!(f)
    }
}

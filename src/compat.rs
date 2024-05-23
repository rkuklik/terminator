use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;
use std::ops::DerefMut;

mod sealed {
    use std::error::Error;

    pub trait Sealed {
        #[doc(hidden)]
        /// The lower-level source of this error, if any
        fn source(&self) -> Option<&(dyn Error + 'static)>;
    }
    impl Sealed for eyre::Report {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.chain().nth(1)
        }
    }
    impl Sealed for anyhow::Error {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.chain().nth(1)
        }
    }
}

/// Error shim for use with [`Compat`]
pub trait ErrorCompat: Debug + Display + sealed::Sealed {}

impl ErrorCompat for anyhow::Error {}

impl ErrorCompat for eyre::Report {}

/// Newtype wrapper for trait object based error types to implement [`Error`]
pub struct Compat<T>(pub T)
where
    T: ErrorCompat;

impl<T: ErrorCompat> Debug for Compat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: ErrorCompat> Display for Compat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: ErrorCompat> Error for Compat<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        sealed::Sealed::source(&self.0)
    }
}

impl<T: ErrorCompat> AsRef<T> for Compat<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: ErrorCompat> AsMut<T> for Compat<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: ErrorCompat> Deref for Compat<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ErrorCompat> DerefMut for Compat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MSG: &str = "some random error";

    #[test]
    fn conversions() {
        let anyhow_error = anyhow::Error::msg(MSG);
        let eyre_error = eyre::Report::msg(MSG);
        let eyre: eyre::Report = Compat(anyhow_error).into();
        let anyhow: anyhow::Error = Compat(eyre_error).into();
        assert_eq!(eyre.to_string(), anyhow.to_string());
    }
}

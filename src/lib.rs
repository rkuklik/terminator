#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//! ## Appearance
//!
//! By default, `Terminator` should look like the following
#![doc = include_str!("../backtrace.html")]

#[cfg(all(feature = "anyhow", feature = "eyre"))]
#[rustfmt::skip]
compile_error!(r#"Features `anyhow` and `eyre` are mutualy exclusive. Use feature `compat` to bridge between the two."#);

use std::sync::OnceLock;

pub use backtrace::Frame;
pub use backtrace::FrameFilter;
pub use config::Config;
pub use error::InstallError;
pub use error::Terminator;
pub use location::Location;
pub use theme::Color;
pub use theme::Effect;
pub use theme::Style;
pub use theme::Theme;
pub use verbosity::Verbosity;

mod backtrace;
mod config;
mod consts;
mod error;
mod indent;
mod location;
mod panic;
mod theme;
mod verbosity;

macro_rules! cfg_compat {
    ($($item:item)*) => {
        $(
        #[cfg(feature = "compat")]
        #[cfg_attr(docsrs, doc(cfg(feature = "compat")))]
        $item
        )*
    };
}

cfg_compat!(
    mod compat;
    pub use compat::Compat;
    pub use compat::ErrorCompat;
);

static GLOBAL_SETTINGS: OnceLock<Config> = OnceLock::new();

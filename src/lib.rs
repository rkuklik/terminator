#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use std::sync::OnceLock;

pub use backtrace::Frame;
pub use backtrace::FrameFilter;
pub use config::Config;
pub use error::InstallError;
pub use error::PrettyError;
pub use location::Location;
pub use theme::Theme;
pub use verbosity::Verbosity;

pub(crate) use indent::Indent;

mod backtrace;
mod config;
mod consts;
mod error;
mod indent;
mod location;
mod panic;
mod theme;
mod verbosity;

static GLOBAL_SETTINGS: OnceLock<Config> = OnceLock::new();

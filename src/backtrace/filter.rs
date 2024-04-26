use crate::consts::FILE_PREFIXES_DEP;
use crate::consts::SYM_PREFIX_DEP;
use crate::consts::SYM_PREFIX_INIT;
use crate::consts::SYM_PREFIX_INTERNAL;
use crate::consts::SYM_PREFIX_PANIC;
use crate::location::Location;

use super::Frame;

/// Callback for filtering a vector of [`Frame`]s
pub type FrameFilter = dyn Fn(&mut Vec<Frame>) + Send + Sync + 'static;

impl Frame<'_> {
    pub(super) fn is_dependency_code(&self) -> bool {
        // Inspect name.
        if let Some(ref name) = self.name {
            if SYM_PREFIX_DEP.iter().copied().any(prefixes(name)) {
                return true;
            }
        }

        // Inspect filename.
        if let Some(ref location) = self.location {
            let filename = location.file();
            if FILE_PREFIXES_DEP.iter().copied().any(prefixes(filename))
                || filename.contains("/.cargo/registry/src/")
            {
                return true;
            }
        }

        false
    }

    /// Heuristically determine whether a frame is likely to be a post panic
    /// frame.
    ///
    /// Post panic frames are frames of a functions called after the actual panic
    /// is already in progress and don't contain any useful information for a
    /// reader of the backtrace.
    fn is_post_panic_code(&self) -> bool {
        match self.name.as_ref() {
            Some(name) => SYM_PREFIX_PANIC.iter().copied().any(prefixes(name)),
            None => false,
        }
    }

    /// Heuristically determine whether a frame is likely to be part of language
    /// runtime.
    fn is_runtime_init_code(&self) -> bool {
        let (Some(name), Some(file)) = (
            self.name.as_ref(),
            self.location.as_ref().map(Location::file),
        ) else {
            return false;
        };

        if SYM_PREFIX_INIT.iter().copied().any(prefixes(name)) {
            return true;
        }

        // For Linux, this is the best rule for skipping test init I found.
        if name == "{{closure}}" && file == "src/libtest/lib.rs" {
            return true;
        }

        false
    }

    fn is_internal_machinery(&self) -> bool {
        let Some(name) = self.name() else {
            return false;
        };
        SYM_PREFIX_INTERNAL.iter().copied().any(prefixes(name))
    }

    pub(crate) fn default_filters() -> Vec<Box<FrameFilter>> {
        vec![Box::new(runtime), Box::new(internal)]
    }
}

fn runtime(frames: &mut Vec<Frame>) {
    let top = frames
        .iter()
        .rposition(Frame::is_post_panic_code)
        .map_or(0, |x| x + 1);

    let bottom = frames
        .iter()
        .position(Frame::is_runtime_init_code)
        .unwrap_or(frames.len());

    let range = top..bottom;

    frames.retain(|frame| range.contains(&frame.index()));
}

fn internal(frames: &mut Vec<Frame>) {
    frames.retain(|frame| !frame.is_internal_machinery());
}

fn prefixes(string: &str) -> impl Fn(&str) -> bool + '_ {
    |prefix| string.starts_with(prefix)
}

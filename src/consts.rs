macro_rules! constified {
    ($name:ident: $type:ty => $value:expr) => {
        pub const $name: $type = $value;
    };
    (
    $type:ty;
    $($name:ident => $value:expr),+ $(,)?
    ) => {
        $(
        constified!($name: $type => $value);
        )+
    };
}

macro_rules! unknown {
    ($($thing:literal)?) => {
        concat!("<unknown", $(" ", $thing,)? ">")
    };
}

constified!(
    &str;
    UNKNOWN => unknown!(),
    //UNKNOWN_FILE => unknown!("source file"),
    //UNKNOWN_LINE => unknown!("line"),
    BACKTRACE => "RUST_BACKTRACE",
    LIB_BACKTRACE => "RUST_LIB_BACKTRACE",
);

constified!(
    &[&str];
    SYM_PREFIX_DEP => &[
        "std::",
        "core::",
        "backtrace::backtrace::",
        "_rust_begin_unwind",
        "color_traceback::",
        "__rust_",
        "___rust_",
        "__pthread",
        "_main",
        "main",
        "__scrt_common_main_seh",
        "BaseThreadInitThunk",
        "_start",
        "__libc_start_main",
        "start_thread",
    ],
    SYM_PREFIX_PANIC => &[
        "_rust_begin_unwind",
        "rust_begin_unwind",
        "core::result::unwrap_failed",
        "core::option::expect_none_failed",
        "core::panicking::panic_fmt",
        "color_backtrace::create_panic_handler",
        "std::panicking::begin_panic",
        "begin_panic_fmt",
        "failure::backtrace::Backtrace::new",
        "backtrace::capture",
        "failure::error_message::err_msg",
        "<failure::error::Error as core::convert::From<F>>::from",
    ],
    SYM_PREFIX_INIT => &[
        "std::rt::lang_start::",
        "test::run_test::run_test_inner::",
        "std::sys_common::backtrace::__rust_begin_short_backtrace",
    ],
    SYM_PREFIX_INTERNAL => &[
        "anyhow::",
        "eyre::",
        "terminator::",
    ],
    FILE_PREFIXES_DEP => &[
        "/rustc/",
        "src/libstd/",
        "src/libpanic_unwind/",
        "src/libtest/",
    ],
);

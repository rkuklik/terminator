# terminator

Rust library to provide fancy formatting for errors and backtraces. Based of and
inspired by [`color-eyre`](https://crates.io/crates/color-eyre). It is intended
to be used in binary applications, not as error type in libraries.

## Usage

To display pretty errors from your main function, you can just switch your error type
to `terminator::Terminator`. However, to customise the appearance and behaviour of
errors or to pretty print panics, set up your main function as below:

```rust
fn main() -> Result<(), terminator::Terminator> {
    terminator::Config::new()
        // modify config if you so wish
        // and install config (setting up panic hook)
        .install()?;

    Ok(())
}
```

## Feature flags

Terminator can bundle support for common error trait object libraries like
[`anyhow`](https://crates.io/crates/anyhow) and [`eyre`](https://crates.io/crates/eyre).
Setting respective flags will enable conversions and `?` operator for `Terminator`.
By default, `Terminator` is backed by `Box<dyn Error>`.

Following flags are provided:

- **anyhow**: use `anyhow::Error` as backend for `Terminator` (conflicts with **eyre** feature)
- **eyre**: use `eyre::Report` as backend for `Terminator` (conflicts with **anyhow** feature)
- **compat**: enable `Compat` struct as bridge between `eyre` and `anyhow` if both are used

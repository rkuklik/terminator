# terminator

Rust library to provide fancy formatting for errors and backtraces. Based of and
inspired by [`color-eyre`](https://crates.io/crates/color-eyre).
Currently backed by [`anyhow`](https://crates.io/crates/anyhow)

## Usage

To display pretty errors from your main function, you can just switch your error type
to `terminator::PrettyError`. However, to customise the appearance and behaviour of
errors or to pretty print panics, set up your main function as below:

```rust
fn main() -> Result<(), terminator::PrettyError> {
    terminator::Config::new()
        // modify config as per docs
        // then save config and setup panic hook
        .install()?;

    Ok(())
}
```

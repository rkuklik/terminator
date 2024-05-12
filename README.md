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

## Appearance

By default, `Terminator` should look like the following (you should see colors)

<pre>
Error:
   0: <span style="color: red">alright, I'm done, show yourself out with pretty formatting and a fancy backtrace</span>
   1: <span style="color: red">it is pretty nasty, let's send it back to caller</span>
   2: <span style="color: red">an error is never late, nor is it early, it arrives precisely when it means to</span>
   3: <span style="color: red">wild error has appeared</span>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                <span style="color: cyan">⋮ 2 frames hidden ⋮</span>
   2: <span style="color: red">example::module::function</span>
      at <span style="color: pink">./src/example.rs</span>:<span style="color: pink">25</span>
   3: <span style="color: lime">core::ops::function::FnOnce::call_once</span>
      at <span style="color: pink">/rustc/hash/library/core/src/ops/function.rs</span>:<span style="color: pink">250</span>
   4: <span style="color: red">&lt;F as example::Eval&lt;A&gt;&gt;::eval</span>
      at <span style="color: pink">./src/lib.rs</span>:<span style="color: pink">20</span>
   5: <span style="color: red">example::main</span>
      at <span style="color: pink">./src/main.rs</span>:<span style="color: pink">25</span>
   6: <span style="color: lime">core::ops::function::FnOnce::call_once</span>
      at <span style="color: pink">/rustc/hash/library/core/src/ops/function.rs</span>:<span style="color: pink">250</span>
                                <span style="color: cyan">⋮ 15 frames hidden ⋮</span>
</pre>

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

By default, `Terminator` should look like:

<pre>
Error:
   0: <span class="err">alright, I'll show myself out with pretty formatting and a fancy backtrace</span>
   1: <span class="err">it is pretty nasty, let's send it back to caller</span>
   2: <span class="err">an error is never late, nor is it early, it arrives precisely when it means to</span>
   3: <span class="err">wild error has appeared</span>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                <span class="hid">⋮ 5 frames hidden ⋮</span>
   2: <span class="sym">example::module::function</span>
      at <span class="loc">./src/example.rs</span>:<span class="loc">25</span>
   3: <span class="dep">core::ops::function::FnOnce::call_once</span>
      at <span class="loc">/rustc/hash/library/core/src/ops/function.rs</span>:<span class="loc">250</span>
   4: <span class="sym">&lt;F as example::Eval&lt;A&gt;&gt;::eval</span>
      at <span class="loc">./src/lib.rs</span>:<span class="loc">20</span>
   5: <span class="sym">example::main</span>
      at <span class="loc">./src/main.rs</span>:<span class="loc">25</span>
   6: <span class="dep">core::ops::function::FnOnce::call_once</span>
      at <span class="loc">/rustc/hash/library/core/src/ops/function.rs</span>:<span class="loc">250</span>
                                <span class="hid">⋮ 15 frames hidden ⋮</span>
</pre>
<style>
        .err,
        .sym {
          color: red;
        }
        .hid {
          color: cyan;
        }
        .dep {
          color: lime;
        }
</style>

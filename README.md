This repo is for experimenting with better ways of displaying backtraces in Rust.

This is part of the [Clean async stack traces](https://github.com/rust-lang/wg-async-foundations/issues/251) initiative in the Rust [Async Foundations Working Group](https://rust-lang.github.io/wg-async-foundations/).

# Examples

This repo currently includes two examples that call several functions and then explicitly panic.
These examples provide a quick way to show how backtraces are formatted.
The two examples are `simple-backtrace.rs`, which shows is a simple  program with no async code, and `async-backtrace.rs` which is very similar except the functions are asynchronous and there is a loop to repeatedly poll the futures so that this can run without an async runtime.

Running the examples currently requires a nightly version of Rust.
To switch to nightly Rust, run the following:

    rustup override set nightly

Then you can run the examples using Cargo:

    cargo run --example simple-backtrace

    cargo run --example async-backtrace

# Features

Currently better-backtrace improves the standard backtrace formatting in two ways:

1. User-friendly formatting of compiler-generated symbols
2. Filtering based on module paths

## Formatting Compiler-generated Symbols

Because of the way the compiler handles async functions, normal backtraces end up with surprising frame names like `async_backtrace::bar::async_fn$0`.

Better-backtrace reformats these to more closely resemble what the programmer wrote in the first place: `async fn async_backtrace::bar`

## Filtering Based On Module Paths

Backtraces normally contain many frames that are usually not interesting to the programmer.
For example, there may be frames that are internal to libstd or whatever async runtime is in use.
Unless the user is developing or debugging these libraries, it would normally be cleaner not to see them.

Better-backtrace supports filtering frames by taking inspiration from the `RUST_LOG` environment variable.
The `BETTER_BACKTRACE` variable can be used to control what frames are shown.
The variable can be set to a comma-separate list of paths that are either included (prefixed by `+`) or excluded (prefixed by `-`).

As an example, here's a backtrace from `async-backtrace.rs` with the standard backtrace:

.... TODO: Add full backtrace ....

On the other hand, here is an example with filtering applied:

```
> BETTER_BACKTRACE="0,+async_backtrace" cargo run --example async-backtrace

panicked at 'explicit panic', examples\async-backtrace.rs:18:5
Backtrace:
 0 [12]: async fn async_backtrace::bar
        at .\examples\async-backtrace.rs:18
 1 [14]: async fn async_backtrace::foo
        at .\examples\async-backtrace.rs:14
 2 [16]: fn async_backtrace::block_on<tuple$<>,core::future::from_generator::GenFuture<async fn async_backtrace::foo> >
        at .\examples\async-backtrace.rs:48
 3 [17]: fn async_backtrace::main
        at .\examples\async-backtrace.rs:10
error: process didn't exit successfully: `target\debug\examples\async-backtrace.exe` (exit code: 101)
```

# Using

# Hacking


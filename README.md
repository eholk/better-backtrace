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

```
> RUST_BACKTRACE=1 cargo run --example async-backtrace

thread 'main' panicked at 'explicit panic', examples\async-backtrace.rs:18:5
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\/library\std\src\panicking.rs:584
   1: core::panicking::panic_fmt
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\/library\core\src\panicking.rs:143
   2: core::panicking::panic
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\/library\core\src\panicking.rs:48
   3: async_backtrace::bar::async_fn$0
             at .\examples\async-backtrace.rs:18
   4: core::future::from_generator::impl$1::poll<async_backtrace::bar::async_fn_env$0>
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\library\core\src\future\mod.rs:91
   5: async_backtrace::foo::async_fn$0
             at .\examples\async-backtrace.rs:14
   6: core::future::from_generator::impl$1::poll<async_backtrace::foo::async_fn_env$0>
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\library\core\src\future\mod.rs:91
   7: async_backtrace::block_on<tuple$<>,core::future::from_generator::GenFuture<async_backtrace::foo::async_fn_env$0> >
             at .\examples\async-backtrace.rs:48
   8: async_backtrace::main
             at .\examples\async-backtrace.rs:10
   9: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/4ce3749235fc31d15ebd444b038a9877e8c700d7\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

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

We first want to emphasize that better-backtrace is meant as a way to explore better backtrace formatting, particularly as it relates to async-heavy code.
Thus, we do not recommend using this in production.

That said, if you would like to try out better-backtrace, we provide a custom panic handler that replaces Rust's built-in backtrace formatting with the ideas we are experimenting with here.
To activate the panic handler, run the following somewhere near the beginning of your program:

    better_backtrace::install_panic_handler().unwrap();

If you have ideas for how formatting could be improved, please [open an issue](https://github.com/eholk/better-backtrace/issues/new)!

# Hacking

This section gives a brief tour of how better-backtrace is implemented.
The goal is to help those wanting to contribute code to get started.

Most of the code lives in [`src/lib.rs`](https://github.com/eholk/better-backtrace/blob/main/src/lib.rs).
The entry point for formatting a backtrace is the `format_backtrace` function.
This function calls out to a number of other helpers to aid with parsing raw frame names, nested type names, etc.
This file also includes code to parse the `BETTER_BACKTRACE` environment variable into a list of `FilterClause`s, which are used to decide which frames to show.

The panic handler is implemented in [`src/panic_handler.rs`](https://github.com/eholk/better-backtrace/blob/main/src/panic_handler.rs).
It essentially just calls the `format_backtrace` function.

Finally, we have several unit tests in [`src/test.rs`](https://github.com/eholk/better-backtrace/blob/main/src/test.rs).
These mostly focus on parsing portions of raw frame names, and formatting specific kinds of frame names.

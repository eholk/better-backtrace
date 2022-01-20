This repo is for experimenting with better ways of displaying backtraces in Rust.

This is part of the [Clean async stack traces](https://github.com/rust-lang/wg-async-foundations/issues/251) initiative in the Rust [Async Foundations Working Group](https://rust-lang.github.io/wg-async-foundations/).

Example async backtrace:

```
panicked at 'explicit panic', examples\async-backtrace.rs:18:5
Backtrace:
 0 [12]: async fn async_backtrace::bar
        at C:\Users\ericholk\repo\better-backtrace\examples\async-backtrace.rs:18
 1 [14]: async fn async_backtrace::foo
        at C:\Users\ericholk\repo\better-backtrace\examples\async-backtrace.rs:14
 2 [17]: fn async_backtrace::main
        at C:\Users\ericholk\repo\better-backtrace\examples\async-backtrace.rs:10
```

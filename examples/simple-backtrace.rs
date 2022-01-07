fn main() {
    better_backtrace::install_panic_handler().unwrap();
    foo()
}

fn foo() {
    bar()
}

fn bar() {
    panic!()
}
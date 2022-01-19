use std::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

fn main() {
    better_backtrace::install_panic_handler().unwrap();
    block_on(foo());
}

async fn foo() {
    bar().await
}

async fn bar() {
    panic!()
}

fn block_on<R, F>(mut fut: F) -> R
where
    F: Future<Output = R>,
{
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    fn clone(_: *const ()) -> RawWaker {
        unimplemented!()
    }

    fn wake(_: *const ()) {
        unimplemented!()
    }

    fn wake_by_ref(_: *const ()) {
        unimplemented!()
    }

    fn drop(_: *const ()) {}

    const vtable: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &vtable)) };

    let mut cx = Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(ret) => return ret,
            std::task::Poll::Pending => continue,
        }
    }
}

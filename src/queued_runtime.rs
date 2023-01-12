extern crate alloc;
#[cfg(not(feature = "no_std"))]
extern crate std;

use core::mem;
use core::time::Duration;
use core::{cell::RefCell, future::Future, pin::Pin, task::Poll};

use alloc::boxed::Box;
use alloc::collections::VecDeque;

use async_core::*;

/// A very small async runtime, with support for adding more tasks as it runs. This uses a VecDeque
/// internally.
pub struct QueuedRuntime {
    queue: RefCell<VecDeque<(BoxFuture<'static, ()>, u64)>>,
    counter: u64,
}

impl QueuedRuntime {
    /// Creates a new, empty QueuedRuntime. Awaiting this does nothing unless futures are pushed to
    /// it.
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(VecDeque::new()),
            counter: 0,
        }
    }

    /// Creates a new QueuedRuntime. Unlike new(), this adds a single future immediately, so
    /// awaiting this will have an effect.
    pub fn new_with_boxed(future: BoxFuture<'static, ()>) -> Self {
        let mut r = Self::new();
        Runtime::push_boxed(&mut r, future);
        r
    }
    /// Creates a new QueuedRuntime. Unlike new(), this adds a single future immediately, so
    /// awaiting this will have an effect.
    pub fn new_with(future: impl Future<Output = ()> + 'static) -> Self {
        let mut r = Self::new();
        r.push(future);
        r
    }
}

impl Default for QueuedRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for QueuedRuntime {
    type Output = ();

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let me = self.get_mut();
        if me.counter == u64::MAX {
            return Poll::Ready(());
        }
        set_current_runtime(me);
        let mut all_pending = true;
        let mut i = 0;
        // SAFETY: The queue *must* only be accessed from the thread that is executing the runtime,
        // because the runtime does not implement Send. This makes these borrows necessarily
        // exclusive.
        let r = loop {
            let mut q = me.queue.borrow_mut();
            let Some(mut future) = q.pop_front() else { break Poll::Ready(()) };
            mem::drop(q);
            if future.0.as_mut().poll(cx).is_pending() {
                me.queue.borrow_mut().push_back(future);
            } else {
                all_pending = false;
            }
            if me.counter == u64::MAX {
                break Poll::Ready(());
            }
            i += 1;
            // if queue was traversed with no progress made, stop
            if i >= me.queue.borrow().len() {
                if all_pending {
                    break Poll::Pending;
                }
                all_pending = true;
                i = 0;
            }
        };
        clear_current_runtime();
        r
    }
}

impl InternalRuntime for QueuedRuntime {
    fn push_boxed(&mut self, future: BoxFuture<'static, ()>) -> u64 {
        if self.counter == u64::MAX {
            return self.counter;
        }
        self.counter += 1;
        self.queue.borrow_mut().push_back((future, self.counter));
        self.counter
    }

    fn contains(&mut self, id: u64) -> bool {
        self.queue.borrow().iter().any(|x| x.1 == id)
    }

    fn sleep<'b>(&self, duration: Duration) -> BoxFuture<'b, ()> {
        Box::pin(crate::wait(duration))
    }

    fn stop(&mut self) -> Stop {
        self.counter = u64::MAX;
        Stop
    }
}

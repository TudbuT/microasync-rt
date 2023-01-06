extern crate alloc;
#[cfg(not(feature = "no_std"))]
extern crate std;

use core::mem;
use core::ops::Deref;
use core::ptr::null_mut;
use core::{cell::RefCell, future::Future, pin::Pin, task::Poll};

use alloc::collections::VecDeque;
use microasync::{prep, BoxFuture};

struct ForceSync<T>(T);

unsafe impl<T> Send for ForceSync<T> {}
unsafe impl<T> Sync for ForceSync<T> {}
impl<T> Deref for ForceSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "no_std")]
// SAFETY: We can ForceSync this because we assume no_std means we won't do threading.
static CURRENT_RUNTIME: ForceSync<RefCell<*mut QueuedRuntime>> =
    ForceSync(RefCell::new(null_mut()));

#[cfg(not(feature = "no_std"))]
std::thread_local! {
    static CURRENT_RUNTIME: RefCell<*mut QueuedRuntime> = RefCell::new(null_mut());
}

/// A very small async runtime, with support for adding more tasks as it runs. This uses a VecDeque
/// internally.
pub struct QueuedRuntime {
    queue: RefCell<VecDeque<BoxFuture<'static, ()>>>,
}

impl QueuedRuntime {
    /// Creates a new, empty QueuedRuntime. Awaiting this does nothing unless futures are pushed to
    /// it.
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(VecDeque::new()),
        }
    }

    /// Creates a new QueuedRuntime. Unlike new(), this adds a single future immediately, so
    /// awaiting this will have an effect.
    pub fn new_with_boxed(future: BoxFuture<'static, ()>) -> Self {
        let mut r = Self::new();
        r.push_boxed(future);
        r
    }
    /// Creates a new QueuedRuntime. Unlike new(), this adds a single future immediately, so
    /// awaiting this will have an effect.
    pub fn new_with(future: impl Future<Output = ()> + 'static) -> Self {
        let mut r = Self::new();
        r.push(future);
        r
    }

    /// Adds a new future to the queue to be completed.
    pub fn push_boxed(&mut self, future: BoxFuture<'static, ()>) -> &mut Self {
        self.queue.borrow_mut().push_back(future);
        self
    }

    /// Adds a new future to the queue to be completed.
    pub fn push(&mut self, future: impl Future<Output = ()> + 'static) -> &mut Self {
        self.queue.borrow_mut().push_back(prep(future));
        self
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
        #[cfg(feature = "no_std")]
        {
            *CURRENT_RUNTIME.borrow_mut() = me as *mut _;
        }
        #[cfg(not(feature = "no_std"))]
        {
            CURRENT_RUNTIME.with(|x| *x.borrow_mut() = me as *mut _);
        }
        let mut all_pending = true;
        let mut i = 0;
        // SAFETY: The queue *must* only be accessed from the thread that is executing the runtime,
        // because the runtime does not implement Send. This makes these borrows necessarily
        // exclusive.
        let r = loop {
            let mut q = me.queue.borrow_mut();
            let Some(mut future) = q.pop_front() else { break Poll::Ready(()) };
            mem::drop(q);
            if future.as_mut().poll(cx).is_pending() {
                me.queue.borrow_mut().push_back(future);
            }
            else {
                all_pending = false;
            }
            i += 1;
            // if queue was traversed with no progress made, stop
            if all_pending && i >= me.queue.borrow().len() {
                break Poll::Pending;
            }
        };
        #[cfg(feature = "no_std")]
        {
            *CURRENT_RUNTIME.borrow_mut() = null_mut();
        }
        #[cfg(not(feature = "no_std"))]
        {
            CURRENT_RUNTIME.with(|x| *x.borrow_mut() = null_mut());
        }
        r
    }
}

#[cfg(feature = "no_std")]
/// This assumes a single-threaded environment. Attempting to use this in a multi-threaded
/// environment is highly unsafe and should NEVER be done.
pub async fn get_current_runtime<'a>() -> &'a mut QueuedRuntime {
    let it = CURRENT_RUNTIME.borrow();
    // SAFETY: CURRENT_RUNTIME *MUST* be set to null when QueuedRuntime finishes a poll, so it
    // *cannot* be freed while this is non-null
    unsafe {
        if let Some(x) = it.as_mut() {
            x
        } else {
            panic!("get_current_runtime MUST only be called from a future running within a QueuedRuntime!")
        }
    }
}

#[cfg(not(feature = "no_std"))]
/// This gets the currently running runtime. PANICS IF IT IS CALLED FROM OUTSIDE THE RUNTIME.
pub async fn get_current_runtime<'a>() -> &'a mut QueuedRuntime {
    let it = CURRENT_RUNTIME.with(|x| *x.borrow());
    // SAFETY: CURRENT_RUNTIME *MUST* be set to null when QueuedRuntime finishes a poll, so it
    // *cannot* be freed while this is non-null
    unsafe {
        if let Some(x) = it.as_mut() {
            x
        } else {
            panic!("get_current_runtime MUST only be called from a future running within a QueuedRuntime!")
        }
    }
}

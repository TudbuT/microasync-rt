extern crate std;

use core::{future::Future, pin::Pin, task::{Context, Poll}};
use std::{sync::mpsc::{Receiver, channel}, thread};

/// A future that runs a function in a new thread without blocking.
pub struct DeferredFuture<Args, T, F>
where
    Args: Send + 'static,
    T: Send + 'static,
    F: (FnOnce(Args) -> T) + Send + 'static,
{
    has_started: bool,
    args: Option<Args>,
    f: Option<F>,
    receiver: Option<Receiver<T>>,
}

impl<Args, T, F> DeferredFuture<Args, T, F>
where
    Args: Send + 'static,
    T: Send + 'static,
    F: (FnOnce(Args) -> T) + Send + 'static,
{
    fn new(f: F, args: Args) -> Self {
        Self {
            has_started: false,
            args: Some(args),
            f: Some(f),
            receiver: None,
        }
    }
}

impl<Args, T, F> Future for DeferredFuture<Args, T, F>
where
    Args: Send + 'static,
    T: Send + 'static,
    F: (FnOnce(Args) -> T) + Send + 'static,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: The pin necessarily lives as long as poll() and is owned, so won't be modified
        let me = unsafe {
            self.get_unchecked_mut()
        };
        // SAFETY: All .take().unwrap() calls can never panic because has_started will only be
        // false if the Options are still present.
        if !me.has_started {
            let (arg_sender, arg_receiver) = channel();
            let (t_sender, t_receiver) = channel();
            let f = me.f.take().unwrap();
            me.receiver = Some(t_receiver);
            arg_sender.send(me.args.take().unwrap()).expect("broken channel");
            thread::spawn(move || {
                let (sender, receiver) = (t_sender, arg_receiver);
                let args = receiver.recv().expect("unable to recv args");
                sender.send(f(args)).expect("DeferredFuture completed, but thread was not ready yet.");
            });
            me.has_started = true;
            return Poll::Pending;
        }
        if let Ok(x) = me.receiver.as_ref().unwrap().try_recv() {
            Poll::Ready(x)
        }
        else {
            Poll::Pending
        }
    }
}

/// Returns a DeferredFuture, which runs a computationally expensive task in a new thread.
pub fn defer<Args, T, F>(f: F, args: Args) -> DeferredFuture<Args, T, F> 
where
    Args: Send + 'static,
    T: Send + 'static,
    F: (FnOnce(Args) -> T) + Send + 'static,
{
    DeferredFuture::new(f, args)
}

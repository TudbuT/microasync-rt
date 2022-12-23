extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{io, net::UdpSocket};

use super::{WriteExactFuture, WriteFuture};

impl<'a> Future for WriteFuture<'a, UdpSocket> {
    type Output = Result<usize, io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = me.0.send(me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        Poll::Ready(r)
    }
}

impl<'a> Future for WriteExactFuture<'a, UdpSocket> {
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let mut n = 0;
        while n != me.1.len() {
            n += match me.0.send(&(me.1)[n..]) {
                Ok(x) => x,
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        Poll::Ready(Ok(()))
    }
}

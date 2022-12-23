extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    io::{self, ErrorKind},
    net::UdpSocket,
};

use super::{ReadExactFuture, ReadFuture};

impl<'a> Future for ReadFuture<'a, UdpSocket> {
    type Output = Result<usize, io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = me.0.recv(me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for ReadExactFuture<'a, UdpSocket> {
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let mut n = 0;
        while n != me.1.len() {
            n += match me.0.recv(&mut (me.1)[n..]) {
                Ok(x) => x,
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        Poll::Ready(Ok(()))
    }
}

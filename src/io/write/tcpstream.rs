extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    io::{self, ErrorKind, Write},
    net::TcpStream,
};

use super::{WriteExactFuture, WriteFuture};

impl<'a> Future for WriteFuture<'a, TcpStream> {
    type Output = Result<usize, io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = Write::write(me.0, me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for WriteExactFuture<'a, TcpStream> {
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = Write::write_all(me.0, me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

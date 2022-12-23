extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    io::{self, ErrorKind, Read},
    net::{SocketAddr, TcpListener, TcpStream},
};

use super::{ReadExactFuture, ReadFuture};

impl<'a> Future for ReadFuture<'a, TcpStream> {
    type Output = Result<usize, io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = Read::read(me.0, me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for ReadExactFuture<'a, TcpStream> {
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = Read::read_exact(me.0, me.1);
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

pub struct AcceptFuture<'a>(&'a mut TcpListener);

impl<'a> Future for AcceptFuture<'a> {
    type Output = Result<(TcpStream, SocketAddr), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        me.0.set_nonblocking(true)
            .expect("unable to set nonblocking-mode.");
        let r = me.0.accept();
        me.0.set_nonblocking(false)
            .expect("unable to clear nonblocking-mode.");

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

/// Returns an AcceptFuture, which is an async TcpListener::accept call.
pub fn accept(listener: &mut TcpListener) -> AcceptFuture {
    AcceptFuture(listener)
}

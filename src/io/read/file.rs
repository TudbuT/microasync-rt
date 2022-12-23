extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use super::{ReadExactFuture, ReadFuture};

impl<'a> Future for ReadFuture<'a, File> {
    type Output = Result<usize, io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        let r = Read::read(me.0, me.1);

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for ReadExactFuture<'a, File> {
    type Output = Result<(), io::Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        fn left(f: &mut File) -> Result<u64, io::Error> {
            let cur = f.seek(SeekFrom::Current(0))?;
            let end = f.seek(SeekFrom::End(0))?;
            if cur != end {
                f.seek(SeekFrom::Start(cur))?;
            }
            Ok(end - cur)
        }

        let me = self.get_mut();
        match left(me.0) {
            Ok(left) => {
                if left < me.1.len() as u64 {
                    return Poll::Pending;
                }
            }
            Err(x) => return Poll::Ready(Err(x)),
        }
        let r = Read::read_exact(me.0, me.1);

        match r {
            Ok(x) => Poll::Ready(Ok(x)),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

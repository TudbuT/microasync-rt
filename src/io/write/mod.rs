extern crate std;

use core::future::Future;
use std::io;

pub struct WriteFuture<'a, T>(&'a mut T, &'a [u8]);

pub struct WriteExactFuture<'a, T>(&'a mut T, &'a [u8]);

/// Trait that adds async variants of some std::io::Write functions.
pub trait WriteAsync<'a, T, FWrite, FWriteExact>
where
    FWrite: Future<Output = Result<usize, io::Error>> + 'a,
{
    /// Async equivalent to std::io::Write::write.
    fn write(&'a mut self, bytes: &'a [u8]) -> FWrite;

    /// Async equivalent to std::io::Write::write_exact. When using UDP, this may send multiple
    /// packets.
    fn write_exact(&'a mut self, bytes: &'a [u8]) -> FWriteExact;
}

impl<'a, T> WriteAsync<'a, usize, WriteFuture<'a, T>, WriteExactFuture<'a, T>> for T
where
    WriteFuture<'a, T>: Future<Output = Result<usize, io::Error>> + 'a,
    WriteExactFuture<'a, T>: Future<Output = Result<(), io::Error>> + 'a,
{
    fn write(&'a mut self, bytes: &'a [u8]) -> WriteFuture<'a, T> {
        WriteFuture(self, bytes)
    }

    fn write_exact(&'a mut self, bytes: &'a [u8]) -> WriteExactFuture<'a, T> {
        WriteExactFuture(self, bytes)
    }
}

pub mod file;
pub mod tcpstream;
pub mod udpsocket;

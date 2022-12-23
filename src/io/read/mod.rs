extern crate std;

use core::future::Future;
use std::io;

pub struct ReadFuture<'a, T>(&'a mut T, &'a mut [u8]);

pub struct ReadExactFuture<'a, T>(&'a mut T, &'a mut [u8]);

/// Trait that adds async variants of some std::io::Read functions.
pub trait ReadAsync<'a, T, FRead, FReadExact>
where
    FRead: Future<Output = Result<usize, io::Error>> + 'a,
{
    /// Async equivalent to std::io::Read::read.
    fn read(&'a mut self, bytes: &'a mut [u8]) -> FRead;

    /// Async equivalent to std::io::Read::read_exact. This MAY read more bytes than the length of
    /// the array if needed (namely, when using UdpSockets), but will avoid doing so to the best of its ability.
    fn read_exact(&'a mut self, bytes: &'a mut [u8]) -> FReadExact;
}

impl<'a, T> ReadAsync<'a, usize, ReadFuture<'a, T>, ReadExactFuture<'a, T>> for T
where
    ReadFuture<'a, T>: Future<Output = Result<usize, io::Error>> + 'a,
    ReadExactFuture<'a, T>: Future<Output = Result<(), io::Error>> + 'a,
{
    fn read(&'a mut self, bytes: &'a mut [u8]) -> ReadFuture<'a, T> {
        ReadFuture(self, bytes)
    }

    fn read_exact(&'a mut self, bytes: &'a mut [u8]) -> ReadExactFuture<'a, T> {
        ReadExactFuture(self, bytes)
    }
}

pub mod file;
pub mod tcpstream;
pub mod udpsocket;

use tokio::prelude::*;
use tokio::io;

pub struct ReadU8<R> {
    reader: io::ReadExact<R, [u8; 1]>,
}

impl<R> ReadU8<R>
where
    R: AsyncRead,
{
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader: io::read_exact(reader, [0; 1]),
        }
    }
}

impl<R> Future for ReadU8<R>
where
    R: AsyncRead,
{
    type Item = (R, u8);
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(reader, buf)| {
                (reader, buf[0])
            })
        )
    }
}

pub struct ReadI8<R> {
    reader: io::ReadExact<R, [u8; 1]>,
}

impl<R> ReadI8<R>
where
    R: AsyncRead,
{
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader: io::read_exact(reader, [0; 1]),
        }
    }
}

impl<R> Future for ReadI8<R>
where
    R: AsyncRead,
{
    type Item = (R, i8);
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(reader, buf)| {
                (reader, buf[0] as i8)
            })
        )
    }
}

pub struct WriteU8<W> {
    writer: io::WriteAll<W, [u8; 1]>,
}

impl<W> WriteU8<W>
where
    W: AsyncWrite,
{
    pub(crate) fn new(writer: W, n: u8) -> Self {
        Self {
            writer: io::write_all(writer, [n]),
        }
    }
}

impl<W> Future for WriteU8<W>
where
    W: AsyncWrite,
{
    type Item = W;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.writer.poll().map(|res|
            res.map(|(writer, _)| writer)
        )
    }
}

pub struct WriteI8<W> {
    writer: io::WriteAll<W, [u8; 1]>,
}

impl<W> WriteI8<W>
where
    W: AsyncWrite,
{
    pub(crate) fn new(writer: W, n: i8) -> Self {
        Self {
            writer: io::write_all(writer, [n as u8]),
        }
    }
}

impl<W> Future for WriteI8<W>
where
    W: AsyncWrite,
{
    type Item = W;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.writer.poll().map(|res|
            res.map(|(writer, _)| writer)
        )
    }
}


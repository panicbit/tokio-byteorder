use tokio::prelude::*;
use tokio::io;
use byteorder::ByteOrder;
use std::marker::PhantomData;

pub struct ReadUInt<B, R> {
    reader: io::ReadExact<R, [u8; 8]>,
    nbytes: usize,
    _byteorder: PhantomData<B>,
}

impl<B, R> ReadUInt<B, R>
where
    R: AsyncRead,
{
    pub(crate) fn new(reader: R, nbytes: usize) -> Self {
        Self {
            reader: io::read_exact(reader, [0; 8]),
            nbytes,
            _byteorder: PhantomData,
        }
    }
}

impl<B, R> Future for ReadUInt<B, R>
where
    B: ByteOrder,
    R: AsyncRead,
{
    type Item = (R, u64);
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(reader, buf)| {
                (reader, B::read_uint(&buf, self.nbytes))
            })
        )
    }
}

pub struct ReadInt<B, R> {
    reader: io::ReadExact<R, [u8; 8]>,
    nbytes: usize,
    _byteorder: PhantomData<B>,
}

impl<B, R> ReadInt<B, R>
where
    R: AsyncRead,
{
    pub(crate) fn new(reader: R, nbytes: usize) -> Self {
        Self {
            reader: io::read_exact(reader, [0; 8]),
            nbytes,
            _byteorder: PhantomData,
        }
    }
}

impl<B, R> Future for ReadInt<B, R>
where
    B: ByteOrder,
    R: AsyncRead,
{
    type Item = (R, i64);
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(reader, buf)| {
                (reader, B::read_int(&buf, self.nbytes))
            })
        )
    }
}

pub struct WriteUInt<W> {
    reader: io::WriteAll<W, [u8; 8]>,
}

impl<W> WriteUInt<W>
where
    W: AsyncWrite,
{
    pub(crate) fn new<B>(writer: W, n: u64, nbytes: usize) -> Self
    where
        B: ByteOrder,
    {
        let mut buf = [0; 8];
        B::write_uint(&mut buf, n, nbytes);
        
        Self {
            reader: io::write_all(writer, buf),
        }
    }
}

impl<W> Future for WriteUInt<W>
where
    W: AsyncWrite,
{
    type Item = W;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(writer, _)| writer)
        )
    }
}

pub struct WriteInt<W> {
    reader: io::WriteAll<W, [u8; 8]>,
}

impl<W> WriteInt<W>
where
    W: AsyncWrite,
{
    pub(crate) fn new<B>(writer: W, n: i64, nbytes: usize) -> Self
    where
        B: ByteOrder,
    {
        let mut buf = [0; 8];
        B::write_int(&mut buf, n, nbytes);
        
        Self {
            reader: io::write_all(writer, buf),
        }
    }
}

impl<W> Future for WriteInt<W>
where
    W: AsyncWrite,
{
    type Item = W;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.reader.poll().map(|res|
            res.map(|(writer, _)| writer)
        )
    }
}
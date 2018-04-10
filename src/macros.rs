
macro_rules! impl_read {
    ($name:ident, $fun:ident, $int:ty, $size:expr) => {
        pub struct $name<B, R> {
            reader: tokio::io::ReadExact<R, [u8; $size]>,
            _byteorder: std::marker::PhantomData<B>,
        }

        impl<B, R> $name<B, R>
        where
            R: AsyncRead,
        {
            fn new(reader: R) -> Self {
                Self {
                    reader: tokio::io::read_exact(reader, [0; $size]),
                    _byteorder: std::marker::PhantomData,
                }
            }
        }

        impl<B, R> tokio::prelude::Future for $name<B, R>
        where
            B: ByteOrder,
            R: AsyncRead,
        {
            type Item = (R, $int);
            type Error = tokio::io::Error;

            fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
                self.reader.poll().map(|res|
                    res.map(|(reader, buf)| {
                        (reader, B::$fun(&buf))
                    })
                )
            }
        }
    }
}

macro_rules! impl_write {
    ($name:ident, $fun:ident, $int:ty, $size:expr) => {
        pub struct $name<W> {
            writer: tokio::io::WriteAll<W, [u8; $size]>,
        }

        impl<W> $name<W>
        where
            W: AsyncWrite,
        {
            fn new<B>(writer: W, n: $int) -> Self
            where
                B: byteorder::ByteOrder,
            {
                let mut buf = [0; $size];
                B::$fun(&mut buf, n);

                Self {
                    writer: tokio::io::write_all(writer, buf),
                }
            }
        }

        impl<W> tokio::prelude::Future for $name<W>
        where
            W: AsyncWrite,
        {
            type Item = W;
            type Error = tokio::io::Error;

            fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
                self.writer.poll().map(|res|
                    res.map(|(writer, _)| writer)
                )
            }
        }
    }
}

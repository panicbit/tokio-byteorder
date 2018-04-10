extern crate tokio;
extern crate byteorder;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::prelude::Async;

pub use byteorder::{ByteOrder, BigEndian, LittleEndian, BE, LE};
pub use self::byte::{ReadU8, ReadI8, WriteU8, WriteI8};
pub use self::int::{ReadUInt, ReadInt, WriteUInt, WriteInt};

#[macro_use]
mod macros;
mod byte;
mod int;

impl_read!(ReadU16, read_u16, u16, 2);
impl_read!(ReadI16, read_i16, i16, 2);

impl_read!(ReadU24, read_u24, u32, 3);
impl_read!(ReadI24, read_i24, i32, 3);

impl_read!(ReadU32, read_u32, u32, 4);
impl_read!(ReadI32, read_i32, i32, 4);

impl_read!(ReadU64, read_u64, u64, 8);
impl_read!(ReadI64, read_i64, i64, 8);

impl_read!(ReadF32, read_f32, f32, 4);
impl_read!(ReadF64, read_f64, f64, 8);

pub trait ReadBytesExt: Sized + AsyncRead {
    fn read_u8(self) -> ReadU8<Self> { ReadU8::new(self) }
    fn read_i8(self) -> ReadI8<Self> { ReadI8::new(self) }

    fn read_u16<B: ByteOrder>(self) -> ReadU16<B, Self> { ReadU16::new(self) }
    fn read_i16<B: ByteOrder>(self) -> ReadI16<B, Self> { ReadI16::new(self) }

    fn read_u24<B: ByteOrder>(self) -> ReadU24<B, Self> { ReadU24::new(self) }
    fn read_i24<B: ByteOrder>(self) -> ReadI24<B, Self> { ReadI24::new(self) }

    fn read_u32<B: ByteOrder>(self) -> ReadU32<B, Self> { ReadU32::new(self) }
    fn read_i32<B: ByteOrder>(self) -> ReadI32<B, Self> { ReadI32::new(self) }

    fn read_u64<B: ByteOrder>(self) -> ReadU64<B, Self> { ReadU64::new(self) }
    fn read_i64<B: ByteOrder>(self) -> ReadI64<B, Self> { ReadI64::new(self) }

    fn read_f32<B: ByteOrder>(self) -> ReadF32<B, Self> { ReadF32::new(self) }
    fn read_f64<B: ByteOrder>(self) -> ReadF64<B, Self> { ReadF64::new(self) }

    fn read_int<B: ByteOrder>(self, nbytes: usize) -> ReadInt<B, Self> { ReadInt::new(self, nbytes) }
    fn read_uint<B: ByteOrder>(self, nbytes: usize) -> ReadUInt<B, Self> { ReadUInt::new(self, nbytes) }
}

impl_write!(WriteU16, write_u16, u16, 2);
impl_write!(WriteI16, write_i16, i16, 2);

impl_write!(WriteU24, write_u24, u32, 3);
impl_write!(WriteI24, write_i24, i32, 3);

impl_write!(WriteU32, write_u32, u32, 4);
impl_write!(WriteI32, write_i32, i32, 4);

impl_write!(WriteU64, write_u64, u64, 8);
impl_write!(WriteI64, write_i64, i64, 8);

impl_write!(WriteF32, write_f32, f32, 4);
impl_write!(WriteF64, write_f64, f64, 8);

pub trait WriteBytesExt: Sized + AsyncWrite {
    fn write_u8(self, n: u8) -> WriteU8<Self> { WriteU8::new(self, n) }
    fn write_i8(self, n: i8) -> WriteI8<Self> { WriteI8::new(self, n) }

    fn write_u16<B: ByteOrder>(self, n: u16) -> WriteU16<Self> { WriteU16::new::<B>(self, n) }
    fn write_i16<B: ByteOrder>(self, n: i16) -> WriteI16<Self> { WriteI16::new::<B>(self, n) }

    fn write_u24<B: ByteOrder>(self, n: u32) -> WriteU24<Self> { WriteU24::new::<B>(self, n) }
    fn write_i24<B: ByteOrder>(self, n: i32) -> WriteI24<Self> { WriteI24::new::<B>(self, n) }

    fn write_u32<B: ByteOrder>(self, n: u32) -> WriteU32<Self> { WriteU32::new::<B>(self, n) }
    fn write_i32<B: ByteOrder>(self, n: i32) -> WriteI32<Self> { WriteI32::new::<B>(self, n) }

    fn write_u64<B: ByteOrder>(self, n: u64) -> WriteU64<Self> { WriteU64::new::<B>(self, n) }
    fn write_i64<B: ByteOrder>(self, n: i64) -> WriteI64<Self> { WriteI64::new::<B>(self, n) }

    fn write_f32<B: ByteOrder>(self, n: f32) -> WriteF32<Self> { WriteF32::new::<B>(self, n) }
    fn write_f64<B: ByteOrder>(self, n: f64) -> WriteF64<Self> { WriteF64::new::<B>(self, n) }

    fn write_int<B: ByteOrder>(self, n: i64, nbytes: usize) -> WriteInt<Self> { WriteInt::new::<B>(self, n, nbytes) }
    fn write_uint<B: ByteOrder>(self, n: u64, nbytes: usize) -> WriteUInt<Self> { WriteUInt::new::<B>(self, n, nbytes) }
}

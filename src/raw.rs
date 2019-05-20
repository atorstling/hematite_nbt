//! Primitive functions for serializing and deserializing NBT data.

// (\w+)\.(\w+)::<BigEndian>
// ByteOrdered::runtime($1, Endianness::Big).$2

use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use byteordered::{ByteOrdered, Endianness};

use error::{Error, Result};

/// A convenience function for closing NBT format objects.
///
/// This function writes a single `0x00` byte to the `io::Write` destination,
/// which in the NBT format indicates that an open Compound is now closed.
pub fn close_nbt<W>(dst: &mut W) -> Result<()>
    where W: io::Write {

    dst.write_u8(0x00).map_err(From::from)
}

#[inline]
pub fn write_bare_byte<W>(dst: &mut W, value: i8) -> Result<()>
   where W: io::Write
{
    dst.write_i8(value).map_err(From::from)
}

#[inline]
pub fn write_bare_short<W>(dst: &mut W, value: i16) -> Result<()>
   where W: io::Write
{
    ByteOrdered::runtime(dst, Endianness::Big).write_i16(value).map_err(From::from)
}

#[inline]
pub fn write_bare_int<W>(dst: &mut W, value: i32) -> Result<()>
   where W: io::Write
{
    ByteOrdered::runtime(dst, Endianness::Big).write_i32(value).map_err(From::from)
}

#[inline]
pub fn write_bare_long<W>(dst: &mut W, value: i64) -> Result<()>
   where W: io::Write
{
    ByteOrdered::runtime(dst, Endianness::Big).write_i64(value).map_err(From::from)
}

#[inline]
pub fn write_bare_float<W>(dst: &mut W, value: f32) -> Result<()>
   where W: io::Write
{
    ByteOrdered::runtime(dst, Endianness::Big).write_f32(value).map_err(From::from)
}

#[inline]
pub fn write_bare_double<W>(dst: &mut W, value: f64) -> Result<()>
   where W: io::Write
{
    ByteOrdered::runtime(dst, Endianness::Big).write_f64(value).map_err(From::from)
}

#[inline]
pub fn write_bare_byte_array<W>(dst: &mut W, value: &[i8]) -> Result<()>
   where W: io::Write
{
    let mut bo = ByteOrdered::runtime(dst, Endianness::Big);
    try!(bo.write_i32(value.len() as i32));
    for &v in value {
        try!(bo.write_i8(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_int_array<W>(dst: &mut W, value: &[i32]) -> Result<()>
   where W: io::Write
{
    let mut bo = ByteOrdered::runtime(dst, Endianness::Big);
    try!(bo.write_i32(value.len() as i32));
    for &v in value {
        try!(bo.write_i32(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_long_array<W>(dst: &mut W, value: &[i64]) -> Result<()>
   where W: io::Write
{
    let mut bo = ByteOrdered::runtime(dst, Endianness::Big);
    bo.write_i32(value.len() as i32)?;
    for &v in value {
        bo.write_i64(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_string<W>(dst: &mut W, value: &str) -> Result<()>
   where W: io::Write
{
    let mut bo: ByteOrdered<&mut W, Endianness> = ByteOrdered::runtime(dst, Endianness::Big);
    try!(bo.write_u16(value.len() as u16));
    bo.inner_mut().write_all(value.as_bytes()).map_err(From::from)
}

/// Extracts the next header (tag and name) from an NBT format source.
///
/// This function will also return the `TAG_End` byte and an empty name if it
/// encounters it.
pub fn emit_next_header<R>(src: &mut R) -> Result<(u8, String)>
    where R: io::Read
{
    let tag  = try!(src.read_u8());

    match tag {
        0x00 => { Ok((tag, "".to_string())) },
        _    => {
            let name = try!(read_bare_string(src));
            Ok((tag, name))
        },
    }
}

#[inline]
pub fn read_bare_byte<R>(src: &mut R) -> Result<i8>
    where R: io::Read
{
    src.read_i8().map_err(From::from)
}

#[inline]
pub fn read_bare_short<R>(src: &mut R) -> Result<i16>
    where R: io::Read
{
    ByteOrdered::runtime(src, Endianness::Big).read_i16().map_err(From::from)
}

#[inline]
pub fn read_bare_int<R>(src: &mut R) -> Result<i32>
    where R: io::Read
{
    ByteOrdered::runtime(src, Endianness::Big).read_i32().map_err(From::from)
}

#[inline]
pub fn read_bare_long<R>(src: &mut R) -> Result<i64>
    where R: io::Read
{
    ByteOrdered::runtime(src, Endianness::Big).read_i64().map_err(From::from)
}

#[inline]
pub fn read_bare_float<R>(src: &mut R) -> Result<f32>
    where R: io::Read
{
    ByteOrdered::runtime(src, Endianness::Big).read_f32().map_err(From::from)
}

#[inline]
pub fn read_bare_double<R>(src: &mut R) -> Result<f64>
    where R: io::Read
{
    ByteOrdered::runtime(src, Endianness::Big).read_f64().map_err(From::from)
}

#[inline]
pub fn read_bare_byte_array<R>(src: &mut R) -> Result<Vec<i8>>
    where R: io::Read
{
    // FIXME: Is there a way to return [u8; len]?
    let mut bo = ByteOrdered::runtime(src, Endianness::Big);
    let len = try!(bo.read_i32()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(bo.read_i8()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_int_array<R>(src: &mut R) -> Result<Vec<i32>>
    where R: io::Read
{
    // FIXME: Is there a way to return [i32; len]?
    let mut bo = ByteOrdered::runtime(src, Endianness::Big);
    let len = try!(bo.read_i32()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(bo.read_i32()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_long_array<R>(src: &mut R) -> Result<Vec<i64>>
    where R: io::Read
{
    let mut bo = ByteOrdered::runtime(src, Endianness::Big);
    let len = bo.read_i32()? as usize;
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        buf.push(bo.read_i64()?);
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_string<R>(src: &mut R) -> Result<String>
    where R: io::Read
{
    let mut bo = ByteOrdered::runtime(src, Endianness::Big);
    let len = try!(bo.read_u16()) as usize;

    if len == 0 { return Ok("".to_string()); }

    let mut bytes = vec![0; len];
    let mut n_read = 0usize;
    while n_read < bytes.len() {
        match try!(bo.inner_mut().read(&mut bytes[n_read..])) {
            0 => return Err(Error::IncompleteNbtValue),
            n => n_read += n
        }
    }

    String::from_utf8(bytes).map_err(From::from)
}

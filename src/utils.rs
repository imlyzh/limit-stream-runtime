use rmp::{
    decode::{read_array_len, read_marker},
    encode::{write_array_len, write_sint, write_str, write_uint},
    Marker::*,
};

use crate::{
    builtin_type::{Int, Uint},
    Deser, Ser,
};
use rmp::decode::RmpRead;

pub type Bytes<'a> = rmp::decode::Bytes<'a>;
pub type ByteBuf = rmp::encode::buffer::ByteBuf;

/// write

pub fn ls_write_value<T: Ser>(wr: &mut ByteBuf, i: T) -> Result<(), ()> {
    i.ser(wr)
}

pub fn ls_write_array_len(wr: &mut ByteBuf, len: u32) -> Result<(), ()> {
    write_array_len(wr, len).map_err(|_| ())?;
    Ok(())
}

#[inline]
pub fn ls_write_int(wr: &mut ByteBuf, val: &Int) -> Result<(), ()> {
    write_sint(wr, *val).map_err(|_| ())?;
    Ok(())
}

#[inline]
pub fn ls_write_uint(wr: &mut ByteBuf, val: &Uint) -> Result<(), ()> {
    write_uint(wr, *val).map_err(|_| ())?;
    Ok(())
}

#[inline]
pub fn ls_write_str(wr: &mut ByteBuf, val: &str) -> Result<(), ()> {
    write_str(wr, val).map_err(|_| ())
}

/// read

pub fn ls_read_value<T: Deser<Res = Result<T, ()>>>(wr: &mut Bytes) -> Result<T, ()> {
    T::deser(wr)
}

pub fn ls_read_array_len(rd: &mut Bytes) -> Result<u32, ()> {
    read_array_len(rd).map_err(|_| ())
}

#[inline]
pub fn ls_read_int(rd: &mut Bytes) -> Result<Int, ()> {
    let m = read_marker(rd).map_err(|_| ())?;
    let r = match m {
        FixNeg(val) => val as Int,
        I8 => rd.read_data_i8().map_err(|_| ())? as Int,
        I16 => rd.read_data_i16().map_err(|_| ())? as Int,
        I32 => rd.read_data_i32().map_err(|_| ())? as Int,
        I64 => rd.read_data_i64().map_err(|_| ())? as Int,
        _ => return Err(()),
    };
    Ok(r)
}

#[inline]
pub fn ls_read_uint(rd: &mut Bytes) -> Result<Uint, ()> {
    let m = read_marker(rd).map_err(|_| ())?;
    let r = match m {
        FixPos(val) => val as Uint,
        U8 => rd.read_data_u8().map_err(|_| ())? as Uint,
        U16 => rd.read_data_u16().map_err(|_| ())? as Uint,
        U32 => rd.read_data_u32().map_err(|_| ())? as Uint,
        U64 => rd.read_data_u64().map_err(|_| ())? as Uint,
        _ => return Err(()),
    };
    Ok(r)
}

#[inline]
pub fn ls_read_str(rd: &mut Bytes) -> Result<String, ()> {
    let m = read_marker(rd).map_err(|_| ())?;
    let r = match m {
        FixStr(size) => {
            let size = size as usize;
            let str = (0..size)
                .map(|_| rd.read_u8().map_err(|_| ()))
                .collect::<Result<Vec<u8>, _>>()?;
            String::from_utf8(str).map_err(|_| ())?
        }
        Str8 => {
            let size = rd.read_data_u8().map_err(|_| ())? as usize;
            let str = (0..size)
                .map(|_| rd.read_u8().map_err(|_| ()))
                .collect::<Result<Vec<u8>, _>>()?;
            String::from_utf8(str).map_err(|_| ())?
        }
        Str16 => {
            let size = rd.read_data_u16().map_err(|_| ())? as usize;
            let str = (0..size)
                .map(|_| rd.read_u8().map_err(|_| ()))
                .collect::<Result<Vec<u8>, _>>()?;
            String::from_utf8(str).map_err(|_| ())?
        }
        Str32 => {
            let size = rd.read_data_u32().map_err(|_| ())? as usize;
            let str = (0..size)
                .map(|_| rd.read_u8().map_err(|_| ()))
                .collect::<Result<Vec<u8>, _>>()?;
            String::from_utf8(str).map_err(|_| ())?
        }
        _ => return Err(()),
    };
    Ok(r)
}

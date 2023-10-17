use rmp::{
    decode::{read_i64, read_marker, read_u64, Bytes, RmpRead},
    encode::{write_i64, write_u64, ByteBuf},
    Marker::*,
};

use crate::builtin_type::{Int, Uint};

#[inline]
pub fn ls_write_int(buf: &mut ByteBuf, val: &Int) -> Result<(), ()> {
    // fixme
    write_i64(buf, *val).map_err(|_| ())?;
    Ok(())
}

#[inline]
pub fn ls_write_uint(buf: &mut ByteBuf, val: &Uint) -> Result<(), ()> {
    // fixme
    write_u64(buf, *val).map_err(|_| ())?;
    Ok(())
}

#[inline]
pub fn ls_read_int(buf: &mut Bytes) -> Result<Int, ()> {
    // fixme
    read_i64(buf).map_err(|_| ())
}

#[inline]
pub fn ls_read_uint(buf: &mut Bytes) -> Result<Uint, ()> {
    // fixme
    read_u64(buf).map_err(|_| ())
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

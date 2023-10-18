use rmp::{decode::read_bool, encode::write_bool};

use crate::{
    utils::{ls_read_int, ls_read_uint, ls_write_int, ls_write_uint},
    Deser, Ser,
};

pub type Bool = bool;
pub type Int = i64;
pub type Uint = u64;
pub type Float = f64;
// pub type Float = f32;
// pub type Double = f64;

impl Ser for Bool {
    fn ser(&self, buf: &mut crate::utils::ByteBuf) -> Result<(), ()> {
        write_bool(buf, *self).map_err(|_| ())
    }
}

impl Deser for Bool {
    type Res = Result<Self, ()>;

    fn deser(rd: &mut crate::utils::Bytes) -> Self::Res {
        read_bool(rd).map_err(|_| ())
    }
}

impl Ser for Int {
    fn ser(&self, buf: &mut crate::utils::ByteBuf) -> Result<(), ()> {
        ls_write_int(buf, self)
    }
}

impl Deser for Int {
    type Res = Result<Self, ()>;

    fn deser(rd: &mut crate::utils::Bytes) -> Self::Res {
        ls_read_int(rd)
    }
}

impl Ser for Uint {
    fn ser(&self, buf: &mut crate::utils::ByteBuf) -> Result<(), ()> {
        ls_write_uint(buf, self)
    }
}

impl Deser for Uint {
    type Res = Result<Self, ()>;

    fn deser(rd: &mut crate::utils::Bytes) -> Self::Res {
        ls_read_uint(rd)
    }
}

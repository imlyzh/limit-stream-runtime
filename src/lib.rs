use utils::{ByteBuf, Bytes};

pub mod builtin_type;
pub mod utils;

pub trait Ser {
    fn ser(&self, buf: &mut ByteBuf) -> Result<(), ()>;
}

pub trait Deser {
    type Res;
    fn deser(i: &mut Bytes) -> Self::Res;
}

pub mod builtin_type;
pub mod channel;
pub mod utils;

pub use builtin_type::*;
pub use channel::*;
pub use utils::*;

pub trait Ser {
    fn ser(&self, buf: &mut ByteBuf) -> Result<(), ()>;
}

pub trait Deser {
    type Res;
    fn deser(i: &mut Bytes) -> Self::Res;
}

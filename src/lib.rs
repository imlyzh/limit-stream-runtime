pub mod builtin_type;
pub mod utils;

pub trait Serialize<T, E> {
    fn serialize(&self) -> Result<T, E>;
}

pub trait Deserialize<T> {
    type Res;
    fn deserialize(i: T) -> Self::Res;
}

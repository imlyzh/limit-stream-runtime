use limit_stream_runtime::builtin_type::*;
use limit_stream_runtime::utils::{ls_read_str, ls_read_uint, ls_write_uint};
use limit_stream_runtime::{Deserialize, Serialize};
use rmp::decode::read_array_len;
use rmp::{
    decode::Bytes,
    encode::{buffer::ByteBuf, write_array_len, write_str},
};

#[derive(Debug, PartialEq, Default)]
pub struct User {
    pub name: String,
    pub age: Uint,
    pub description: String,
}

impl Serialize<Vec<u8>, ()> for User {
    fn serialize(&self) -> Result<Vec<u8>, ()> {
        let mut buf = ByteBuf::new();
        write_array_len(&mut buf, 3).map_err(|_| ())?;

        write_str(&mut buf, &self.name).map_err(|_| ())?;

        ls_write_uint(&mut buf, &self.age).map_err(|_| ())?;

        write_str(&mut buf, &self.description).map_err(|_| ())?;

        Ok(buf.into_vec())
    }
}

impl Deserialize<&[u8]> for User {
    type Res = Result<Self, ()>;
    fn deserialize(i: &[u8]) -> Result<Self, ()> {
        let mut buf = Bytes::new(i);
        if read_array_len(&mut buf).map_err(|_| ())? != 3 {
            return Err(());
        }
        // #[allow(invalid_value)]
        // let mut value = unsafe { MaybeUninit::<User>::uninit().assume_init() };
        let mut value = User::default();

        value.name = ls_read_str(&mut buf)?;

        value.age = ls_read_uint(&mut buf)?;

        value.description = ls_read_str(&mut buf)?;

        Ok(value)
    }
}

fn main() {
    let src = User {
        name: "runner".to_string(),
        age: 24,
        description: "runner1".to_string(),
    };
    let channel = src.serialize().unwrap();
    let dst = User::deserialize(&channel).unwrap();
    assert_eq!(dst, src);
}

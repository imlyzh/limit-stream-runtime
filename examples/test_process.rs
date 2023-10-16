
use limit_stream_runtime::*;
use limit_stream_runtime::builtin_type::*;
use rmp::{encode::{buffer::ByteBuf, write_str, write_u64, write_array_len}, decode::{Bytes, read_marker}};



pub struct User {
    pub name: String,
    pub age: Uint,
    pub description: String,
}

impl Serialize<Vec<u8>, ()> for User {
    fn serialize(&self) -> Result<Vec<u8>, ()> {
        let mut buf = ByteBuf::new();
        write_array_len(&mut buf, 3).map_err(|_| ())?;
        write_str(&mut buf, &self.name).map_err(|_|())?;
        write_u64(&mut buf, self.age).map_err(|_|())?;
        Ok(buf.into_vec())
    }
}

impl Deserialize<&[u8]> for User {
    type Res = Result<Self, ()>;
    fn deserialize(i: &[u8]) -> Result<Self, ()> {
        let mut buf = Bytes::new(i);
        // if let Some() = read_marker(&mut buf).map_err(|_| ())? {
        // }
        todo!()
    }
}

fn main() {

}
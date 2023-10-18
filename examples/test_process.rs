use limit_stream_runtime::builtin_type::*;
use limit_stream_runtime::utils::{
    ls_read_array_len, ls_read_str, ls_read_uint, ls_write_array_len, ls_write_str, ls_write_uint,
    ByteBuf, Bytes,
};
use limit_stream_runtime::{Deser, Ser};

#[derive(Debug, PartialEq, Default)]
pub struct User {
    pub name: String,
    pub age: Uint,
    pub description: String,
}

impl Ser for User {
    fn ser(&self, buf: &mut ByteBuf) -> Result<(), ()> {
        ls_write_array_len(buf, 3)?;

        ls_write_str(buf, &self.name)?;

        ls_write_uint(buf, &self.age)?;

        ls_write_str(buf, &self.description)?;

        Ok(())
    }
}

impl Deser for User {
    type Res = Result<Self, ()>;
    fn deser(buf: &mut Bytes) -> Result<Self, ()> {
        if ls_read_array_len(buf)? != 3 {
            return Err(());
        }
        // #[allow(invalid_value)]
        // let mut value = unsafe { MaybeUninit::<User>::uninit().assume_init() };
        let mut value = User::default();

        value.name = ls_read_str(buf)?;

        value.age = ls_read_uint(buf)?;

        value.description = ls_read_str(buf)?;

        Ok(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum UserForm {
    User(User),
    Id(Uint),
}

impl Ser for UserForm {
    fn ser(&self, buf: &mut ByteBuf) -> Result<(), ()> {
        ls_write_array_len(buf, 2)?;

        match self {
            UserForm::User(v) => {
                ls_write_str(buf, "User")?;
                v.ser(buf)?;
            }
            UserForm::Id(v) => {
                ls_write_str(buf, "User")?;
                ls_write_uint(buf, v)?;
            }
        }

        return Ok(());
    }
}

impl Deser for UserForm {
    type Res = Result<Self, ()>;
    fn deser(buf: &mut Bytes) -> Result<Self, ()> {
        if ls_read_array_len(buf)? != 2 {
            return Err(());
        }

        match ls_read_str(buf)?.as_str() {
            "User" => Ok(UserForm::User(User::deser(buf)?)),
            "Id" => Ok(UserForm::Id(ls_read_uint(buf)?)),
            _ => return Err(()),
        }
    }
}

fn main() {
    let src = User {
        name: "runner".to_string(),
        age: 24,
        description: "runner1".to_string(),
    };
    let mut channel = ByteBuf::new();
    src.ser(&mut channel).unwrap();
    let dst = User::deser(&mut Bytes::new(channel.as_slice())).unwrap();
    assert_eq!(dst, src);
}

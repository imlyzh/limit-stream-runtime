// use limit_stream_runtime::builtin_type::*;
// use limit_stream_runtime::utils::{
// ls_read_array_len, ls_read_str, ls_write_array_len, ls_write_str,
// ByteBuf, Bytes,
// };
// use limit_stream_runtime::{Deser, Ser};

#[derive(Debug, PartialEq, Default)]
pub struct User {
    pub name: String,
    pub age: limit_stream_runtime::Uint,
    pub description: String,
}

impl limit_stream_runtime::Ser for User {
    fn ser(&self, buf: &mut limit_stream_runtime::ByteBuf) -> Result<(), ()> {
        limit_stream_runtime::utils::ls_write_array_len(buf, 3)?;

        self.name.ser(buf)?;

        self.age.ser(buf)?;

        self.description.ser(buf)?;

        Ok(())
    }
}

impl limit_stream_runtime::Deser for User {
    type Res = Result<Self, ()>;
    fn deser(buf: &mut limit_stream_runtime::Bytes) -> Result<Self, ()> {
        if limit_stream_runtime::utils::ls_read_array_len(buf)? != 3 {
            return Err(());
        }
        // #[allow(invalid_value)]
        // let mut value = unsafe { MaybeUninit::<User>::uninit().assume_init() };
        let mut value = User::default();

        value.name = limit_stream_runtime::ls_read_value(buf)?;

        value.age = limit_stream_runtime::ls_read_value(buf)?;

        value.description = limit_stream_runtime::ls_read_value(buf)?;

        Ok(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum UserForm {
    User(User),
    Id(limit_stream_runtime::Uint),
}

impl limit_stream_runtime::Ser for UserForm {
    fn ser(&self, buf: &mut limit_stream_runtime::ByteBuf) -> Result<(), ()> {
        limit_stream_runtime::utils::ls_write_array_len(buf, 2)?;

        match self {
            UserForm::User(v) => {
                limit_stream_runtime::ls_write_str(buf, "User")?;
                v.ser(buf)?;
            }
            UserForm::Id(v) => {
                limit_stream_runtime::ls_write_str(buf, "User")?;
                v.ser(buf)?;
            }
        }

        return Ok(());
    }
}

impl limit_stream_runtime::Deser for UserForm {
    type Res = Result<Self, ()>;
    fn deser(buf: &mut limit_stream_runtime::Bytes) -> Result<Self, ()> {
        if limit_stream_runtime::ls_read_array_len(buf)? != 2 {
            return Err(());
        }

        match limit_stream_runtime::ls_read_str(buf)?.as_str() {
            "User" => Ok(UserForm::User(limit_stream_runtime::ls_read_value(
                buf,
            )?)),
            "Id" => Ok(UserForm::Id(limit_stream_runtime::ls_read_value(
                buf,
            )?)),
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
    let mut channel = rmp::encode::ByteBuf::new();
    limit_stream_runtime::Ser::ser(&src, &mut channel).unwrap();
    let dst = <User as limit_stream_runtime::Deser>::deser(
        &mut limit_stream_runtime::Bytes::new(channel.as_slice()),
    )
    .unwrap();
    assert_eq!(dst, src);
}

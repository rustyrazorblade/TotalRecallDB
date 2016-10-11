extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};


#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    data: Vec<u8>
}

impl From<i64> for Value {
    fn from(val: i64) -> Value {
        let mut buffer = Vec::new();
        buffer.write_i64::<BigEndian>(val).unwrap();
        Value { data: buffer }
    }
}

impl From<String> for Value {
    fn from(val: String) -> Value {
        Value{data:val.into_bytes()}
    }
}

impl<'a> From<&'a str> for Value {
    fn from(val: &'a str) -> Value {
        let tmp = val.as_bytes();
        let mut v : Vec<u8> = Vec::new();
        v.extend_from_slice(&tmp);
        Value{data:v}
    }
}
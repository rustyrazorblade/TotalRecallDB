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

#[cfg(test)]
mod tests {
    use super::Value;
    #[test]
    fn ints_should_compare_equal() {
        let mut x = Value::from(1);
        let mut y = Value::from(1);
        assert_eq!(x, y);

        y = Value::from(2);
        assert!(x != y);
    }
}
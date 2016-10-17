extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};


#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    data: Vec<u8>
}

impl Value {
    fn to_int(&self) -> i64 {
        let mut cur = Cursor::new(self.data.clone());
        cur.read_i64::<LittleEndian>().unwrap()
    }
    fn to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Value {
        let mut buffer = Vec::new();
        buffer.write_i64::<LittleEndian>(val).unwrap();
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

    #[test]
    fn int_converts_back_ok() {
        let x = Value::from(1);
        let y = x.to_int();
        assert_eq!(1, y);
    }

    #[test]
    fn string_equality() {
        let x = Value::from("test");
        let y = Value::from("test");
        assert_eq!(x, y);
        let y = Value::from("test2");
        assert!(x != y);
    }

    #[test]
    fn string_conversions() {
        let tmp = Value::from("this is a test");
        let x = tmp.to_string();
        assert_eq!(x, "this is a test");
    }
}
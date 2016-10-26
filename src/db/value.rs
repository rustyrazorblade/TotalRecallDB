extern crate byteorder;

use std::cmp::{Ord, Ordering};

use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use super::schema::{Schema, Type};

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

struct ValueComparator {
    val: Value,
    dtype: Type,
}

impl ValueComparator {
    fn new(val: Value, dtype: Type) -> ValueComparator {
        ValueComparator{val:val, dtype:dtype}
    }
}

impl Ord for ValueComparator {
    fn cmp(&self, other: &ValueComparator) -> Ordering {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int().cmp(&other.val.to_int()),
            (&Type::String, &Type::String) => self.val.to_string().cmp(&other.val.to_string()),
            _ => Ordering::Less
        }
    }
}

impl PartialEq for ValueComparator {
    fn eq(&self, other: &ValueComparator) -> bool {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int() == other.val.to_int(),
            (&Type::String, &Type::String) => self.val.to_string() == other.val.to_string(),
            _ => false
        }
    }
}

impl Eq for ValueComparator {}

impl PartialOrd for ValueComparator {
    fn partial_cmp(&self, other: &ValueComparator) -> Option<Ordering> {
        Some(self.cmp(other))
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
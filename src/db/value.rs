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

impl<'a> From<&'a [u8]> for Value {
    fn from(bytes: &'a [u8]) -> Value {
        let mut v : Vec<u8> = Vec::new();
        v.extend_from_slice(bytes);
        Value{data:v}
    }
}

#[derive(Debug)]
pub struct ValueComparator<'a> {
    val: &'a Value,
    dtype: Type,
}

impl<'a> ValueComparator<'a> {
    pub fn new(val: &Value, dtype: Type) -> ValueComparator {
        ValueComparator{val:val, dtype:dtype}
    }
}

impl<'a> Ord for ValueComparator<'a> {
    fn cmp(&self, other: &ValueComparator) -> Ordering {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int().cmp(&other.val.to_int()),
            (&Type::String, &Type::String) => self.val.to_string().cmp(&other.val.to_string()),
            _ => Ordering::Less
        }
    }
}

impl<'a> PartialEq for ValueComparator<'a> {
    fn eq(&self, other: &ValueComparator) -> bool {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int() == other.val.to_int(),
            (&Type::String, &Type::String) => self.val.to_string() == other.val.to_string(),
            _ => false
        }
    }
}

impl<'a> Eq for ValueComparator<'a> {}

impl<'a> PartialOrd for ValueComparator<'a> {
    fn partial_cmp(&self, other: &ValueComparator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, ValueComparator};
    use db::schema::Type;

    #[test]
    fn ints_should_compare_equal() {
        let mut x = Value::from(1);
        let mut y = Value::from(1);
        assert_eq!(x, y);

        y = Value::from(2);
        assert!(x != y);
    }

    #[test]
    fn test_int_custom_comparator() {
        let mut x = Value::from(1);
        let mut y = Value::from(1);

        assert_eq!(ValueComparator::new(&x, Type::Int),
                   ValueComparator::new(&y, Type::Int));

        let mut x = Value::from(2);
        let mut y = Value::from(1);

        assert!(ValueComparator::new(&x, Type::Int) >
                ValueComparator::new(&y, Type::Int));

        let mut x = Value::from(1);
        let mut y = Value::from(2);

        assert!(ValueComparator::new(&y, Type::Int) >
                ValueComparator::new(&x, Type::Int));

        let mut x = Value::from(-1);
        let mut y = Value::from(2);

        assert!(ValueComparator::new(&y, Type::Int) >
                ValueComparator::new(&x, Type::Int));

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
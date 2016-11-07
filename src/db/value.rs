extern crate byteorder;

use std::cmp::{Ord, Ordering};
use std::ops::Deref;

use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use super::schema::{Schema, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub data: Vec<u8>
}

impl Value {
    pub fn to_int(&self) -> i64 {
        let mut cur = Cursor::new(self.data.clone());
        cur.read_i64::<BigEndian>().unwrap()
    }
    pub fn to_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
    pub fn to_bool(&self) -> bool {
        let mut cur = Cursor::new(self.data.clone());
        cur.read_u8().unwrap() == 1

    }
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }
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

impl From<bool> for Value {
    fn from(val: bool) -> Value {
        let x  = val as u8;
        let mut buffer = Vec::new();
        buffer.write_u8(x).unwrap();
        Value { data: buffer }
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
pub struct TypedValue {
    val: Value,
    dtype: Type,
}

impl TypedValue {
    pub fn new(val: Value, dtype: Type) -> TypedValue {
        TypedValue {val:val, dtype:dtype}
    }
}

impl Ord for TypedValue {
    fn cmp(&self, other: &TypedValue) -> Ordering {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int().cmp(&other.val.to_int()),
            (&Type::String, &Type::String) => self.val.to_string().cmp(&other.val.to_string()),
            _ => Ordering::Less
        }
    }
}

impl PartialEq for TypedValue {
    fn eq(&self, other: &TypedValue) -> bool {
        match (&self.dtype, &other.dtype) {
            (&Type::Int, &Type::Int) => self.val.to_int() == other.val.to_int(),
            (&Type::String, &Type::String) => self.val.to_string() == other.val.to_string(),
            _ => false
        }
    }
}

impl From<i64> for TypedValue {
    fn from(val: i64) -> TypedValue {
        let v = Value::from(val);
        TypedValue::new(v, Type::Int)
    }
}

impl From<bool> for TypedValue {
    fn from(val: bool) -> TypedValue {
        let v = Value::from(val);
        TypedValue::new(v, Type::Bool)
    }
}

impl From<String> for TypedValue {
    fn from(val: String) -> TypedValue {
        let v = Value::from(val);
        TypedValue::new(v, Type::String)
    }
}

impl<'a> From<&'a str> for TypedValue {
    fn from(val: &'a str) -> TypedValue {
        let v = Value::from(val);
        TypedValue::new(v, Type::String)
    }
}

impl Eq for TypedValue {}

impl PartialOrd for TypedValue {
    fn partial_cmp(&self, other: &TypedValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for TypedValue {
    type Target = Value;
    fn deref(&self) -> &Value {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use super::{Value, TypedValue};
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

        assert_eq!(TypedValue::new(x, Type::Int),
                   TypedValue::new(y, Type::Int));

        let mut x = Value::from(2);
        let mut y = Value::from(1);

        assert!(TypedValue::new(x, Type::Int) >
                TypedValue::new(y, Type::Int));

        let mut x = Value::from(1);
        let mut y = Value::from(2);

        assert!(TypedValue::new(y, Type::Int) >
                TypedValue::new(x, Type::Int));

        let mut x = Value::from(-1);
        let mut y = Value::from(2);

        assert!(TypedValue::new(y, Type::Int) >
                TypedValue::new(x, Type::Int));

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
        let _ = env_logger::init();
        let tmp = Value::from("this is a test");
        debug!("String data: {:?}", tmp.data);
        assert!(tmp.data.len() > 10);
        let x = tmp.to_string();
        assert_eq!(x, "this is a test");
    }

    #[test]
    fn test_bool_value() {
        let x = Value::from(true);
        let y = x.to_bool();
        assert!(y);

        let x = Value::from(false);
        let y = x.to_bool();
        assert!(!y);
    }
}
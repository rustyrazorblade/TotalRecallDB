use std::collections::HashMap;
use super::value::Value;

pub struct RowBuilder {
    pub data: HashMap<String, Value>,
}
/// RowBuilder is a nice wrapper around the Values
/// This is used in conjunction with Stream.insert
impl RowBuilder {
    pub fn new() -> RowBuilder {
        RowBuilder{data:HashMap::new()}
    }
    pub fn set_string(&mut self, key: &str, val: &str) -> &mut RowBuilder {
        self.data.insert(key.to_string(), Value::from(val));
        self
    }
    pub fn set_int(&mut self, key: &str, val: i64) -> &mut RowBuilder {
        self.data.insert(key.to_string(), Value::from(val));
        self
    }
    pub fn set(&mut self, key: &str, val: Value) -> &mut RowBuilder {
        self.data.insert(key.to_string(), val);
        self

    }
}

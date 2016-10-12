use std::collections::HashMap;
use super::value::Value;

pub struct RowBuilder {
    pub data: HashMap<String, Value>,
}

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
}

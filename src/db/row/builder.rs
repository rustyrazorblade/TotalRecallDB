use std::collections::HashMap;
use db::value::Value;
use super::row::{RowError, Row};
use db::schema::Schema;

#[derive(Debug, Clone)]
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
    pub fn to_row(mut self, schema: &Schema) -> Result<Row, RowError> {
        let mut row_map : HashMap<u16, Value> = HashMap::new();
        for (key, val) in self.data.drain() {
            // get the field from the schema
            // TypeDef
            let tmp = schema.get(&key)
                            .ok_or(RowError::FieldNotFound(key.to_string()))?;
            row_map.insert(tmp.id, val);

        }

        Row::new(row_map)
    }
}

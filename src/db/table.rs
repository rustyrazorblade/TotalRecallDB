
use std::collections::hash_map::HashMap;
use super::row::Row;

pub struct Table {
    num_rows: u64,
    rows: HashMap<u64, Row>
}


impl Table {
    pub fn new() -> Table {
        Table{
            num_rows: 0,
            rows:HashMap::new()
        }
    }
}
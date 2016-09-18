
use std::collections::hash_map::HashMap;
use super::row::Row;

pub struct Stream {
    num_rows: u64,
    rows: HashMap<u64, Row>
}


impl Stream {
    pub fn new() -> Stream {
        Stream {
            num_rows: 0,
            rows:HashMap::new()
        }
    }
}
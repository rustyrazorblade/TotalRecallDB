use std::collections::hash_map::HashMap;
use db::row::Row;

pub struct Table {
    num_rows: u64,
    rows: HashMap<u64, Row>
}
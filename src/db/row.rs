use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use super::field::Field;

pub struct Row {
    id: u64,
    fields: HashMap<u8, Field>,
}

impl Row {
    fn new(id: u64, fields: HashMap<u8, Field>) -> Row {
        Row{ id: id, fields: fields}
    }
}
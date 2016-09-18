use std::collections::HashMap;

use super::row::Row;
use super::schema::{Schema, Type};
use super::field::Field;

pub enum StreamError {
    ValidationError(String),
}

pub struct Stream {
    max_id: u64,
    rows: Vec<Row>,
    schema: Schema,
}

/**
This is a weird DB.  There's no primary key, since everything is based off append only streaming

*/
impl Stream {
    pub fn new() -> Stream {
        let mut stream = Stream {
            max_id: 0,
            rows: Vec::new(),
            schema: Schema::new(),
        };
        stream.schema.add_type("_id", Type::Int);
        stream.schema.add_type("_created", Type::Timestamp);
        stream
    }

    pub fn insert(data: HashMap<String, Field>) -> Result<Row, StreamError> {
        // validate the inserted data
        Err(StreamError::ValidationError("Could not insert".to_string()))
    }
}



#[cfg(tests)]
mod tests {

}
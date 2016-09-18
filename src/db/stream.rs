use std::collections::HashMap;
use std::error;

use super::row::{Row, RowError};
use super::schema::{Schema, Type};
use super::field::Field;

pub enum StreamError {
    ValidationError(String),
    FieldNotFound(String),
    MissingRowId
}

impl From<RowError> for StreamError {
    fn from(err: RowError) -> StreamError {
        StreamError::MissingRowId
    }

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

    /* we take a HashMap of String -> Field here
    * we're going to convert it to HashMap<u8, Field> for the Row struct
    */
    pub fn insert(&mut self, data: &mut HashMap<String, Field>) -> Result<Row, StreamError> {
        // validate the inserted data
        let mut row_map : HashMap<u8, Field> = HashMap::new();
        for (key, val) in data.drain() {
            // get the field from the schema
            // TypeDef
            let tmp = try!(self.schema.get(&key)
                                      .ok_or(StreamError::FieldNotFound(key.to_string())));
            row_map.insert(tmp.id, val);

        }
        let row = try!(Row::new(row_map));
        Ok(row)
    }
}



#[cfg(tests)]
mod tests {

}
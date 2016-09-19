use std::collections::HashMap;
use std::error;

use super::row::{Row, RowError};
use super::schema::{Schema, Type};
use super::field::Field;

#[derive(Debug)]
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

pub struct RowBuilder {
    data: HashMap<String, Field>,
}

impl RowBuilder {
    fn new() -> RowBuilder {
        RowBuilder{data:HashMap::new()}
    }
    fn set_string(&mut self, key: &str, val: &str) -> &mut RowBuilder {
        self.data.insert(key.to_string(), Field::String(val.to_string()));
        self
    }
    fn set_int(&mut self, key: &str, val: i64) -> &mut RowBuilder {
        self.data.insert(key.to_string(), Field::Int(val));
        self
    }
}

#[derive(Debug)]
pub struct Stream {
    max_id: u64,
    inserts: u64,
    ttl: Option<u64>,
    rows: Vec<Row>,
    schema: Schema,
}

/**
This is a weird DB.  There's no primary key, since everything is based off append only streaming

*/
impl Stream {
    pub fn new() -> Stream {
        let mut stream = Stream::new_empty();
        stream.schema.add_type("_id", Type::Int);
        stream.schema.add_type("_created", Type::Timestamp);
        stream
    }

    // used in temporary tables
    pub fn new_empty() -> Stream {
        let mut stream = Stream {
            max_id: 0,
            inserts: 0,
            rows: Vec::new(),
            schema: Schema::new(),
            ttl: None,
        };
        stream
    }

    /* we take a HashMap of String -> Field here
    * we're going to convert it to HashMap<u8, Field> for the Row struct
    */
    pub fn insert(&mut self, mut row_builder: RowBuilder) -> Result<Row, StreamError> {
        // validate the inserted data
        let mut row_map : HashMap<u8, Field> = HashMap::new();
        for (key, val) in row_builder.data.drain() {
            // get the field from the schema
            // TypeDef
            let tmp = try!(self.schema.get(&key)
                               .ok_or(StreamError::FieldNotFound(key.to_string())));
            row_map.insert(tmp.id, val);

        }
        let row = try!(Row::new(row_map));
        self.rows.push(row.clone());
        self.inserts += 1;
        Ok(row)
    }

    fn scan(&self) -> Result<Stream, StreamError> {
        Ok(Stream::new())
    }

}



#[cfg(test)]
mod tests {
    use super::{Stream, RowBuilder};
    use super::super::schema::{Schema, Type};
    use super::super::field::Field;
    use std::collections::HashMap;

    fn get_stream() -> Stream {
        let mut s = Stream::new();
        s.schema.add_type("name", Type::String);
        s.schema.add_type("age", Type::Int);
        s.schema.add_type("created", Type::Timestamp);
        s

    }

    #[test]
    fn test_insert_works_normal_case() {
        let mut s = Stream::new();
        s.schema.add_type("name", Type::String);

        let mut row = RowBuilder::new();
        row.set_string("name", "test");
        let result = s.insert(row).unwrap();
        assert_eq!(s.inserts, 1);
        // was the data inserted?

    }

    fn test_builder() {
        let mut s = get_stream();
        let mut row = RowBuilder::new();

        row.set_string("name", "value")
           .set_int("age", 10);
        s.insert(row);
    }
}
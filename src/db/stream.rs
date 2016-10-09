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
    inserts: u64, // total number of inserts this Stream has seen
    lowest_id: Option<u64>,  // lowest id we still have in the Stream
    ttl: Option<u64>,
    rows: HashMap<u64, Row>,
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
            inserts: 0,
            lowest_id: None,
            rows: HashMap::new(),
            schema: Schema::new(),
            ttl: None,
        };
        stream
    }

    /* we take a HashMap of String -> Field here
    * we're going to convert it to HashMap<u8, Field> for the Row struct
    */
    pub fn insert(&mut self, mut row_builder: RowBuilder) -> Result<u64, StreamError> {
        // validate the inserted data
        let mut row_map : HashMap<u8, Field> = HashMap::new();
        for (key, val) in row_builder.data.drain() {
            // get the field from the schema
            // TypeDef
            let tmp = try!(self.schema.get(&key)
                               .ok_or(StreamError::FieldNotFound(key.to_string())));
            row_map.insert(tmp.id, val);

        }
        let row_id = self.inserts;
        let row = try!(Row::new(row_map));
        self.rows.insert(self.inserts, row.clone());
        self.inserts += 1;
        Ok(row_id)
    }

    fn get(&self, position: u64) -> Option<&Row> {
        self.rows.get(&position)
    }


}

impl<'a> IntoIterator for &'a Stream {
    type Item = Row;
    type IntoIter = StreamIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // TODO fix the position
        StreamIterator{position:0,
                       stream: self}
    }
}

pub struct StreamIterator<'a> {
    position: u64,
    stream: &'a Stream,
}

impl<'a> StreamIterator<'a> {
    fn offset(&'a mut self, num: u64) -> &'a mut StreamIterator {
        self.position = num;
        self
    }
}

impl<'a> Iterator for StreamIterator<'a> {
    type Item = Row;
    fn next(&mut self) -> Option<Row> {
        None
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
        assert_eq!(result, 0);
        assert_eq!(s.inserts, 1);
        let r = s.get(result);

        // was the data inserted?

    }

    #[test]
    fn test_builder() {
        let mut s = get_stream();
        let mut row = RowBuilder::new();

        row.set_string("name", "value")
           .set_int("age", 10);
        s.insert(row);

        let mut c = 0;
        for r in s.into_iter() {
            c += 1;
        }
        assert_eq!(c, 1);
//        let r = s.next().unwrap();

    }
}
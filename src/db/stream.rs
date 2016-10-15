use std::collections::HashMap;
use std::error;

use super::row::{Row, RowError};
use super::schema::{Schema, Type};
use super::value::Value;
pub use super::row_builder::RowBuilder;
pub use super::row_reader::RowReader;

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



#[derive(Debug)]
pub struct Stream {
    inserts: u64, // total number of inserts this Stream has seen
    lowest_id: Option<u64>,  // lowest id we still have in the Stream
    ttl: Option<u64>,
    rows: HashMap<u64, Row>,
    pub schema: Schema,

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
        let mut row_map : HashMap<u8, Value> = HashMap::new();
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

    pub fn get(&self, position: u64) -> Option<RowReader> {
        if let Some(tmp) = self.rows.get(&position) {
            return Some(RowReader::new(&self.schema, &tmp));
        }
        None

    }


}

impl<'a> IntoIterator for &'a Stream {
    type Item = RowReader<'a>;
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
    type Item = RowReader<'a>;
    fn next(&mut self) -> Option<RowReader<'a>> {
        let tmp = self.stream.get(self.position);
        self.position = self.position + 1;
        tmp
    }
}

#[cfg(test)]
mod tests {
    #![feature(test)]
    extern crate test;
    use super::{Stream, RowBuilder};

    use db::schema::{Schema, Type};
    use db::value::Value;
    use std::collections::HashMap;
    use self::test::Bencher;

    fn get_stream() -> Stream {
        let mut s = Stream::new();
        s.schema.add_type("name", Type::String);
        s.schema.add_type("age", Type::Int);
        s.schema.add_type("created", Type::Timestamp);
        s
    }

    fn get_stream_with_data() -> Stream {
        let mut s = get_stream();

        let names = vec!["Jon", "Dani", "Pete", "Sammy", "Steph",
                         "Blake", "Mr. Robot", "Ted", "Rachel", "Gloria"];
        let ages = vec![35, 29, 45, 56, 100,
                        35, 78, 32, 32, 67];

        for x in 0..10 {
            let mut row = RowBuilder::new();
            row.set_string("name", names[x]);
            row.set_int("age", ages[x]);
            let result = s.insert(row).unwrap();
        }
        s
    }

    #[test]
    fn test_insert_works_normal_case() {
        let mut s = Stream::new();
        let name_id = s.schema.add_type("name", Type::String);

        let mut row = RowBuilder::new();
        row.set_string("name", "test");
        let result = s.insert(row).unwrap();
        assert_eq!(result, 0);
        assert_eq!(s.inserts, 1);
        let row2 = s.get(result).unwrap();
        let name = row2.get("name").unwrap();
        // was the data inserted?
        assert_eq!(*row2.get("name").unwrap(),
                   Value::from("test"));

    }

    #[test]
    fn test_equality() {
        assert_eq!(Value::from("jon"), Value::from("jon"));
    }

    #[test]
    fn test_builder_insertion() {
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
    }

    #[test]
    fn test_iterator_offset() {
        let mut s = get_stream_with_data();
        let mut i = 0;
        for row in s.into_iter().offset(9) {
            i = i + 1;
        }
        assert_eq!(i, 1);
    }

    #[test]
    fn test_chaining() {
        let mut s = get_stream_with_data();
        // SELECT * from X where age > 25
//        for row in s.into_iter().filter(|x| x["age"] > 25)  {
//
//        }

    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| 2 + 2);
    }

}

use super::row::Row;
use super::schema::{Schema, Type};


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
}



#[cfg(tests)]
mod tests {

}
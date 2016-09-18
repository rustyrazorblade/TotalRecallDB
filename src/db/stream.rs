
use std::collections::hash_map::HashMap;
use super::row::Row;

/*
DB types
*/
enum Type {
    Int,
    String
}

pub struct Stream {
    max_id: u64,
    rows: HashMap<u64, Row>
}

/**
This is a weird DB.  There's no primary key, since everything is based off append only streaming

*/
impl Stream {
    pub fn new() -> Stream {
        Stream {
            max_id: 0,
            rows:HashMap::new()
        }
    }
}



#[cfg(tests)]
mod tests {

}

use super::row::Row;
use super::schema::Schema;

/*
DB types
*/
enum Type {
    Int,
    String
}

pub struct Stream {
    max_id: u64,
    rows: Vec<Row>
}

/**
This is a weird DB.  There's no primary key, since everything is based off append only streaming

*/
impl Stream {
    pub fn new() -> Stream {
        Stream {
            max_id: 0,
            rows:Vec::new()
        }
    }
}



#[cfg(tests)]
mod tests {

}
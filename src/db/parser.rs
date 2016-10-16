peg_file! streamql("streamql.rustpeg");


#[derive(Debug)]
pub enum Statement {
    Insert(InsertStatement),
}


#[derive(Debug)]
pub struct InsertStatement {
    stream: String
}

impl InsertStatement {
    pub fn new(stream: String) -> InsertStatement {
        InsertStatement{stream: stream}
    }
}



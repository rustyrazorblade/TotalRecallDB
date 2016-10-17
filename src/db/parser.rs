peg_file! streamql("streamql.rustpeg");

use self::streamql::{statement, ParseError};

fn parse_statement(query: &str) -> Result<Statement, ParseError> {
    statement(query)
}

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

#[cfg(test)]
mod tests {
    use super::parse_statement;
    #[test]
    fn test_basic_inserts() {
        let result = parse_statement("INSERT INTO test set k=1");
        assert!(result.is_ok());
    }

}



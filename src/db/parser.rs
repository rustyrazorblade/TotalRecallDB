peg_file! streamql("streamql.rustpeg");

use self::streamql::{statement, ParseError};

fn parse_statement(query: &str) -> Result<Statement, ParseError> {
    statement(query)
}

#[derive(Debug)]
pub enum Statement {
    Insert(InsertStatement),
    CreateStream,
    DropStream,
    UseDatabase,
    Select,
    Subscribe,
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
    use super::Statement;
    use super::streamql::int_value;

    #[test]
    fn test_basic_insert() {
        let result = parse_statement("INSERT INTO test set k=1;").unwrap();

        if let Statement::Insert(x) = result {
            assert_eq!(x.stream, String::from("test"));
        } else {
            panic!("")
        }
        let x = "INSERT INTO test set k=1, v=2;";
        let result = parse_statement(x).unwrap();
    }

    #[test]


    #[test]
    fn test_int_parsing() {
        let tmp = int_value("42").unwrap();
        

    }
}




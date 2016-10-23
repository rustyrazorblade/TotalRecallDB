use self::streamql::statement;
pub use self::streamql::ParseError;
use super::row_builder::RowBuilder;

mod integration_tests;

use nom::{IResult,digit};
use nom::IResult::*;



peg_file! streamql("streamql.rustpeg");


pub fn parse_statement(query: &str) -> Result<Statement, ParseError> {
    statement(query)
}

#[derive(Debug)]
pub enum Statement {
    // Stream & Keys
    Insert(String, RowBuilder),
    // name
    DeclareStream(String),
    DropStream,
    UseDatabase,
    Select,
    Subscribe,
}






#[cfg(test)]
mod test {
    use super::parse_statement;
    use super::Statement;
    use super::streamql::*;
    use db::value::Value;

    #[test]
    fn test_basic_insert() {

        let result = parse_statement("INSERT INTO test set k=1;").unwrap();

        if let Statement::Insert(stream, builder) = result {
            assert_eq!(stream, String::from("test"));
        } else {
            panic!("")
        }
        let x = "INSERT INTO test set k=1, v=2;";
        let result = parse_statement(x).unwrap();

    }

    #[test]
    fn test_parsing_with_quoted_string() {
        let result = parse_statement("INSERT INTO test set k='test';").unwrap();
        let result = parse_statement("INSERT INTO test set k='test my ''friend''';").unwrap();

    }

    #[test]
    fn test_int_parsing() {
        let tmp = int_value("42").unwrap();
    }

    #[test]
    fn test_basic_string_parsing() {
        string("hello this is a test").unwrap();
    }

    #[test]
    fn test_escaped_string() {
        let x = string("hello this is a ''test''").unwrap();
        assert_eq!(x, "hello this is a 'test'");
    }

    #[test]
    fn test_quoted_string() {
        let x = "'it''s mine'";
        let p = quoted_string(x).unwrap();
        assert_eq!(p, "it's mine");
    }

    #[test]
    fn value_string_parsing() {
        let x = "'hello'";
        if Value::from(x) == value(x).unwrap() {
            assert_eq!(x, "hello");
        }

    }

    #[test]
    fn declare_stream_parsing() {
        let x = "declare stream readings ( ts int, val int, s text );";
        let p = parse_statement(x).unwrap();
    }

    #[test]
    fn test_basic_parse_column_specification() {
        let x = "ts int";
        let p = parse_statement(x).unwrap();

        let x = "ts text";
        let p = parse_statement(x).unwrap();

    }
}







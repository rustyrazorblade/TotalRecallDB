use self::streamql::statement;
pub use self::streamql::ParseError;
use super::row_builder::RowBuilder;
use super::schema::Type;

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
    DeclareStream(String, Vec<ColumnSpec>),
    DropStream,
    UseDatabase,
    // Stream
    Select(String),
    Subscribe,
}

#[derive(Debug)]
pub struct ColumnSpec {
    pub name: String,
    pub ftype: Type,
}

impl ColumnSpec {
    fn new(name: String, ftype: Type) -> ColumnSpec {
        ColumnSpec{name: name,
                   ftype: ftype}
    }
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
        let p = column_spec(x).unwrap();

        let x = "ts text";
        let p = column_spec(x).unwrap();
    }

    #[test]
    fn test_column_spec_list() {
        let x = "ts int, val int, s text ";
        let p = column_spec_list(x).unwrap();

        assert_eq!(p.len(), 3);

        assert_eq!(p[0].name, "ts");
        assert_eq!(p[1].name, "val");
        assert_eq!(p[2].name, "s");
    }

    #[test]
    fn test_select_queries() {
        let queries = ["select * from test",
                        "select name, age from users",
                        "select name from cities where state = 'CA'"];
        for q in queries.into_iter() {
            parse_statement(&q).expect(q);
        }

    }

    #[test]
    fn test_field_list() {
        let queries = ["name, email",
            "*", "stuff"];
        for q in queries.into_iter() {
            field_list(q).expect(q);
        }

    }

    #[test]
    fn test_where_clause() {
        let queries = ["where name = 'jon'",
        "where id = 5",
        "where name = 'bill' and age > 35"];
        for q in queries.into_iter() {
            field_list(q).expect(q);
        }
    }
}







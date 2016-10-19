extern crate db;

use db::parse_statement;
use db::parser::Statement;
use db::parser::streamql::*;

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
fn test_int_parsing() {
    let tmp = int_value("42").unwrap();
}

#[test]
fn test_basic_string_parsing() {
    string("hello this is a test").unwrap();
    string(r#"hello this is a \"test\""#).unwrap();
}

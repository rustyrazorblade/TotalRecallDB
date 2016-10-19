peg_file! streamql("streamql.rustpeg");

use self::streamql::{statement, ParseError};
use super::row_builder::RowBuilder;

fn parse_statement(query: &str) -> Result<Statement, ParseError> {
    statement(query)
}

#[derive(Debug)]
pub enum Statement {
    // Stream & Keys
    Insert(String, RowBuilder),
    CreateStream,
    DropStream,
    UseDatabase,
    Select,
    Subscribe,
}






use std::collections::HashMap;
use std::path::PathBuf;
use std::env;

use tempdir::TempDir;

use super::stream::{Stream, StreamError};
use super::parser::{parse_statement, Statement, ParseError, ColumnSpec};
use super::row::builder::RowBuilder;
use super::parser::Expression;

use super::storage::disk::Disk;
use super::storage::memory::Memory;

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    TableExists,
    QueryParseError,
    UnknownError,
    StreamNotFound,
    FieldNotFound(String),
}

type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug)]
pub enum QueryResult {
    ResultSet(ResultSet),
    Insert(u64),
    StreamCreated,
}

impl From<ParseError> for DatabaseError {
    fn from(err: ParseError) -> DatabaseError {
        DatabaseError::QueryParseError
    }
}
impl From<StreamError> for DatabaseError {
    fn from(err: StreamError) -> DatabaseError {
        let tmp = match err {
            StreamError::FieldNotFound(x) =>
                DatabaseError::FieldNotFound(x),
            _ =>
                DatabaseError::UnknownError

        };
        tmp
    }
}

#[derive(Debug)]
struct ResultSet {
    statement: Statement,
    num_results: u64,
}
impl ResultSet {
    fn new(statement: Statement) -> ResultSet {
        ResultSet{num_results:0, statement: statement}
    }
}

pub struct Database {
    path: PathBuf,
    tables: HashMap<String, Stream>
}

impl Database {
    pub fn new(path: PathBuf) -> Database {
        // create the directory if it doesn't exist?
        Database{
            tables: HashMap::new(),
            path: path,
        }
    }
    pub fn new_temp() -> Database {
        let tmpdir = TempDir::new("totalrecalldb").expect("Creating temp dir failed");
        info!("Created temporary DB at {:?}", tmpdir.path());
        Database::new(tmpdir.into_path())
    }

    pub fn create_stream(&mut self, name: &str) -> DatabaseResult<&mut Stream> {
        if self.tables.contains_key(name) {
            return Err(DatabaseError::TableExists);
        }

        let mut p = self.path.clone();
        p.push(name);

        let storage = Disk::new(50, p).expect("Could not create disk storage");
        let tmp = Stream::new(storage);

        self.tables.insert(name.to_string(), tmp);
        let stream = self.tables.get_mut(name).unwrap();
        Ok(stream)
    }
    pub fn create_temp_stream(&mut self) -> DatabaseResult<Stream> {
        let storage = Memory::new().expect("Memory is failing uggghhh");
        let mut stream = Stream::new(storage);
        Ok(stream)
    }

    fn get_stream(&self, name: &str) -> Option<&Stream> {
        self.tables.get(name)
    }

    fn get_stream_mut(&mut self, name: &str) -> Option<&mut Stream> {
        self.tables.get_mut(name)
    }


    pub fn execute(&mut self, query: &str) -> Result<QueryResult, DatabaseError> {
        let parsed = parse_statement(query)?;
        let p2 = parsed.clone();
        let result = match parsed {
            Statement::Insert(stream, row_builder) =>
                self.insert(&stream, row_builder),
            Statement::DeclareStream(stream, fields) =>
                self.declare_stream(&stream, fields),
            Statement::Select(table, predicates) => {
                // going to return the resultset now
                // the expectation is that all the validation be done up front
                let tmp = ResultSet::new(p2);
                Ok(QueryResult::ResultSet(tmp))
            }
            _ => Err(DatabaseError::UnknownError)
        };
        result
    }

    pub fn select(&self, stream: &str, predicates: Option<Box<Expression>>) ->
                    Result<QueryResult, DatabaseError> {
        let result = self.get_stream(stream)
                         .ok_or(DatabaseError::StreamNotFound)?
                         .select(predicates)?;

        Err(DatabaseError::UnknownError)
    }

    pub fn insert(&mut self, stream: &str, row_builder: RowBuilder) -> DatabaseResult<QueryResult> {
        let stream = self.get_stream_mut(stream).ok_or(DatabaseError::StreamNotFound)?;
        let id = stream.insert(row_builder)?;
        Ok(QueryResult::Insert(id))
    }

    pub fn declare_stream(&mut self,
                          stream: &str,
                          fields: Vec<ColumnSpec>) -> DatabaseResult<QueryResult> {
        let stream = self.create_stream(stream)?;
        for col_spec in fields {
            stream.schema.add_type(&col_spec.name, col_spec.ftype);
        }
        Ok(QueryResult::StreamCreated)
    }

}

#[cfg(test)]
mod tests {
    use super::Database;
    use super::DatabaseError;

    // returns a valid DB for use with testing with valid simple schema
    fn get_db_with_stream() -> Database {
        let mut db = Database::new_temp();
        db.create_stream("Jon");

        db
    }

    #[test]
    fn create_table() {
        let mut db = Database::new_temp();
        db.create_stream("Jon");
    }

    #[test]
    fn create_table_fails_when_table_exists() {
        let mut db = get_db_with_stream();
        if let Err(result) = db.create_stream("Jon") {
            assert_eq!(result, DatabaseError::TableExists);
        } else {
            panic!("Was expecting DatabaseError::TableExists, got an OK");
        }

    }
}

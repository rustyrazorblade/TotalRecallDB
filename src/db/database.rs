use std::collections::HashMap;
use super::stream::Stream;

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    TableExists,
}



pub struct Database {
    tables: HashMap<String, Stream>
}

impl Database {
    pub fn new() -> Database {
        Database{
            tables: HashMap::new()
        }
    }

    pub fn create_stream(&mut self, name: &str) -> Result<&mut Stream, DatabaseError> {
        let tmp = Stream::new();
        if self.tables.contains_key(name) {
            return Err(DatabaseError::TableExists);
        }
        self.tables.insert(name.to_string(), tmp);
        let stream = self.tables.get_mut(name).unwrap();
        Ok(stream)
    }

    fn get_stream_mut(&mut self, name: &str) -> Option<&mut Stream> {
        self.tables.get_mut(name)
    }

    pub fn execute(&mut self, query: &str) {

    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use super::DatabaseError;

    // returns a valid DB for use with testing with valid simple schema
    fn get_db_with_stream() -> Database {
        let mut db = Database::new();
        db.create_stream("Jon");

        db
    }

    #[test]
    fn create_table() {
        let mut db = Database::new();
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

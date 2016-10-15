use std::collections::HashMap;
use super::stream::Stream;


pub struct Database {
    tables: HashMap<String, Stream>
}

impl Database {
    pub fn new() -> Database {
        Database{
            tables: HashMap::new()
        }
    }

    fn create_stream(&mut self, name: &str)  {
        let tmp = Stream::new();
        self.tables.insert(name.to_string(), tmp);
    }

    fn get_table_mut(&mut self, name: &str) -> Option<&mut Stream> {
        self.tables.get_mut(name)
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    // returns a valid DB for use with testing with valid simple schema
    fn get_db_with_stream() {
        let mut db = Database::new();
        db.create_stream("Jon");


    }

    #[test]
    fn create_table() {
        let mut db = Database::new();
        db.create_stream("Jon");
    }



}

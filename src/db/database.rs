use std::collections::HashMap;
use super::table::Table;


pub struct Database {
    tables: HashMap<String, Table>
}

impl Database {
    pub fn new() -> Database {
        Database{
            tables: HashMap::new()
        }
    }
    fn create_table(&mut self, name: String)  {
        let tmp = Table::new();
        self.tables.insert(name, tmp);
    }
}
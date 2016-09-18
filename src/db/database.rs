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

    fn create_table(&mut self, name: &str)  {
        let tmp = Table::new();
        self.tables.insert(name.to_string(), tmp);
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    #[test]
    fn create_table() {
        let mut db = Database::new();
        db.create_table("Jon");
    }
}

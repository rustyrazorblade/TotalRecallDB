use std::collections::HashMap;
use db::table::Table;


struct Database {
    tables: HashMap<String, Table>
}
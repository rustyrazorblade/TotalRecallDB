use std::collections::HashMap;
use super::table::Table;


pub struct Database {
    tables: HashMap<String, Table>
}
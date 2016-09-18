use std::collections::HashMap;
use super::database::Database;

pub struct Server {
    databases: HashMap<String, Database>

}

impl Server {
    pub fn new() -> Server {
        Server{
            databases: HashMap::new()
        }
    }
}
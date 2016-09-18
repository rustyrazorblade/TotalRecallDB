use std::collections::HashMap;

/*
DB types
*/
#[derive(Debug)]
pub enum Type {
    Int,
    String
}


#[derive(Debug)]
pub struct TypeDef {
    id: u8,
    dbtype: Type,
}
// owned by a Stream
#[derive(Debug)]
pub struct Schema {
    num_fields: u8,
    // maps a name to an id
    fields: HashMap<String, u8>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema{num_fields: 0, fields: HashMap::new()}
    }

    pub fn add_type(&mut self, name: &str, dbtype: Type)  {

    }

}
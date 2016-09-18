use std::collections::HashMap;

/*
DB types
*/
#[derive(Debug)]
pub enum Type {
    Int,
    String,
    Timestamp,
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
    fields: HashMap<String, TypeDef>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema{num_fields: 0, fields: HashMap::new()}
    }

    pub fn add_type(&mut self, name: &str, dbtype: Type) {
        let next_id = self.num_fields + 1;
        let typedef = TypeDef{ id: next_id, dbtype: dbtype};
        self.fields.insert(name.to_string(), typedef);
        self.num_fields += 1;

    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_type_gets_new_id() {

    }
}
use std::collections::HashMap;

/*
DB types
*/
#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Timestamp,
    Bool,
}


#[derive(Debug)]
pub struct TypeDef {
    // id to be used in the on disk serialization
    pub id: usize,
    pub dbtype: Type,
}
// owned by a Stream
#[derive(Debug)]
pub struct Schema {
    num_fields: usize,
    // maps a name to an id
    fields: HashMap<String, TypeDef>,
}


impl Schema {
    pub fn new() -> Schema {
        Schema{num_fields: 0, fields: HashMap::new()}
    }

    pub fn add_type(&mut self, name: &str, dbtype: Type) -> usize {
        let typedef = TypeDef{ id: self.num_fields, dbtype: dbtype};
        self.fields.insert(name.to_string(), typedef);
        self.num_fields += 1;
        self.num_fields - 1

    }
    pub fn get(&self, name: &str) -> Option<&TypeDef> {
        self.fields.get(&name.to_string())
    }



}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_type_gets_new_id() {

    }
}
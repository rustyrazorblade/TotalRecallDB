use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::fmt;
use super::value::Value;

pub enum RowError {
    MissingId
}

struct Header {
    offsets: Vec<(u8, u16)>
}


impl Header {
    fn new() -> Header {
        Header{offsets:Vec::new()}
    }
    // returns an offset in number bytes for the byte data in a Row
    fn get_offset(field: u8) -> u16 {
        0
    }

}

impl<'a> From<&'a [u8]> for Header {
    fn from(bytes: &'a [u8]) -> Header {
        Header::new()
    }
}

// it's the table's job to validate the data going into the row
#[derive(Clone)]
pub struct Row {
    fields: HashMap<u8, Value>,
}


impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tmp = f.debug_struct("Row");
        for (key, field) in self.fields.iter() {
            tmp.field(&String::from("something"), field);
        }
        tmp.finish()

    }

}


impl Row {
    pub fn new(fields: HashMap<u8, Value>) -> Result<Row, RowError>  {
        Ok(Row{fields: fields})
    }

    pub fn get(&self, field: u8) -> Option<&Value> {
        self.fields.get(&field)
    }

//    fn get_string(&self, field: u8) ->
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::value::Value;
    use super::*;

    #[test]
    fn test_indexing() {
        let mut fields = HashMap::new();
        fields.insert(0, Value::from("test"));
        let r = Row::new(fields);
    }
}

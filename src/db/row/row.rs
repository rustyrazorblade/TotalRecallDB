
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::fmt;
use db::value::Value;
use super::Header;

// big endian for life
use byteorder::{BigEndian, WriteBytesExt};



pub enum RowError {
    MissingId
}



// it's the table's job to validate the data going into the row
#[derive(Clone)]
pub struct Row {
//    header: Header,
    fields: HashMap<u16, Value>,
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

impl<'a> From<&'a [u8]> for Row {
    fn from(bytes: &'a [u8]) -> Row {
        Row::empty()
    }
}

impl Row {
    pub fn new(fields: HashMap<u16, Value>) -> Result<Row, RowError>  {
        Ok(Row{fields: fields})
    }

    pub fn get(&self, field: u16) -> Option<&Value> {
        self.fields.get(&field)
    }
    pub fn empty() -> Row {
        Row{fields: HashMap::new()}
    }

    /*
    encoding stuff
    */
    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer = vec![];

        // header is just the number of fields
        let fields = self.fields.len() as u16;

        buffer.write_u16::<BigEndian>(fields);

        for (k, v) in self.fields.iter()  {
            // write field (u16), size (u64), data
            buffer.write_u16::<BigEndian>(*k);
            buffer.write_u64::<BigEndian>(v.len());
            buffer.extend_from_slice(v.as_slice());
        }
        buffer
    }

//    fn get_string(&self, field: u8) ->
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use db::value::Value;
    use super::*;

    #[test]
    fn test_indexing() {
        let mut fields = HashMap::new();
        fields.insert(0, Value::from("test"));
        let r = Row::new(fields);
    }
}

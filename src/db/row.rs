use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use super::field::Field;
use std::fmt;

pub enum RowError {
    MissingId
}

// it's the table's job to validate the data going into the row
#[derive(Clone)]
pub struct Row {
    fields: HashMap<u8, Field>,
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
    pub fn new(fields: HashMap<u8, Field>) -> Result<Row, RowError>  {
        Ok(Row{fields: fields})
    }

    fn get(&self, field: u8) -> Option<Field> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::field::Field;
    use super::*;
}

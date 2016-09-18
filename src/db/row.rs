use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use super::field::Field;

pub enum RowError {
    MissingId
}

// it's the table's job to validate the data going into the row
pub struct Row {
    fields: HashMap<u8, Field>,
}

impl Row {
    fn new(fields: HashMap<u8, Field>) -> Result<Row, RowError>  {
        Ok(Row{fields: fields})
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::field::Field;
    use super::*;


}

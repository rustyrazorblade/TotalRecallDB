use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use super::field::Field;

pub enum RowError {
    MissingId
}

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

    #[test]
    fn test_missing_id_fails() {
        let mut tmp = HashMap::new();
        tmp.insert(1, Field::String("test".to_string()));
        match Row::new(tmp) {
            Err(RowError::MissingId) => {},
            _ => {
                panic!("Was expecting a RowError");
            }
        }

    }
}

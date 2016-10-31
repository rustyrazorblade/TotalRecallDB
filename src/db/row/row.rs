use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::fmt;
use db::value::Value;
use super::Header;
use std::io::Cursor;
use std::io::Read;
// big endian for life
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[derive(Debug, PartialEq)]
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
        f.write_str("Row: ");
        let mut tmp = f.debug_map();
        tmp.entries( self.fields.iter().map(|(k,v)| (k,v) )).finish()
    }

}

impl<'a> From<&'a [u8]> for Row {
    fn from(bytes: &'a [u8]) -> Row {
        let mut cur = Cursor::new(bytes);
        let fields = cur.read_u16::<BigEndian>()
                        .expect("no fields length");

        debug!("Reading row, total fields expected: {}", fields);
        let mut hm = HashMap::new();
        for x in 0..fields {
            debug!("Reading field {}", x);
            let field = cur.read_u16::<BigEndian>()
                                .expect("Expecting Field id");
            debug!("Field: {}", field);
            let size = cur.read_u64::<BigEndian>()
                          .expect("Expecting Field length");
            debug!("Field size: {}", size);
            debug!("Cursor position before reading value: {}", cur.position());
            let mut data = Vec::with_capacity(size as usize);
            {
                cur.by_ref().take(size).read_to_end(&mut data);
                debug!("Raw Data: {:?}", data);
            }
            let pos = cur.position();
            cur.set_position(pos + size);
            debug!("Cursor position after reading value: {}", cur.position());
            //            hm.set(field, )
            let val = Value::from(&*data);
            debug!("Data: {:?}", val);
            hm.insert(field, val);
        }

        Row::new(hm).unwrap()
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
        debug!("Writing row to bytes");
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

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use db::value::Value;
    use super::*;

    extern crate env_logger;


    #[test]
    fn test_indexing() {
        let mut fields = HashMap::new();
        fields.insert(0, Value::from("test"));
        let r = Row::new(fields);
    }

    #[test]
    fn test_reading_and_writing_row() {
        let _ = env_logger::init();

        let mut fields = HashMap::new();
        fields.insert(0, Value::from("this is a test"));
//        fields.insert(1, Value::from(50));
        let r = Row::new(fields).expect("Valid row");

        let encoded = r.to_vec();
        let decoded = Row::from(encoded.as_slice());

        info!("encoded: {:?}", encoded);
        debug!("decoded: {:?}", decoded);
        assert_eq!(*decoded.get(0).unwrap(), Value::from("this is a test"));
    }
}

/*
Header for a row

Stores a Vector of field & offset pairs
Used with a Row to get correct slices of data to create Values
*/

#[derive(Clone, Debug)]
pub struct Header {
    offsets: Vec<(u8, u16)>
}

impl Header {
    pub fn new() -> Header {
        Header{offsets:Vec::new()}
    }
    // returns an offset in number bytes for the byte data in a Row
    pub fn get_offset(field: u8) -> u16 {
        0
    }

    // should this consume self? do I need a header after it's written out?
    // how will that work with a Row?
    // I don't think it can..  making it take &self for now and return new bytes
    pub fn into_vec(&self) -> Vec<u8> {
        Vec::new()
    }


}

impl<'a> From<&'a [u8]> for Header {
    fn from(bytes: &'a [u8]) -> Header {
        Header::new()
    }
}

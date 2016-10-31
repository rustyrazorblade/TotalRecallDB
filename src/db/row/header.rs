/*
Header for a row

Stores a Vector of field & offset pairs
Used with a Row to get correct slices of data to create Values
*/
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

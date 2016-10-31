/*
Header for a row

Stores a Vector of field & offset pairs
Used with a Row to get correct slices of data to create Values

Headers are 4 byte pairs - 2 for field id and 2 for offset
*/

enum HeaderError {
    FieldExists
}

#[derive(Clone, Debug)]
pub struct Header;

impl Header {
    pub fn new() -> Header {
        Header{}
    }

}

impl<'a> From<&'a [u8]> for Header {
    fn from(bytes: &'a [u8]) -> Header {
        Header::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_header() {
        let h = Header::new();
    }
}

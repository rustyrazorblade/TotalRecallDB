/*
Header for a row
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
    pub fn to_vec(&self) -> Vec<u8> {
        Vec::new()
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

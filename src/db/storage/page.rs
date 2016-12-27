use std::io::Cursor;
use std::io::{Read, Write};

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use db::row::Row;

// mis info about what's on each page.
const HEADER_SIZE_IN_BYTES : usize = 64;
pub const PAGE_SIZE : usize = 4096;

pub struct Page {
    header: Header,
//    data: [u8; PAGE_SIZE - HEADER_SIZE_IN_BYTES],
    data: Vec<u8>,
    bytes_used: usize, // tracking where we are in the current page

                                      // if we go past the page boundary on a write we fault
}

#[derive(Debug)]
pub enum PageError {
    Full
}

type PageResult<T> = Result<T, PageError>;

struct Header;

impl Header {
    fn as_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(HEADER_SIZE_IN_BYTES);
        result.resize(HEADER_SIZE_IN_BYTES, 0);
        result
    }
}

// page deals with bytes, and has zero knowledge of a Row
// storage engine needs to serialize
impl Page {
    pub fn new() -> Page {
        let tmp: [u8; PAGE_SIZE - HEADER_SIZE_IN_BYTES];

        Page{header: Header::new(),
             data: Vec::with_capacity(PAGE_SIZE),
             bytes_used: 0,
             }
    }

    // attempt to write to a page
    // can fail if the page is already full
    // if it fails, the storage engine will have to flush this page
    // then allocate a new page
    pub fn write(&mut self, bytes: &[u8]) -> PageResult<()> {
        if bytes.len() > self.space_available() {
            return Err(PageError::Full)
        }
        // we need the length of the row
        let len = bytes.len() as u16;
        self.data.write_u16::<BigEndian>(len);
        self.data.write(bytes);

        self.bytes_used += 2 + len as usize;

        // TODO: update indexes
        Ok(())
    }

    // internal call, try to write instead since it holds a mutable ref
    fn space_available(&self) -> usize {
        PAGE_SIZE - HEADER_SIZE_IN_BYTES - self.bytes_used
    }

    // returns PAGE_SIZE bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(PAGE_SIZE);
        // get the header
        let header = self.header.as_vec();
        result.extend(header);
        result.extend(&self.data);
        result.resize(PAGE_SIZE, 0);
        result
    }

}

impl Header {
    fn new() -> Header {
        Header{}
    }
}

#[cfg(test)]
mod tests {
    use super::{Page, HEADER_SIZE_IN_BYTES, PAGE_SIZE};
    #[test]
    fn test_page_insert_ok() {
        let mut p = Page::new();

        assert_eq!(p.space_available(), PAGE_SIZE - HEADER_SIZE_IN_BYTES);
        assert_eq!(p.bytes_used, 0);

        let data: [u8; 16] = [0; 16];
        p.write(&data).expect("Data written");
        // 2 extra bytes from the size
        assert_eq!(p.bytes_used, 18);

        let result = p.to_bytes();
        assert_eq!(result.len(), PAGE_SIZE);
    }

    #[test]
    #[should_panic]
    fn test_page_fault() {
        let mut p = Page::new();
        let data: [u8; 5000] = [0; 5000];
        p.write(&data).expect("Failure is expected");
    }
}


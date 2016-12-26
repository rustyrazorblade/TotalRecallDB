use db::row::Row;
// mis info about what's on each page.
const HEADER_SIZE_IN_BYTES : usize = 64;
pub const PAGE_SIZE : usize = 4096;

pub struct Page {
    header: Header,
    data: [u8; PAGE_SIZE - HEADER_SIZE_IN_BYTES],
    bytes_used: usize, // tracking where we are in the current page

                                      // if we go past the page boundary on a write we fault
}

#[derive(Debug)]
pub enum PageError {
    Full
}

type PageResult<T> = Result<T, PageError>;

struct Header;

// page deals with bytes, and has zero knowledge of a Row
// storage engine needs to serialize
impl Page {
    pub fn new() -> Page {
        Page{header: Header::new(),
             data: [0; 4096 - HEADER_SIZE_IN_BYTES],
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
        self.data[self.bytes_used .. bytes.len() + self.bytes_used].copy_from_slice(bytes);
        self.bytes_used += bytes.len();
        Ok(())
    }

    // internal call, try to write instead since it holds a mutable ref
    fn space_available(&self) -> usize {
        PAGE_SIZE - HEADER_SIZE_IN_BYTES - self.bytes_used
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
        assert_eq!(p.bytes_used, 16);
    }

    #[test]
    #[should_panic]
    fn test_page_fault() {
        let mut p = Page::new();
        let data: [u8; 5000] = [0; 5000];
        p.write(&data).expect("Failure is expected");
    }
}


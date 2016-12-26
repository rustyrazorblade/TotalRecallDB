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
enum PageError {
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
             bytes_used: HEADER_SIZE_IN_BYTES,
             }
    }

    // attempt to write to a page
    // can fail if the page is already full
    // if it fails, the storage engine will have to flush this page
    // then allocate a new page
    fn write(&mut self, bytes: &[u8]) -> PageResult<()> {
        Err(PageError::Full)
    }

    fn space_available(self) -> usize {
        PAGE_SIZE - self.bytes_used
    }

}

impl Header {
    fn new() -> Header {
        Header{}
    }
}

#[cfg(test)]
mod tests {
    use super::{Page, HEADER_SIZE_IN_BYTES};
    #[test]
    fn test_page_insert_ok() {
        let mut p = Page::new();
        let data: [u8; 16] = [0; 16];
        p.write(&data).expect("Data written");
        assert_eq!(p.bytes_used, HEADER_SIZE_IN_BYTES + 16);
    }

    #[test]
    fn test_page_fault() {

    }
}


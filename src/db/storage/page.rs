use db::row::Row;
// mis info about what's on each page.
const HEADER_SIZE_IN_BYTES : usize = 128;

pub struct Page {
    header: Header,
    data: [u8; 4096 - HEADER_SIZE_IN_BYTES],
    pos: u16,
}

enum PageError {
    Full
}

struct Header;

impl Page {
    fn new() -> Page {
        Page{header: Header::new(),
             data: [0; 4096 - HEADER_SIZE_IN_BYTES],
             pos: 0}
    }
    fn write(&mut self, row: &Row) -> Result<(), PageError> {
        Err(PageError::Full)
    }
}

impl Header {
    fn new() -> Header {
        Header{}
    }
}
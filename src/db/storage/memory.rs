use super::{Storage, StorageResult, StorageError};
use db::storage::Page;
use super::{Row, RowBuilder};

// in memory storage.  has a bunch of pages in a vector
pub struct Memory {
    pages: Vec<Page>
}
impl Memory {
    pub fn new() -> StorageResult<Memory> {
        Ok(Memory{pages:Vec::new()})
    }
}
impl Storage for Memory {
    fn get_page(&self, num: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
    fn write_page(&mut self, page: &Page) -> StorageResult<()> {
        unimplemented!()
    }
}
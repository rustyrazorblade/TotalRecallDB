use super::{Storage, StorageResult, StorageError};
use db::storage::Page;

// in memory storage.  has a bunch of pages in a vector
struct Memory {
    pages: Vec<Page>
}

impl Storage for Memory {
    fn get_page(num: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
}
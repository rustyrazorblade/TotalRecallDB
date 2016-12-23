
use super::{Storage, StorageError, StorageResult};
use super::Page;

struct Disk {

}

impl Storage for Disk {
    fn get_page(_: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
}

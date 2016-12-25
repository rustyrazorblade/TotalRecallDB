pub mod page;
pub mod memory;
pub mod disk;

use super::storage::page::Page;
pub use self::memory::Memory;
pub use self::disk::Disk;


#[derive(Debug)]
pub enum StorageError {
    PageNotFound,
    StreamAllocationError, // could not create a stream
}

type StorageResult<T> = Result<T, StorageError>;

// storage for a single stream
pub trait Storage {
    fn get_page(u64) -> StorageResult<Page>;
//    fn append(Page);
}
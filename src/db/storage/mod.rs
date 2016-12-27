pub mod page;
pub mod memory;
pub mod disk;

use super::storage::page::{Page, PAGE_SIZE, PageError};
pub use self::memory::Memory;
pub use self::disk::Disk;

pub use db::row::{RowBuilder, Row};


#[derive(Debug)]
pub enum StorageError {
    PageNotFound,
    StreamAllocationError, // could not create a stream
}

type StorageResult<T> = Result<T, StorageError>;

// storage for a single stream
pub trait Storage {
    fn get_page(&self, u64) -> StorageResult<Page>;
    // storage insert doesn't know about Rows
    // the stream will serialize the data before coming here
    fn insert(&mut self, row: &RowBuilder) -> StorageResult<()>;

}
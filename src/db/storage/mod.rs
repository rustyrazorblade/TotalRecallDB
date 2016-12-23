pub mod page;
pub mod memory;
pub mod disk;

use super::storage::page::Page;

pub enum StorageError {
    PageNotFound
}

type StorageResult<T> = Result<T, StorageError>;

pub trait Storage {
    fn get_page(u64) -> StorageResult<Page>;
//    fn append(Page);
}
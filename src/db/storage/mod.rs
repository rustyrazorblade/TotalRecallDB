pub mod page;
pub mod memory;
pub mod disk;

use super::storage::page::Page;

enum StorageError

pub trait Storage {
    fn get_page(u64) -> Page;
    fn append(Page);
}
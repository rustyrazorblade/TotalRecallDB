pub mod disk;
pub mod memory;

use db::storage::{Memory, Disk, StorageEngine};
use tempdir::TempDir;
use db::storage::Storage;

#[macro_export]
macro_rules! test_storage {
    ( $f:ident, $x:ident, $y:ident ) => {
        $f(&$x);
        $f(&$y);
    };
}

#[macro_export]
macro_rules! storage {
    ($f:ident) => {
        let storage = get_memory_storage();
        let storage2 = get_disk_storage();

        $f(&storage);
        $f(&storage2);
    };
}

fn get_memory_storage<'a>() -> StorageEngine<'a> {
    let mem = Memory::new().expect("Memory storage");
    StorageEngine::new(mem)
}

fn get_disk_storage<'a>() -> StorageEngine<'a> {
    // disk storage
    let tmp = TempDir::new("totalrecallwhatever").expect("Expected dir");
    let mut tmp2 = tmp.into_path();
    tmp2.push("blah");

    let disk = Disk::new(10, tmp2).expect("Disk storage");
    StorageEngine::new(disk)
}



#[test]
fn test_storage_engine_creation() {
    let storage = get_memory_storage();
    let storage2 = get_disk_storage();

    test_storage!(test_page_write_and_get, storage, storage2);
}

fn test_page_write_and_get(s: &StorageEngine) {

}

#[test]
fn test_blah() {
    fn something(s: &StorageEngine) {

    }
    storage!(something);
}


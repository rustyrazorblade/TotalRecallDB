pub mod disk;
pub mod memory;

use db::storage::{Memory, Disk, StorageEngine};
use tempdir::TempDir;
use db::storage::Storage;

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
    fn something(s: &StorageEngine) {
    }
    storage!(something);
}

#[test]
fn test_insert_and_get() {
    fn body(s: &StorageEngine) {

    }
    storage!(body);
}

pub mod disk;
pub mod memory;

use db::storage::{Memory, Disk, StorageEngine};
use tempdir::TempDir;
use db::storage::Storage;

macro_rules! test_storage {
    ( $f:ident, $x:ident, $y:ident ) => {
        $f(&$x);
        $f(&$y);
    };
}

#[test]
fn test_storage_engine_creation() {
    let mem = Memory::new().expect("Memory storage");
    let storage = StorageEngine::new(mem);

    // disk storage
    let tmp = TempDir::new("totalrecallwhatever").expect("Expected dir");
    let mut tmp2 = tmp.into_path();
    tmp2.push("blah");

    let disk = Disk::new(10, tmp2).expect("Disk storage");
    let storage2 = StorageEngine::new(disk);

    test_storage!(test_page_write_and_get, storage, storage2);
}

fn test_page_write_and_get(s: &StorageEngine) {

}


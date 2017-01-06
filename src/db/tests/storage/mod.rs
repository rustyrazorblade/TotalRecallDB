pub mod disk;
pub mod memory;

#[cfg(test)]
mod tests {
    use db::storage::{Memory, Disk, StorageEngine};
    use tempdir::TempDir;

    #[test]
    fn test_storage_engine_creation() {
        let mem = Memory::new().expect("Memory storage");
        let storage = StorageEngine::new(mem);

        let tmp = TempDir::new("totalrecallwhatever").expect("Expected dir");
        let mut tmp2 = tmp.into_path();
        tmp2.push("blah");

        let disk = Disk::new(10, tmp2).expect("Disk storage");
        let s2 = StorageEngine::new(disk);
    }

}

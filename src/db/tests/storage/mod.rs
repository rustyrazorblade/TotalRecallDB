pub mod disk;
pub mod memory;

#[cfg(test)]
mod tests {
    use db::storage::{Memory, Disk, StorageEngine};

    #[test]
    fn test_storage_interface() {
        let mem = Memory::new().expect("Memory storage");
        let storage = StorageEngine::new(mem);

    }

}

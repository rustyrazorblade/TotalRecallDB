
use std::path::PathBuf;
use super::{Storage, StorageError, StorageResult};
use super::Page;

struct Disk {
    segment_size: usize, // bytes
    directory: PathBuf,
}

impl Disk {
    fn new(segment_size: usize, dir: PathBuf) -> StorageResult<Disk> {
        Ok(Disk{segment_size:segment_size,
                directory: dir})
    }
}


// on disk storage will have to be broken into segments.
// a segment will be a fixed number of pages
struct Segment {

}

impl Storage for Disk {
    fn get_page(_: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
}

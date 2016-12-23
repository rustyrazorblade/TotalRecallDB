
use super::{Storage, StorageError, StorageResult};
use super::Page;

struct Disk {
    segment_size: usize, // bytes
    directory: String,
}

impl Disk {
    fn new(segment_size: usize, dir: &str) -> StorageResult<Disk> {
        Ok(Disk{segment_size:segment_size,
                directory: dir.to_string()})
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

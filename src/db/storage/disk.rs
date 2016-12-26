
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;

use super::{Storage, StorageError, StorageResult};
use super::{Page, PAGE_SIZE};

pub struct Disk {
    segment_size: usize, // bytes
    pages_per_segment: usize,
    directory: PathBuf,
    first_segment: u64,
    segments: Vec<Segment>,
}

impl Disk {
    pub fn new(segment_size_in_mb: usize, dir: PathBuf) -> StorageResult<Disk> {
        let segment_size = segment_size_in_mb * 1024 * 1024;
        let pages_per_segment = segment_size / PAGE_SIZE;
        Ok(Disk{segment_size: segment_size,
                directory: dir,
                pages_per_segment: pages_per_segment,
                segments: Vec::new(),
                first_segment: 0})
    }
}


// on disk storage will have to be broken into segments.
// a segment will be a fixed number of pages
struct Segment {
    fp: File,
}
enum SegmentError {
    FlushFailure
}
type SegmentResult<T> = Result<T, SegmentError>;

impl Segment {
    fn new(location: &Path) -> SegmentResult<Segment> {
        let fp = File::create(location).expect("Could not created segment");
        Ok(Segment{fp: fp})
    }

    // flushes the current page to disk
    fn flush() -> SegmentResult<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod segment_tests {
    use tempdir::TempDir;
    use super::Segment;
    #[test]
    fn test_normal_segment_usage() {
        let dir = TempDir::new("total_recall_segments").expect("Couldn't make a temp dir");
        let d2 = dir.path().join("segment.seg");
        let segment = Segment::new(&d2);
    }
}

impl Storage for Disk {
    // getting a page requires first getting the right segment

    fn get_page(&self, _: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
    fn insert(&mut self, row: &[u8]) -> StorageResult<()> {
        unimplemented!()
    }
}

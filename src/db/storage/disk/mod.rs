pub mod segment;

use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::fs;

use super::{Storage, StorageError, StorageResult};
use super::{Page, PAGE_SIZE, PageError};

pub use self::segment::Segment;


pub struct Disk {
    segment_size: usize, // bytes
    pages_per_segment: usize,
    directory: PathBuf,
    first_segment: u64,
    segments: Vec<Segment>,
    current_page: Page,
    current_segment: Segment,
}

impl Disk {

    // dir is the directory of the segments
    pub fn new(segment_size_in_mb: usize, dir: PathBuf) -> StorageResult<Disk> {
        // create the data directory for this disk storage
        fs::create_dir(&dir).expect("Could not create disk storage");

        let segment_size = segment_size_in_mb * 1024 * 1024;
        let pages_per_segment = segment_size / PAGE_SIZE;

        // make sure the directory is empty
        let seg_path = dir.as_path().join("seg1.segment");
        println!("Segment: {:?}", seg_path);
        let segment = Segment::new(&seg_path).expect("Disk Storage: Could not create segment");

        Ok(Disk{segment_size: segment_size,
            directory: dir,
            pages_per_segment: pages_per_segment,
            segments: Vec::new(),
            first_segment: 0,
            current_page: Page::new(),
            current_segment: segment})
    }


}



impl Storage for Disk {
    // getting a page requires first getting the right segment

    fn get_page(&self, _: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
    fn insert(&mut self, row: &[u8]) -> StorageResult<()> {


        Err(StorageError::StreamAllocationError)
    }
}

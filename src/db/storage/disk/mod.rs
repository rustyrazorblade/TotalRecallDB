pub mod segment;

use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::fs;

use super::{Storage, StorageError, StorageResult};
use super::{Page, PAGE_SIZE, PageError};
use super::{Row, RowBuilder};

pub use self::segment::Segment;


pub struct Disk {
    segment_size: usize, // bytes
    pages_per_segment: usize,
    directory: PathBuf,
    first_segment: u64,
    segments: Vec<Segment>,
    current_page: Page,
    current_segment: Segment,
    segment_sequence_id: u64,
}

impl Disk {

    // dir is the directory of the segments
    pub fn new(segment_size_in_mb: usize, dir: PathBuf) -> StorageResult<Disk> {
        // create the data directory for this disk storage
        fs::create_dir(&dir).expect("Could not create disk storage");

        let segment_size = segment_size_in_mb * 1024 * 1024;
        let pages_per_segment = segment_size / PAGE_SIZE;

        let segment = Disk::open_segment(&dir, 0).expect("Could not open new segment");

        Ok(Disk{segment_size: segment_size,
            directory: dir,
            pages_per_segment: pages_per_segment,
            segments: Vec::new(),
            first_segment: 0,
            current_page: Page::new(),
            current_segment: segment,
            segment_sequence_id: 0})
    }

    pub fn open_segment(dir: &PathBuf, id: u64) -> StorageResult<Segment> {
        let name = format!("seg{}.segment", id);
        let seg_path = dir.as_path().join(name);
        info!("Creating new segment at {:?}", seg_path);
        let segment = Segment::new(&seg_path).expect("Disk Storage: Could not create segment");
        Ok(segment)

    }

    // close the current segment and open a new one
    pub fn flush(&mut self) -> StorageResult<()> {
        info!("Flushing segment {}", self.segment_sequence_id);
        info!("Current segment: {:?}", self.current_segment);
        self.segment_sequence_id += 1;
//        self.current_segment.close();
        let segment = Disk::open_segment(&self.directory, self.segment_sequence_id).expect("Expected segment");
        self.current_segment = segment;
        Ok(())
    }


}



impl Storage for Disk {

    // getting a page requires first getting the right segment
    fn get_page(&self, page: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
    fn write_page(&mut self, page: &Page) -> StorageResult<()> {
        // write the page to the current segment
        // check if the segment is full
        // if it's full, close and open a new one
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::{Disk, Page};
    use tempdir::TempDir;

    fn get_disk_storage() -> Disk {
        let dir = TempDir::new("disk_storage").expect("Need a temp dir");
        let dir2 = dir.path().join("stream");
        Disk::new(1, dir2).expect("Want that disk")
    }

    #[test]
    fn test_disk_flush() {
        let mut disk = get_disk_storage();
        disk.flush();
        disk.flush();
    }

    #[test]
    fn test_disk_writes_segments_correctly() {

    }
}
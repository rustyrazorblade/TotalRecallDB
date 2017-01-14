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
    current_segment: Segment,
    // number of the current segment
    segment_sequence_id: u64,
    flushes: usize,

}

impl Disk {

    // dir is the directory of the segments
    pub fn new(segment_size_in_mb: usize, dir: PathBuf) -> StorageResult<Disk> {
        // create the data directory for this disk storage
        fs::create_dir(&dir).expect("Could not create disk storage");

        let segment_size = segment_size_in_mb * 1024 * 1024;
        let pages_per_segment = segment_size / PAGE_SIZE;
        info!("pages per segment: {}", pages_per_segment);

        let segment = Disk::open_segment(&dir, 0).expect("Could not open new segment");

        Ok(Disk{segment_size: segment_size,
            directory: dir,
            pages_per_segment: pages_per_segment,
            segments: Vec::new(),
            first_segment: 0,
            current_segment: segment,
            segment_sequence_id: 0,
            flushes: 0})
    }

    pub fn open_segment(dir: &PathBuf, segment_id: u64) -> StorageResult<Segment> {
        info!("Creating new segment at {:?}", dir);
        let segment = Segment::new(&dir, segment_id, 0).expect("Disk Storage: Could not create segment");
        Ok(segment)

    }

    fn set_pages_per_segment(&mut self, pages_per_segment: usize) {
        self.pages_per_segment = pages_per_segment;
    }

    // close the current segment and open a new one
    pub fn flush(&mut self) -> StorageResult<()> {
        info!("Flushing segment {}", self.segment_sequence_id);
        info!("Current segment: {:?}", self.current_segment);
        self.segment_sequence_id += 1;
//        self.current_segment.close();
        let segment = Disk::open_segment(&self.directory, self.segment_sequence_id).expect("Expected segment");
        self.current_segment = segment;
        self.flushes += 1;
        Ok(())
    }


}



impl Storage for Disk {

    // getting a page requires first getting the right segment
    fn get_page(&self, page: u64) -> StorageResult<Page> {
        Err(StorageError::PageNotFound)
    }
    fn write_page(&mut self, page: &Page) -> StorageResult<()> {
        self.current_segment.write(&page);
        // if the segment is full, flush
        if self.current_segment.pages >= self.pages_per_segment {
            self.flush();
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::{Disk, Page};
    use db::storage::Storage;
    use tempdir::TempDir;

    fn get_disk_storage() -> Disk {
        let mut dir = TempDir::new("disk_storage").expect("Need a temp dir").into_path();
        dir.push("stream");
        Disk::new(1, dir).expect("Want that disk")
    }

    #[test]
    fn test_disk_flush() {
        let mut disk = get_disk_storage();
        disk.flush();
        assert_eq!(disk.flushes, 1);
        disk.flush();
        assert_eq!(disk.flushes, 2);

    }

    #[test]
    fn test_disk_writes_segments_correctly() {
        let mut disk = get_disk_storage();
        disk.set_pages_per_segment(2); // flush after every page

        let mut page = Page::new();
        let data: [u8; 16] = [0; 16];
        page.write(&data);

        disk.write_page(&page);
        assert_eq!(disk.flushes, 0);

        disk.write_page(&page);
        assert_eq!(disk.flushes, 1);

        disk.get_page(0);

    }
}
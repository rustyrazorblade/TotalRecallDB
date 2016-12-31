use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{SeekFrom, Seek};
use std::io::Write;

use super::{Page, PAGE_SIZE};

// on disk storage will have to be broken into segments.
// a segment will be a fixed number of pages
#[derive(Debug)]
pub struct Segment {
    fp: File,
    pages: usize,
}
#[derive(Debug)]
pub enum SegmentError {
    FlushFailure
}
type SegmentResult<T> = Result<T, SegmentError>;

impl Segment {
    pub fn new(location: &Path) -> SegmentResult<Segment> {
        info!("Creating segment at: {:?}", location);
        let fp = File::create(location).expect("Could not created segment");
        Ok(Segment{fp: fp, pages: 0})
    }


    pub fn write(&mut self, data: &Page) -> SegmentResult<()> {
        // append a page to the current segment
        // job of the Disk storage to flush when ready
        // seek to the end of the segment
        self.fp.seek(SeekFrom::End(0));
        self.fp.write(&data.to_bytes()).expect("Write failed");
        self.fp.flush();
        self.pages += 1;
        Ok(())
    }

    fn read_page(&self, page: u64) -> Page {
        unimplemented!()
    }
}

#[cfg(test)]
mod segment_tests {
    use tempdir::TempDir;
    use super::Segment;
    use super::Page;

    #[test]
    fn test_normal_segment_usage() {
        let dir = TempDir::new("total_recall_segments").expect("Couldn't make a temp dir");
        info!("Created temp dir {:?}", dir);
        let d2 = dir.path().join("segment.seg");
        let mut segment = Segment::new(&d2).expect("Could not create segment");

        let mut page = Page::new();
        let data: [u8; 1024] = [1; 1024];
        page.write(&data);
        segment.write(&page);

        dir.into_path();



    }
}
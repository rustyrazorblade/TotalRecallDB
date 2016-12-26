use std::fs::File;
use std::path::{Path, PathBuf};
use super::{Page, PAGE_SIZE};

// on disk storage will have to be broken into segments.
// a segment will be a fixed number of pages
#[derive(Debug)]
pub struct Segment {
    fp: File,
}
#[derive(Debug)]
pub enum SegmentError {
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

    fn write(&mut self, data: &[u8]) -> SegmentResult<()> {
        // append
        unimplemented!()
    }
    fn read_page(&self, page: u64) -> Page {
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
        let segment = Segment::new(&d2).expect("Could not create segment");

    }
}
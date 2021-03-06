use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{SeekFrom, Seek};
use std::io::{Write, Read};

use super::{Page, PAGE_SIZE, PageError};

/// Segments are a bunch of of Pages organized into files
/// A segment might be a few hundred (or thousand pages)
/// Currently each segment has 2 files, a data file and metadata

// on disk storage will have to be broken into segments.
// a segment will be a fixed number of pages
#[derive(Debug)]
pub struct Segment {
    fp: File,
    meta: File,
    pub pages: usize,
    row_offsets: Vec<usize>, // index is the page index of the segment value is the first id
    start_page: u64, // offset.  page 0 in first segment is 0.


}
#[derive(Debug)]
pub enum SegmentError {
    FlushFailure,
    PageNotFound,
    UnknownError,
}

// TODO fix this up a bit
impl From<PageError> for SegmentError {
    fn from(err: PageError) -> SegmentError {
        match err {
            PageError::Full =>
                SegmentError::PageNotFound,
            PageError::NotFound =>
                SegmentError::PageNotFound,
        }
    }
}


type SegmentResult<T> = Result<T, SegmentError>;

impl Segment {

    pub fn new(location: &Path, num: u64, start_page: u64) -> SegmentResult<Segment> {
        let name = format!("segment-{}.data", num);
        let seg_path = location.join(name);

        let name = format!("segment-{}.meta", num);
        let meta_path = location.join(name);

        info!("Creating segment at: {:?}", seg_path);

        let fp = File::create(seg_path).expect("Could not created segment");
        let meta = File::create(meta_path).expect("Could not created segment metadata");

        Ok(Segment{fp: fp, pages: 0, meta: meta,
                   row_offsets: Vec::new(),
                   start_page: start_page})
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

    fn read_page(&mut self, page: u64) -> SegmentResult<Page> {
        let p = page - self.start_page;
        let offset = p * PAGE_SIZE as u64;
        self.fp.seek(SeekFrom::Start(offset));

        let mut data : Vec<u8> = Vec::new();
        self.fp.read(&mut data);

        let page = Page::from_bytes(data)?;
        Ok(page)

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
        let mut segment = Segment::new(dir.path(), 0, 0).expect("Could not create segment");

        let mut page = Page::new();
        let data: [u8; 1024] = [1; 1024];
        page.write(&data);
        segment.write(&page);

        dir.into_path();

        segment.read_page(0).expect("Page should be OK");

    }
}
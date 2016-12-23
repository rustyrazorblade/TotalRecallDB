use super::Storage;
use db::storage::Page;

// in memory storage.  has a bunch of pages in a vector
struct Memory {
    pages: Vec<Page>
}

impl Storage for Memory {

}
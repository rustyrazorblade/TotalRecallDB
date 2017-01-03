use super::Storage;
use super::Page;

use db::row::Row;
use db::schema::Schema;
use db::storage::page::{PageResult, PageError};

struct StorageEngine<'a> {
    storage: Box<Storage + 'a>,
    current_page: Page,
}

impl<'a> StorageEngine<'a> {
    pub fn new<T:Storage + 'a>(storage: T) -> StorageEngine<'a> {
        let page = Page::new();
        let s = Box::new(storage);
        StorageEngine{storage:s,
                      current_page: page}
    }

    pub fn insert(&mut self, row: &Row, schema: &Schema) {
        // convert to bytes
        let data = row.to_vec();

        // is there room in the current page?
        if let Err(PageError::Full) = self.current_page.write(&data) {
            self.storage.write_page(&self.current_page);
            self.current_page = Page::new();
            self.current_page.write(&data);
        }
    }

    pub fn get_by_pk(&mut self, id: u64) {

    }

}
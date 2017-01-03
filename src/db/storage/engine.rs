use super::Storage;
use super::Page;


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
}

pub mod parser;

use db::Database;

fn create_test_db() -> Database {
    let mut db = Database::new();
    // create a sample stream
    {
        let stream = db.create_stream("test").expect("Could not create stream");

    }
    db
}

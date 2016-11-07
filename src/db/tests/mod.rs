
pub mod parser;
pub mod expressions;

use db::Database;
use db::schema::{Type, Schema};
use db::stream::Stream;
use db::row::RowBuilder;


fn get_sample_schema() -> Schema {
    let mut s = Schema::new();
    s.add_type("name", Type::String);
    s.add_type("age", Type::Int);
    s
}

fn create_test_db() -> Database {
    let mut db = Database::new();
    // create a sample stream
    {
        let stream = db.create_stream("test").expect("Could not create stream");
        (*stream).add_type("name", Type::String);
        (*stream).add_type("age", Type::Int);

    }
    db
}

fn populate_stream(stream: &mut Stream, rows: u64) {
    for x in 0..rows {
        let age = x + 20;
        let name = format!("Test {}", x);
        let mut builder = RowBuilder::new();
        builder.set_string("name", &name);
        builder.set_int("age", age as i64);
        stream.insert(builder);
    }
}

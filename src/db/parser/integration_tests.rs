use db::database::{Database, QueryResult};
use db::schema::Type;

#[test]
fn real_db_insert_parsing() {
    let mut db = Database::new();
    {
        let mut stream = db.create_stream("users").unwrap();
        stream.schema.add_type("age", Type::String);
    }

    let q= r"INSERT INTO users set age=1;";
    if let QueryResult::Insert(result) = db.execute(q).unwrap() {
        assert_eq!(result, 0);
    } else {
        panic!("Everything on fire");
    }

    let q= r"INSERT INTO users set age=2;";
    if let QueryResult::Insert(result) = db.execute(q).unwrap() {
        assert_eq!(result, 1);
    } else {
        panic!("Everything on fire");
    }
}

#[test]
fn define_stream_test() {
    let query = r#""#;
}
use db::database::Database;
use db::schema::Type;

#[test]
fn real_db_insert_parsing() {
    let mut db = Database::new();
    {
        let mut stream = db.create_stream("users").unwrap();
        stream.schema.add_type("name", Type::String);
    }
    let q= r#"INSERT INTO users set name="Jon";"#;
    let result = db.execute(q);
}
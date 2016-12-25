use db::database::{Database, QueryResult, DatabaseError};
use db::schema::Type;

fn get_db() -> Database {
    let mut db = Database::new_temp();
    {
        let mut stream = db.create_stream("users").unwrap();
        stream.schema.add_type("name", Type::String);
        stream.schema.add_type("age", Type::Int);
    }
    db
}

#[test]
fn real_db_insert_parsing() {
    let mut db = Database::new_temp();
    {
        let mut stream = db.create_stream("users").unwrap();
        stream.schema.add_type("age", Type::String);
    }

    let q = r"INSERT INTO users set age=1;";
    if let QueryResult::Insert(result) = db.execute(q).unwrap() {
        assert_eq!(result, 0);
    } else {
        panic!("Everything on fire");
    }

    let q = r"INSERT INTO users set age=2;";
    if let QueryResult::Insert(result) = db.execute(q).unwrap() {
        assert_eq!(result, 1);
    } else {
        panic!("Everything on fire");
    }
}

#[test]
fn test_quoted_string_insert() {

    let mut db = Database::new_temp();
    {
        let mut stream = db.create_stream("users").unwrap();
        stream.schema.add_type("name", Type::String);
    }

    let q= "INSERT INTO users set name='Jon';";
    if let QueryResult::Insert(result) = db.execute(q).unwrap() {
        assert_eq!(result, 0);
    } else {
        panic!("Everything on fire");
    }
}

#[test]
fn test_schema_validation() {
    let mut db = get_db();
    if let DatabaseError::FieldNotFound(x) = db.execute("insert into users set name = 'Jon', age = 35, pie=3;").unwrap_err() {

    } else {
        panic!("Was expecting a FieldNotFound for pie");
    }
}

#[test]
fn define_stream_test() {
    let mut db = get_db();
    db.execute("declare stream data ( id int, name text, reading text);").unwrap();

}
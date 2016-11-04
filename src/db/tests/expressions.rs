extern crate env_logger;

use super::get_sample_schema;
use db::row::{RowBuilder, RowReader};
use db::parser::parse_statement;
use db::parser::where_clause;
#[test]
fn test_evaluate_simple_equality() {
    env_logger::init();
    let s = get_sample_schema(); // name & age
    let mut rb = RowBuilder::new();
    rb.set_string("name", "jon");
    rb.set_int("age", 35);

    let row = rb.to_row(&s).expect("Was expecting a valid row");
    let reader = RowReader::new(&s, &row);

    let p = where_clause("WHERE age = 35").expect("where age = 35");
    info!("WHERE age = 35 -> {:?}", p);

    assert!(reader.evaluate(p)); // expecting true

    let p = where_clause("WHERE age = 36").expect("where age = 36");
    assert!(!reader.evaluate(p));

}

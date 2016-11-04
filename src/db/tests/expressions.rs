use super::get_sample_schema;
use db::row::RowBuilder;



#[test]
fn test_evaluate_simple_equality() {
    let s = get_sample_schema(); // name & age
    let mut rb = RowBuilder::new();
    rb.set_string("name", "jon");
    rb.set_int("age", 35);

    let row = rb.to_row(&s);


}

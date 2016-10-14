pub use super::schema::Schema;
pub use super::row::Row;
pub use super::value::Value;

pub struct RowReader<'a> {
    schema: &'a Schema,
    row: &'a Row
}

impl<'a> RowReader<'a> {
    fn new(schema: &'a Schema, row: &'a Row) -> RowReader<'a> {
        RowReader{schema: schema, row: row}
    }
    fn get(&self, name: &str) -> Option<Value> {
        None
    }
}

mod tests {
    use super::*;
    use db::row_builder::RowBuilder;
    use db::stream::Stream;
    use db::schema::Type;

    #[test]
    fn test_row_reader_simple() {
        let mut stream = Stream::new();
        stream.schema.add_type("name", Type::String);

        let mut row = RowBuilder::new();
        row.set_string("name", "Jon");

        let id = stream.insert(row);

        // TODO make stream.get return the RowReader
        let result = stream.get(0).unwrap();
        {
            let reader = RowReader::new(&stream.schema, &result);
            let name = reader.get("name");
        }
    }

}
pub use super::schema::Schema;
pub use super::row::Row;

struct RowReader {
    schema: Schema,
    row: Row
}

mod tests {
    use super::*;
    use db::row_builder::RowBuilder;
    use db::stream::Stream;

    #[test]
    fn test_row_reader_simple() {
        let mut row = RowBuilder::new();
        let mut stream = Stream::new();
        stream.insert(row);

    }

}
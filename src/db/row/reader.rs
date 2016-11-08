#![feature(box_syntax, box_patterns)]

pub use db::schema::Schema;
pub use db::row::Row;
pub use db::value::{Value, TypedValue};
use db::parser::{Expression, Operator};

pub struct RowReader<'a> {
    schema: &'a Schema,
    row: Row
}

impl<'a> RowReader<'a> {
    pub fn new(schema: &'a Schema, row: Row) -> RowReader<'a> {
        RowReader{schema: schema, row: row}
    }
    pub fn get(&self, name: &str) -> Option<TypedValue> {

        let result = self.schema.get(name).and_then(
            |field| self.row.get(field.id)
        ).and_then(
            |value| Some(TypedValue::new(value.clone(), self.schema.get(name).unwrap().dbtype.clone()))

        );
        result
    }

    // checks if a row matches a given predicate
    // i'm going to assume here all the type checks have been done
    // so tests are going to pass that reference evaluate directly
    pub fn evaluate(&self, expression: &Box<Expression>) -> bool {
        debug!("Evaluating: {:?}", expression);
        self.e(expression).to_bool()
    }

    // internal evaluation, returning Values all the way up
    fn e(&self, expression: &Box<Expression>) -> TypedValue {
        debug!("E: {:?}", expression);

        match **expression {
            Expression::Value(ref v) => v.clone(),
            Expression::Field(ref s) => {
                // should return the value for the field
                debug!("Getting value of field {:?}", s);
                let tmp = self.get(&s).unwrap();
                debug!("Returning value {:?} for expression Field({:?}", tmp, s);
                tmp

            },
            Expression::Comparison(ref operator, ref lhs, ref rhs) => {
                debug!("Parser evaluating comparison operation {:?} and {:?}", lhs, rhs);
                let lhs2 = self.e(lhs);
                let rhs2 = self.e(rhs);
                self.compare(&operator, &lhs2, &rhs2)
            },

            _ => {
                debug!("Not sure what to do on internal parser evaluation, returning false");
                TypedValue::from(false)
            }
        }
    }

    fn compare(&self, operator: &Operator,
               lhs: &TypedValue,
               rhs: &TypedValue) -> TypedValue {

        // finish the evaluation of the left and right sides
        let tmp = match *operator {
            Operator::Equal => lhs == rhs,
            Operator::GreaterThan => lhs > rhs,
            Operator::GreaterThanEqual => lhs >= rhs,
            Operator::LessThan => lhs < rhs,
            Operator::LessThanEqual => lhs <= rhs,
            _ => false
        };
        TypedValue::from(tmp)

    }
}

mod tests {
    use super::*;
    use db::row::RowBuilder;
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
    }


}
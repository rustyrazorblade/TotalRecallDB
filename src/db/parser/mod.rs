use self::streamql::statement;
pub use self::streamql::ParseError;
use db::row::RowBuilder;
use super::schema::Type;
use super::value::{Value, TypedValue};
pub use self::streamql::where_clause;
mod integration_tests;

peg_file! streamql("streamql.rustpeg");

pub fn parse_statement(query: &str) -> Result<Statement, ParseError> {
    let tmp = statement(query);
    debug!("Parsed \"{}\" as query: \n{:?}", query, tmp);
    tmp
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Stream & Keys
    Insert(String, RowBuilder),
    // name
    DeclareStream(String, Vec<ColumnSpec>),
    DropStream,
    UseDatabase,

    // Stream, Where cause
    Select(String, Option<Box<Expression>>),
    Subscribe,
}

// infix operators
// used for 2 expressions
// 2 expressions must evaluate to a bool
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    And,
    Or,
    Plus,
    Minus,
    Multiply,
    Divide,
    Like,
}

#[derive(Debug, Clone)]
pub enum Fields {
    All,
    Named(Vec<String>),
}



#[derive(Debug, Clone)]
pub enum Expression {
    Value(TypedValue),
    Comparison(Operator, Box<Expression>, Box<Expression>),
    Function(String, Vec<Expression> ),
    Field(String), // field name
    None,
}

#[derive(Debug, Clone)]
pub struct ColumnSpec {
    pub name: String,
    pub ftype: Type,
}

impl ColumnSpec {
    fn new(name: String, ftype: Type) -> ColumnSpec {
        ColumnSpec{name: name,
                   ftype: ftype}
    }
}

#[cfg(test)]
mod test {
    use super::parse_statement;
    use super::{Statement, Expression, Operator};
    use super::streamql::*;
    use db::value::{Value, TypedValue};

    #[test]
    fn test_basic_insert() {

        let result = parse_statement("INSERT INTO test set k=1;").unwrap();

        if let Statement::Insert(stream, builder) = result {
            assert_eq!(stream, String::from("test"));
        } else {
            panic!("")
        }
        let x = "INSERT INTO test set k=1, v=2;";
        let result = parse_statement(x).unwrap();

    }

    #[test]
    fn test_parsing_with_quoted_string() {
        let result = parse_statement("INSERT INTO test set k='test';").unwrap();
        let result = parse_statement("INSERT INTO test set k='test my ''friend''';").unwrap();

    }

    #[test]
    fn test_int_parsing() {
        let tmp = int_value("42").unwrap();
    }

    #[test]
    fn test_basic_string_parsing() {
        string("hello this is a test").unwrap();
    }

    #[test]
    fn test_escaped_string() {
        let x = string("hello this is a ''test''").unwrap();
        assert_eq!(x, "hello this is a 'test'");
    }

    #[test]
    fn test_quoted_string() {
        let x = "'it''s mine'";
        let p = quoted_string(x).unwrap();
        assert_eq!(p, "it's mine");
    }

    #[test]
    fn value_string_parsing() {
        let x = "'hello'";
        if TypedValue::from(x) == value(x).unwrap() {
            assert_eq!(x, "hello");
        }

    }

    #[test]
    fn declare_stream_parsing() {
        let x = "declare stream readings ( ts int, val int, s text );";
        let p = parse_statement(x).unwrap();
    }

    #[test]
    fn test_basic_parse_column_specification() {
        let x = "ts int";
        let p = column_spec(x).unwrap();

        let x = "ts text";
        let p = column_spec(x).unwrap();
    }

    #[test]
    fn test_column_spec_list() {
        let x = "ts int, val int, s text ";
        let p = column_spec_list(x).unwrap();

        assert_eq!(p.len(), 3);

        assert_eq!(p[0].name, "ts");
        assert_eq!(p[1].name, "val");
        assert_eq!(p[2].name, "s");
    }

    #[test]
    fn test_select_queries() {
        let queries = ["select * from test",
                        "select name, age from users",
                        "select name from cities where state = 'CA'"];
        for q in queries.into_iter() {
            parse_statement(&q).expect(q);
        }

    }

    #[test]
    fn test_field_list() {
        let queries = ["name, email",
            "*", "stuff"];
        for q in queries.into_iter() {
            field_list(q).expect(q);
        }
    }

    #[test]
    fn test_where_clause() {
        let queries = ["where name = 'jon'",
        "where id = 5",
        "where name = 'bill' and age > 35"];
        for q in queries.into_iter() {
            where_clause(q).expect(q);
        }

    }

    // expression testing
    #[test]
    fn test_value_expression() {
        let tmp = expression_value("1").expect("1");
    }
    #[test]
    fn test_field_expression() {
        let tmp = expression_field("name")
                   .expect("name didn't parse");
    }

    #[test]
    fn test_operators() {
        expression_operator("name > 5")
            .expect("Expecting name > 5");

        let x = "age > 10 and city = 'Boston'";
        // technically is this:
        // let x = "((age > 10) and (city = 'Boston'))";
        // need to rewrite as a bunch of nested binary ops
        // do i need a second parser just to rewrite?
        expression(x).expect(x);

        // same as above but needed to be safe
        let x = "(age > 10) and city = 'Boston'";
        expression(x).expect(x);

        let x = "(age > 10) and (city = 'Boston')";
        expression(x).expect(x);

        let x = "((age > 10) and (city = 'Boston')) or age = 3";
        let y = expression(x).expect(x);

        match *y {
            Expression::Comparison(ref a, ref b, ref c) =>
                {
                    assert_eq!(*a, Operator::Or);
                }
            _ => {
                panic!("NOOO");
            }
        }

        let x = "(field + 3) > 10";
        let x = "10 > (field * 3)";
        expression(x).expect(x);

    }

    #[test]
    fn test_grouping() {
        expression_grouped("(fname > 0)")
            .expect("Expecting name > 5");
        expression("(fname > 0)")
            .expect("Expecting name > 5");


    }


    #[test]
    fn test_and() {
        //expression_and()
    }


}









use super::value::Value;

use gc::GcCell;

use nom::{double, multispace0};

named!(pub value<Value>,
    alt!(string | number | symbol | sexpr));

named!(
    string<Value>,
    map!(
        delimited!(char!('"'), many0!(none_of!("\"")), char!('"')),
        |x| Value::String(x.into_iter().collect())
    )
);

// FIXME: Support numbers in identifiers.
named!(
    symbol<Value>,
    map!(
        many1!(one_of!(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-+?"
        )),
        |x| Value::Symbol(x.into_iter().collect())
    )
);

named!(number<Value>, map!(double, Value::Number));

fn parse_fundamental_forms(contents: Vec<Value>) -> Value {
    if contents.len() == 0 {
        return Value::SExpr(Vec::new());
    }

    // FIXME: Avoid this clone!
    let first = contents[0].clone();
    let rest = contents.into_iter().map(GcCell::new);

    if let Value::Symbol(s) = first {
        match &s as &str {
            "if" => return Value::If(rest.skip(1).collect()),
            _ => {}
        }
    }

    Value::SExpr(rest.collect())
}

named!(
    sexpr<Value>,
    map!(
        delimited!(
            char!('('),
            many0!(delimited!(multispace0, value, multispace0)),
            char!(')')
        ),
        parse_fundamental_forms
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        assert_eq!(
            string(br#""hello, world!""#).unwrap().1,
            Value::String("hello, world!".to_owned())
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(number(b"123 ").unwrap().1, Value::Number(123f64));

        assert_eq!(number(b"42.69 ").unwrap().1, Value::Number(42.69));

        assert_eq!(number(b"1e6 ").unwrap().1, Value::Number(1e6));

        assert_eq!(number(b"-5.123e3 ").unwrap().1, Value::Number(-5.123e3));
    }

    #[test]
    fn test_symbol() {
        assert_eq!(symbol(b"add ").unwrap().1, Value::Symbol("add".to_owned()));
        assert_eq!(
            symbol(b"even-p ").unwrap().1,
            Value::Symbol("even-p".to_owned())
        );
        assert_eq!(symbol(b"+ ").unwrap().1, Value::Symbol("+".to_owned()));
        assert_eq!(symbol(b"-_- ").unwrap().1, Value::Symbol("-_-".to_owned()));
    }

    #[test]
    fn test_sexpr() {
        assert_eq!(
            sexpr(br#"(print -5.123e3 "hello, world" (add 41 1))"#)
                .unwrap()
                .1,
            Value::SExpr(vec![
                GcCell::new(Value::Symbol("print".to_owned())),
                GcCell::new(Value::Number(-5.123e3)),
                GcCell::new(Value::String("hello, world".to_owned())),
                GcCell::new(Value::SExpr(vec![
                    GcCell::new(Value::Symbol("add".to_owned())),
                    GcCell::new(Value::Number(41f64)),
                    GcCell::new(Value::Number(1f64)),
                ])),
            ])
        );
    }
}

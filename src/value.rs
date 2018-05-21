#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),

    String(String),

    Symbol(String),

    // XXX: Should we use `std::collections::LinkedList`?
    SExpr(Vec<Value>),
}

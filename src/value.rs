use gc::GcCell;

use std::fmt;

#[derive(Clone, Trace, Finalize)]
pub enum Value {
    Number(f64),

    String(String),

    Symbol(String),

    // XXX: Should we use `std::collections::LinkedList`?
    SExpr(Vec<GcCell<Value>>),

    FundamentalForm{name: &'static str, func: Box<fn(Vec<Value>) -> Value>},
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => n == m,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Symbol(s1), Value::Symbol(s2)) => s1 == s2,
            (Value::SExpr(v1), Value::SExpr(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => f.debug_tuple("Value::Number").field(&n).finish(),

            Value::String(s) => f.debug_tuple("Value::String").field(&s).finish(),

            Value::Symbol(s) => f.debug_tuple("Value::Symbol").field(&s).finish(),

            Value::SExpr(v) => f.debug_tuple("Value::SExpr").field(&v).finish(),

            Value::FundamentalForm{name, .. } => f.debug_struct("Value::FundamentalForm").field("name", name).finish(),
    }
    }
}

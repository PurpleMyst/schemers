use super::scope::Scope;

use gc::GcCell;

use std::fmt;

#[derive(Clone, Finalize)]
pub enum Value {
    Number(f64),

    String(String),

    Symbol(String),

    // XXX: Should we use `std::collections::LinkedList`?
    SExpr(Vec<GcCell<Value>>),

    True,
    False,

    FundamentalForm {
        name: &'static str,
        func: Box<fn(&mut Scope, Vec<GcCell<Value>>) -> GcCell<Value>>,
    },
}

// XXX: I have zero idea if this is correct. The documentation about how to actually implement
// `Trace` is missing.
macro_rules! derive_but_with_extra_steps {
    ($($name:ident),*) => {
        $(unsafe fn $name(&self) {
            match self {
                Value::Number(..) | Value::String(..) | Value::Symbol(..) | Value::True | Value::False | Value::FundamentalForm{..} => {},

                Value::SExpr(v) => v.iter().for_each(|i| i.$name()),
            }
        })*
    }
}

unsafe impl ::gc::Trace for Value {
    derive_but_with_extra_steps!(trace, root, unroot);

    fn finalize_glue(&self) {
        ::gc::Finalize::finalize(self)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => n == m,
            (Value::String(s1), Value::String(s2)) | (Value::Symbol(s1), Value::Symbol(s2)) => {
                s1 == s2
            }
            (Value::SExpr(v1), Value::SExpr(v2)) => v1 == v2,
            (Value::True, Value::True) => true,
            (Value::False, Value::False) => true,
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

            Value::True => f.debug_tuple("Value::True").finish(),
            Value::False => f.debug_tuple("Value::False").finish(),

            Value::FundamentalForm { name, .. } => f
                .debug_struct("Value::FundamentalForm")
                .field("name", name)
                .finish(),
        }
    }
}

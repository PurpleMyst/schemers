use gc::GcCell;

#[derive(Debug, Clone, Finalize)]
pub enum Value {
    Number(f64),

    String(String),

    Symbol(String),

    SExpr(Vec<GcCell<Value>>),

    If(Vec<GcCell<Value>>),

    True,
    False,
}

// XXX: I have zero idea if this is correct. The documentation about how to actually implement
// `Trace` is missing.
macro_rules! derive_but_with_extra_steps {
    ($($name:ident),*) => {
        $(unsafe fn $name(&self) {
            match self {
                Value::Number(..) | Value::String(..) | Value::Symbol(..) | Value::True | Value::False  => {},

                Value::SExpr(v)|Value::If(v) => v.iter().for_each(|i| i.$name()),
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

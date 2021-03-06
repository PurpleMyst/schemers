use super::value::Value;

use gc::GcCell;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    variables: HashMap<String, GcCell<Value>>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn prelude() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn enter_sub_scope(self) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(self)),
        }
    }

    #[allow(dead_code)]
    pub fn exit_sub_scope(self) -> Option<Self> {
        self.parent.map(|b| *b)
    }

    // XXX: We can probably rewrite this to utilize tail recursion.
    fn eval_symbol(&self, symbol: &str) -> Option<GcCell<Value>> {
        self.variables.get(symbol).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.eval_symbol(symbol))
        })
    }

    pub fn eval(&mut self, value: &GcCell<Value>) -> GcCell<Value> {
        match &*value.borrow() {
            // XXX: We can probably avoid a clone here if we use a `Cow`. The question is, do we
            // want to?
            Value::Number(..) | Value::String(..) | Value::True | Value::False => value.clone(),

            Value::Symbol(s) => {
                if let Some(result) = self.eval_symbol(&s) {
                    result
                } else {
                    panic!("Undefined variable {:?} in scope {:?}", s, self);
                }
            }

            Value::SExpr(contents) => {
                let func = self.eval(&contents[0]);
                let bfunc = func.borrow();

                match &*bfunc {
                    _ => panic!("Tried to call {:?}", func),
                }
            }

            Value::If(contents) => {
                let condition = self.eval(&contents[0]);

                self.eval(if &*condition.borrow() != &Value::False {
                    &contents[1]
                } else {
                    &contents[2]
                })
            }
        }
    }
}

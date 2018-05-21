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
        let mut variables = HashMap::new();

        // NB: Only fundamental forms should be defined via this macro. Once I get the interpreter
        // to a more usable state, I plan on having a `prelude.scm` file.
        macro_rules! prelude_function {
            ($name:expr => $func:expr) => {
                assert!(
                    variables
                        .insert(
                            String::from($name),
                            GcCell::new(Value::FundamentalForm {
                                name: $name,
                                func: Box::new($func)
                            })
                        )
                        .is_none()
                );
            };
        }

        prelude_function!("if" => |scope, args| {
            assert_eq!(args.len(), 3);

            let condition = {
                let condition = scope.eval(&args[0]);
                let condition_borrow = condition.borrow();
                !(condition_borrow.eq(&Value::False))
            };

            if condition {
                scope.eval(&args[1])
            } else {
                scope.eval(&args[2])
            }
        });

        prelude_function!("eq?" => |scope, args| {
            assert_eq!(args.len(), 2);

            let x = scope.eval(&args[0]);
            let y = scope.eval(&args[1]);

            if x.borrow().eq(&y.borrow()) {
                GcCell::new(Value::True)
            } else {
                GcCell::new(Value::False)
            }
        });

        Self {
            variables,
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

            // XXX: Is this actually unreachable?
            Value::FundamentalForm { .. } => unreachable!("s/!/?"),

            Value::SExpr(contents) => {
                let func = self.eval(&contents[0]);
                let bfunc = func.borrow();

                match &*bfunc {
                    Value::FundamentalForm {
                        func: actual_func, ..
                    } => {
                        // XXX: This is inefficient as hell, probably, We can do this in a better
                        // way, somehow.
                        // XXX: Do we need to enter a scope here?
                        actual_func(self, contents.iter().map(Clone::clone).skip(1).collect())
                    }

                    _ => panic!("Tried to call {:?}", func),
                }
            }
        }
    }
}

#[macro_use]
extern crate nom;

extern crate gc;
#[macro_use]
extern crate gc_derive;

mod parser;
mod scope;
mod value;

fn main() {
    let program = parser::value(b"(if (eq? 1 1) 1 2)\0").unwrap().1;
    println!(
        "{:?}",
        scope::Scope::prelude()
            .enter_sub_scope()
            .eval(&gc::GcCell::new(program))
    );
}

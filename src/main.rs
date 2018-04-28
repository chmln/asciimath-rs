// mod lib;
// use lib::parser::{parse, Evaluate, Scope};
extern crate asciimath;
use asciimath::Evaluate;

fn main() {
    let expression = asciimath::parse("10xyz+10").unwrap();

    let mut scope = asciimath::Scope::new();
    scope.set_var("x", 1);
    scope.set_var("y", 2);
    scope.set_var("z", 3);

    println!("{:?}", expression.eval_with(&scope));
}

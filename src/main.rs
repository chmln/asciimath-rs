// mod lib;
// use lib::parser::{parse, Evaluate, Scope};
extern crate asciimath;
use asciimath::Evaluate;

fn main() {
    let expression = asciimath::parse("2(3)").unwrap();

    let mut scope = asciimath::Scope::new();
    scope.set_var("x", 2);

    println!("{:?}", expression.eval_with(&scope));
}

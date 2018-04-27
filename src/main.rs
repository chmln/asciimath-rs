mod lib;
use lib::parser::{parse, Evaluate, Scope};

fn main() {
    // let expr = parse_expr("1*2*3 + 24*5 - 10");
    // println!("{:?}", expr);
    // println!("{}", &expr.eval());3 + 4 * 2 / ( 1 − 5 ) ^ 2 ^ 3
    let mut scope = Scope::new();
    scope.set_var("x", 3);

    println!(
        "{:?}",
        parse("4 + 4 * 2 / ( 1 - 5 ) ^ (2 - 2 ^ 3)").eval_with(&scope)
    );
}

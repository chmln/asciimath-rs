mod lib;

use lib::{parse_expr, Evaluate};

fn main() {
    let expr = parse_expr("1*2*3 + 4 + 15*9 - 100");
    println!("{:?}", expr);
    println!("{}", &expr.eval());
}
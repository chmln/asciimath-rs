mod lib;

use lib::{lexer, rest};

fn main() {
    // let expr = parse_expr("1*2*3 + 24*5 - 10");
    // println!("{:?}", expr);
    // println!("{}", &expr.eval());
    println!("{:?}", lexer::tokenize("1*2*3 + 24*5 - 10"))
}

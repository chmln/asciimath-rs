mod lib;
use lib::parser::{parse, Evaluate};

fn main() {
    // let expr = parse_expr("1*2*3 + 24*5 - 10");
    // println!("{:?}", expr);
    // println!("{}", &expr.eval());3 + 4 * 2 / ( 1 − 5 ) ^ 2 ^ 3
    println!(
        "{:?}",
        parse("3 + 4 * 2 / ( 1 - 5 ) ^ (2 - 2 ^ 3)").eval()
    );
}

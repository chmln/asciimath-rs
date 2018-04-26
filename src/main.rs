mod lib;
use lib::parser;

fn main() {
    // let expr = parse_expr("1*2*3 + 24*5 - 10");
    // println!("{:?}", expr);
    // println!("{}", &expr.eval());
    println!(
        "{:?}",
        parser::parse("3 + 4 * 2 / ( 1 − 5 ) ^ 2 ^ 3")
    );
}

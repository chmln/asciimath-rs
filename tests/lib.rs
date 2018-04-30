extern crate asciimath;
use self::asciimath::{parse, Evaluate, Scope};

#[test]
fn single_item() {
    assert_eq!(Ok(2.0), parse("2").unwrap().eval());
}

#[test]
fn order_of_operations() {
    assert_eq!(
        "3.000122",
        format!(
            "{:.6}",
            parse("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3")
                .unwrap()
                .eval()
                .unwrap()
        )
    );
}

#[test]
fn simple_vars() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);
    assert_eq!(
        Ok(240.0),
        parse("x^2-16").unwrap().eval_with(&scope)
    );
}

#[test]
fn too_many_brackets() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);
    assert_eq!(
        Ok(240.0),
        parse("((((((x^2))-((16))))))")
            .unwrap()
            .eval_with(&scope)
    );
}

#[test]
fn simple_func() {
    assert_eq!(Ok(2.0), parse("max(1,2)").unwrap().eval());
}

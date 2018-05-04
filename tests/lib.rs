extern crate asciimath;
use self::asciimath::{parse, CustomFn, Evaluate, Scope};

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
fn user_defined_functions() {
    let mut scope = Scope::new();
    let my_sum_func: CustomFn = |args| Ok(args.iter().sum::<f64>());

    scope.set_var("sum", my_sum_func);
    assert_eq!(
        Ok(6.0),
        parse("sum(1,2,3)").unwrap().eval_with(&scope)
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
fn func_max() {
    assert_eq!(Ok(2.0), parse("max(1,2)").unwrap().eval());
    assert_eq!(Ok(1.0), parse("max(1)").unwrap().eval());
    assert_eq!(
        Ok(25.75),
        parse("max(1,2,3,25.75,10.5,25.7)")
            .unwrap()
            .eval()
    );
}

#[test]
fn func_min() {
    assert_eq!(Ok(1.0), parse("min(1,2)").unwrap().eval());
    assert_eq!(Ok(1.0), parse("min(1)").unwrap().eval());
    assert_eq!(
        Ok(1.0),
        parse("min(1,2,3,25.75,10.5,25.7)")
            .unwrap()
            .eval()
    );
}

#[test]
fn func_trig() {
    assert_eq!(Ok(1.0), parse("sin(90)").unwrap().eval());
    assert_eq!(Ok(0.5), parse("cos(0)/2").unwrap().eval());
    assert_eq!(
        "0.5",
        format!(
            "{:.1}",
            parse("tan(45) / 2").unwrap().eval().unwrap()
        )
    );
}

#[test]
fn func_nested() {
    assert_eq!(Ok(1.0), parse("abs(abs(-1))").unwrap().eval());
}

#[test]
fn neg_numbers() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);
    assert_eq!(Ok(-1.0), parse("-1").unwrap().eval());
    assert_eq!(
        Ok(15.0),
        parse("x+-1").unwrap().eval_with(&scope)
    );
    assert_eq!(
        Ok(17.0),
        parse("x--1").unwrap().eval_with(&scope)
    );
    assert_eq!(
        Ok(17.0),
        parse("x-(-1)").unwrap().eval_with(&scope)
    );
    assert_eq!(
        Ok(-16.0),
        parse("x*(-1)").unwrap().eval_with(&scope)
    );
}

#[test]
#[should_panic]
fn func_not_enough_args() {
    assert_eq!(Ok(1.0), parse("max()").unwrap().eval());
}

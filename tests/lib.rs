use asciimath::{eval, CustomFn, Error, Scope, Value};
use std::f64;

#[test]
fn single_item() {
    assert_eq!(Ok(2.0.into()), eval("2", &Scope::new()));
}

#[test]
fn order_of_operations() {
    assert_eq!(
        "3.000122",
        format!(
            "{:.6}",
            eval("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3", &Scope::new()).unwrap()
        )
    );
}

#[test]
fn comparison_operators() {
    let mut scope = Scope::new();
    scope.set_var("x", -1);

    assert_eq!(Ok(Value::Bool(false)), eval("x > x", &scope));
    assert_eq!(Ok(Value::Bool(true)), eval("x >= x", &scope));
    assert_eq!(Ok(Value::Bool(true)), eval("x <= x", &scope));
    assert_eq!(Ok(Value::Bool(false)), eval("x < x", &scope));
    assert_eq!(Ok(Value::Bool(false)), eval("x != x", &scope));
    assert_eq!(Ok(Value::Bool(false)), eval("!(x == x)", &scope));
    assert_eq!(Ok(Value::Bool(false)), eval("!((x + 1) == 0)", &scope));
    assert_eq!(eval("x != x", &scope), eval("!(x == x)", &scope));
}

#[test]
fn simple_vars() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);

    assert_eq!(Ok(Value::Num(240.0)), eval("x^2-16", &scope));

    assert_eq!(
        Err(Error::UnknownVariable("y".to_string())),
        eval("y^2-16", &scope)
    );
}

#[test]
fn user_defined_functions() {
    let mut scope = Scope::new();
    let my_sum_func: CustomFn = |args| Ok(args.nums(1)?.sum::<f64>().into());

    scope.set_var("sum", my_sum_func);
    assert_eq!(Ok(Value::Num(6.0)), eval("sum(1,2,3)", &scope));
}

#[test]
fn too_many_brackets() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);
    assert_eq!(
        Ok(Value::Num(240.0)),
        eval("((((((x^2))-((16))))))", &scope)
    );
}

#[test]
fn func_max() {
    assert_eq!(Ok(Value::Num(2.0)), eval("max(1,2)", &Scope::new()));
    assert_eq!(Ok(Value::Num(1.0)), eval("max(1)", &Scope::new()));
    assert_eq!(
        Ok(Value::Num(25.75)),
        eval("max(1,2,3,25.75,10.5,25.7)", &Scope::new())
    );
}

#[test]
fn func_min() {
    assert_eq!(Ok(Value::Num(1.0)), eval("min(1,2)", &Scope::new()));
    assert_eq!(Ok(Value::Num(1.0)), eval("min(1)", &Scope::new()));
    assert_eq!(
        Ok(Value::Num(1.0)),
        eval("min(1,2,3,25.75,10.5,25.7)", &Scope::new())
    );
}

#[test]
fn func_trig() {
    assert_eq!(Ok(Value::Num(1.0)), eval("sin(90)", &Scope::new()));
    assert_eq!(Ok(Value::Num(0.5)), eval("cos(0)/2", &Scope::new()));
    assert_eq!(
        "0.5",
        format!("{:.1}", eval("tan(45) / 2", &Scope::new()).unwrap())
    );
}

#[test]
fn func_log() {
    assert_eq!(Ok(Value::Num(2.0)), eval("log_10(100)", &Scope::new()));
    assert_eq!(Ok(Value::Num(2.0)), eval("log(2, 4)", &Scope::new()));
    assert_eq!(Ok(Value::Num(1.0)), eval("ln(E)", &Scope::new()));
}

#[test]
fn func_floor_ceil() {
    assert_eq!(Ok(Value::Num(2.0)), eval("floor(2.5)", &Scope::new()));
    assert_eq!(Ok(Value::Num(2.0)), eval("ceil(1.51)", &Scope::new()));
}

#[test]
fn func_nested() {
    assert_eq!(Ok(Value::Num(1.0)), eval("abs(abs(-1))", &Scope::new()));
}

#[test]
fn neg_numbers() {
    let mut scope = Scope::new();
    scope.set_var("x", 16);
    assert_eq!(Ok(Value::Num(-1.0)), eval("-1", &scope));
    assert_eq!(Ok(Value::Num(15.0)), eval("x+-1", &scope));
    assert_eq!(Ok(Value::Num(17.0)), eval("x--1", &scope));
    assert_eq!(Ok(Value::Num(17.0)), eval("x-(-1)", &scope));
    assert_eq!(Ok(Value::Num(-16.0)), eval("x*(-1)", &scope));
}

#[test]
fn neg_fn() {
    let mut scope = Scope::new();
    scope.set_var("x", -5);
    assert_eq!(Ok(Value::Num(5.0)), eval("abs(x)", &scope));
    assert_eq!(Ok(Value::Num(4.0)), eval("abs(x + 1)", &scope));
}

#[test]
fn func_not_enough_args() {
    assert_eq!(
        Err(Error::NotEnoughFunctionParams("max".to_string())),
        eval("max()", &Scope::new())
    );
}

#[test]
fn paren_mismatch() {
    assert!(eval("x+())", &Scope::new()).is_err());
}

#[test]
fn constants() {
    assert_eq!(
        Ok(Value::Num(f64::consts::PI * 2.0)),
        eval("2PI", &Scope::new())
    );
}

#[test]
fn division_by_zero() {
    assert_eq!(Ok(Value::Num(f64::INFINITY)), eval("1/0", &Scope::new()));
}

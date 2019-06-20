use crate::ast::{EvaluationResult, NumericLiteral};
use lazy_static::lazy_static;
use std::{collections::HashMap, f64};

pub type Args = Vec<NumericLiteral>;
pub type Func = fn(&Args) -> EvaluationResult;
pub type CustomFn = Func;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, Func> = {
        let mut m = HashMap::with_capacity(15);

        let sin = |args: &Args| Ok(args[0].to_radians().sin());
        let cos = |args: &Args| Ok(args[0].to_radians().cos());
        let tan = |args: &Args| Ok(args[0].to_radians().tan());

        let max = |args: &Args| Ok(args.iter().fold(f64::NAN, |acc: NumericLiteral, x| acc.max(*x)));
        let min = |args: &Args| Ok(args.iter().fold(f64::NAN, |acc: NumericLiteral, x| acc.min(*x)));
        let abs = |args: &Args| Ok(args[0].abs());

        let sqrt = |args: &Args| Ok(args[0].sqrt());
        let cbrt = |args: &Args| Ok(args[0].cbrt());

        let log = |args: &Args| Ok(args[1].log(args[0]));
        let log_10 = |args: &Args| Ok(args[0].log(10.0));
        let ln = |args: &Args| Ok(args[0].ln());

        let floor = |args: &Args| Ok(args[0].floor());
        let ceil = |args: &Args| Ok(args[0].ceil());

        // comparison
        m.insert("min", min as Func);
        m.insert("max", max as Func);
        m.insert("abs", abs as Func);
        m.insert("sqrt", sqrt as Func);
        m.insert("cbrt", cbrt as Func);

        // trig
        m.insert("sin", sin as Func);
        m.insert("cos", cos as Func);
        m.insert("tan", tan as Func);

        m.insert("log", log as Func);
        m.insert("log_10", log_10 as Func);
        m.insert("ln", ln as Func);

        m.insert("floor", floor as Func);
        m.insert("ceil", ceil as Func);

        m
    };
}

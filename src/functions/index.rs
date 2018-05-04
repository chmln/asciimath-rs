use ast::{EvaluationResult, NumericLiteral};
use std::collections::HashMap;

pub type Args = Vec<NumericLiteral>;
pub type Func = fn(&Args) -> EvaluationResult;
pub type CustomFn = Func;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, Func> = {
        let mut m = HashMap::with_capacity(10);


        let sin  = |args: &Args| Ok(args[0].to_radians().sin());
        let cos  = |args: &Args| Ok(args[0].to_radians().cos());
        let tan  = |args: &Args| Ok(args[0].to_radians().tan());

        let max =  |args: &Args| Ok(args.iter().fold(0. / 0., |acc: NumericLiteral, x| acc.max(*x)));
        let min  =  |args: &Args| Ok(args.iter().fold(0. / 0., |acc: NumericLiteral, x| acc.min(*x)));
        let abs  = |args: &Args| Ok(args[0].abs());

        let sqrt = |args: &Args| Ok(args[0].sqrt());
        let cbrt  = |args: &Args| Ok(args[0].cbrt());

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

        m
    };
}

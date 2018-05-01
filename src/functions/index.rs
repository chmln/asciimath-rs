use ast::EvaluationResult;
use std::collections::HashMap;

type Func = fn(&Vec<f64>) -> EvaluationResult;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<String, Func> = {
        let mut m = HashMap::new();
        m.insert("max".to_string(), max as Func);

        m
    };
}

fn max(args: &Vec<f64>) -> EvaluationResult {
    // if let (Some(val1),Some(val2)) = (args.get(0), args.get(1)) {
    //   Ok(val1.max(*val2))
    // }
    // else {
    //   Err("max() - not enough args")
    // }

    Ok(args.iter().fold(0. / 0., |acc, x| acc.max(*x)))
}

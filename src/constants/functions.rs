use crate::ast::{EvaluationResult, NumericLiteral, Value};
use once_cell::sync::Lazy;
use smallvec::SmallVec;
use std::{collections::HashMap, f64};

#[derive(Default)]
pub struct Args(SmallVec<[Value; 4]>);

impl From<Vec<Value>> for Args {
    fn from(args: Vec<Value>) -> Self {
        Args(args.into())
    }
}

impl Args {
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }

    pub fn nums(
        self,
        n: usize,
    ) -> Result<impl Iterator<Item = NumericLiteral>, Error> {
        let nums: SmallVec<[NumericLiteral; 4]> = self
            .0
            .into_iter()
            .filter_map(|x| match x {
                Value::Num(x) => Some(x),
                _ => None,
            })
            .collect();

        if nums.len() < n {
            Err(Error::NotEnoughParams)
        } else {
            Ok(nums.into_iter())
        }
    }
}

pub type Func = fn(Args) -> EvaluationResult;
pub type CustomFn = Func;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid argument given supplied: {}", .0)]
    InvalidArgument(String),
    #[error("expected more parameters for function")]
    NotEnoughParams,
}

pub static FUNCTIONS: Lazy<HashMap<&'static str, Func>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(15);

    let sin = |args: Args| {
        Ok(args.nums(1)?.next().unwrap().to_radians().sin().into())
    };
    let cos = |args: Args| {
        Ok(args.nums(1)?.next().unwrap().to_radians().cos().into())
    };
    let tan = |args: Args| {
        Ok(args.nums(1)?.next().unwrap().to_radians().tan().into())
    };

    let max = |args: Args| {
        Ok(args
            .iter()
            .filter_map(|x| match x {
                Value::Num(x) => Some(x),
                _ => None,
            })
            .fold(f64::NAN, |acc: NumericLiteral, x| acc.max(*x))
            .into())
    };
    let min = |args: Args| {
        Ok(args
            .iter()
            .filter_map(|x| match x {
                Value::Num(x) => Some(x),
                _ => None,
            })
            .fold(f64::NAN, |acc: NumericLiteral, x| acc.min(*x))
            .into())
    };
    let abs = |args: Args| Ok(args.nums(1)?.next().unwrap().abs().into());
    let sqrt = |args: Args| Ok(args.nums(1)?.next().unwrap().sqrt().into());
    let cbrt = |args: Args| Ok(args.nums(1)?.next().unwrap().cbrt().into());
    let floor = |args: Args| Ok(args.nums(1)?.next().unwrap().floor().into());
    let ln = |args: Args| Ok(args.nums(1)?.next().unwrap().ln().into());
    let ceil = |args: Args| Ok(args.nums(1)?.next().unwrap().ceil().into());

    let log_10 = |args: Args| Ok(args.nums(1)?.next().unwrap().log10().into());
    let log = |args: Args| {
        let args = args.nums(2)?.collect::<Vec<_>>();
        Ok(args.get(1).unwrap().log(*args.get(0).unwrap()).into())
    };

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
});

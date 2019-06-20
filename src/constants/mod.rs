mod functions;
pub use self::functions::{Args, CustomFn, Func, FUNCTIONS};

use crate::ast::NumericLiteral;
use lazy_static::lazy_static;
use std::{collections::HashMap, f64};

lazy_static! {
    pub static ref CONSTANTS: HashMap<&'static str, NumericLiteral> = {
        let mut m = HashMap::with_capacity(5);

        // comparison
        m.insert("PI", f64::consts::PI);
        m.insert("E", f64::consts::E);
        m.insert("INFINITY", f64::INFINITY);
        m.insert("NEG_INFINITY", f64::NEG_INFINITY);

        m
    };
}

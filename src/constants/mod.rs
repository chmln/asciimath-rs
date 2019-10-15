mod functions;
pub use self::functions::{Args, CustomFn, Func, FUNCTIONS};

use crate::ast::NumericLiteral;
use once_cell::sync::Lazy;
use std::{collections::HashMap, f64};

pub static CONSTANTS: Lazy<HashMap<&'static str, NumericLiteral>> =
    Lazy::new(|| {
        let mut m = HashMap::with_capacity(5);

        // comparison
        m.insert("PI", f64::consts::PI);
        m.insert("E", f64::consts::E);
        m.insert("INFINITY", f64::INFINITY);
        m.insert("NEG_INFINITY", f64::NEG_INFINITY);

        m
    });

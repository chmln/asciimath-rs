//! A mathematical expression parsing and evaluation library.
//!
//! # Evaluating an expression with variables
//!
//! `eval` is perfect for cases when you just need to evaluate an expression
//! once, with a given a set of variables.
//!
//! ```
//! use asciimath::{eval, scope};
//!
//! let expression = "(x + y * 4) ^ 3";
//! let variables = scope! {
//!    "x" => 8,
//!    "y" => 12.25
//! };
//!
//! assert_eq!(Ok(185193.0), eval(expression, &variables));
//! ```
//!
//!
//! # Compiling Expressions for Repeated Evaluation
//!
//! The example below demonstrates parsing and evaluation of an expression
//! with two sets of variables.
//!
//! The Scope is passed to the compiler for disambiguation in cases of
//! implicit multiplication and function calls.
//!
//! ```
//! use asciimath::{compile, scope, Evaluate};
//!
//! let scope_one = scope! {
//!    "x" => 8,
//!    "y" => 12.25
//! };
//! let scope_two = scope! {
//!    "x" => 3,
//!    "y" => 0
//! };
//! let expression = compile("(x + y * 4) ^ 3", &scope_one).unwrap();
//!
//! assert_eq!(Ok(185193.0), expression.eval_with(&scope_one));
//! assert_eq!(Ok(27.0), expression.eval_with(&scope_two));
//! ```
//!
//! # Custom Functions
//!
//! A lot of effort has been put into making custom functions as easy to write
//! as possible.
//!
//! ```
//! use asciimath::{eval, scope, CustomFn};
//!
//! let my_sum: CustomFn = |args| Ok(args.iter().sum());
//!
//! let scope = scope! {
//!   "x" => 1,
//!   "my_sum" => my_sum,
//! };
//!
//! assert_eq!(Ok(6.0), eval("my_sum(x, 2, 3)", &scope));
//! ```
//!
//! # Builtins
//!
//! Functions:
//! - `cos(x)`
//! - `tan(x)`
//! - `max(a,b,c,...)`
//! - `min(a,b,c,...)`
//! - `abs(x)`
//! - `sqrt(x)`
//! - `cbrt(x)`
//! - `log(base, x)`
//! - `log_10(x)`
//! - `ln(x)`
//! - `floor(x)`
//! - `ceil(x)`
//!
//! Constants:
//! - PI
//! - E (Euler's number)
//! - INFINITY
//! - NEG_INFINITY

mod ast;
pub(crate) mod constants;
mod error;
mod lexer;
#[macro_use]
mod macros;
mod parser;
mod tokens;
mod util;

pub use crate::{
    ast::{Evaluate, Scope},
    constants::CustomFn,
    error::Error,
    parser::{compile, eval},
};

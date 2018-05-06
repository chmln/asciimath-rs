//! A mathematical expression parsing and evaluation library.
//!
//! # Evaluating an expression with variables
//!
//! `eval` is perfect for cases when you just need to evaluate an expression
//! once, with a given a set of variables.
//!
//! ```
//! #[macro_use] extern crate asciimath;
//! use asciimath::{eval,Evaluate};
//!
//! assert_eq!(Ok(185193.0), eval("(x + y * 4) ^ 3", &scope!{
//!    "x" => 8,
//!    "y" => 12.25
//! }));
//! ```
//!
//! For repeated evaluation of an expression, see the next example.
//!
//! # Compiling Expressions
//!
//! The example below demonstrates parsing and evaluation of an expression
//! with two sets of variables.
//!
//! The Scope is passed to the compiler only for disambiguation in cases of
//! implicit multiplication and function calls.
//!
//! ```
//! #[macro_use] extern crate asciimath;
//! use asciimath::{compile,Evaluate};
//!
//! let scope_one = scope!{
//!    "x" => 8,
//!    "y" => 12.25
//! };
//! let scope_two = scope!{
//!    "x" => 3,
//!    "y" => 0
//! };
//! let expression = compile("(x + y * 4) ^ 3", &scope_one).unwrap();
//!
//! assert_eq!(Ok(185193.0), expression.eval_with(&scope_one));
//! //assert_eq!(Ok(27.0), expression.eval_with(&scope_two));
//! ```
//!
//! # Custom Functions
//!
//! ```
//! #[macro_use] extern crate asciimath;
//! use asciimath::{eval,Evaluate,CustomFn};
//!
//! let my_sum: CustomFn = |args| Ok(args.iter().sum());
//!
//! let scope = scope!{
//! "x" => 1,
//! "my_sum" => my_sum,
//! };
//!
//! assert_eq!(Ok(6.0), eval("my_sum(x, 2, 3)",&scope));
//! ```
//!
//!

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod ast;
mod functions;
mod lexer;
mod parser;
mod tokens;

pub use ast::{Evaluate, Scope};
pub use functions::CustomFn;
pub use parser::{compile, eval};

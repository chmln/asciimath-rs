//! A mathematical expression parsing and evaluation library.
//!
//! # Get Started
//!
//! The example below demonstrates parsing and evaluation of an expression
//! with user-defined variables. The compiled expression is immutable and can be evaluated with many scopes.
//!
//! ```
//! extern crate asciimath;
//! use asciimath::{parse,Scope,Evaluate};
//!
//! let parsed_expr = parse("(x + y * 4) ^ 3").unwrap();
//! let mut scope = Scope::new();
//! scope.set_var("x", 8);
//! scope.set_var("y", 12.25);
//!
//! assert_eq!(Ok(185193.0), parsed_expr.eval_with(&scope));
//! ```
//!
//! # Custom Functions
//!
//! ```
//! extern crate asciimath;
//! use asciimath::{parse,Scope,Evaluate,CustomFn};
//!
//! let parsed_expr = parse("my_sum(x, 2, 3)").unwrap();
//! let my_sum: CustomFn = |args| Ok(args.iter().sum());
//!
//! let mut scope = Scope::new();
//! scope.set_var("x", 1);
//! scope.set_var("my_sum", my_sum);
//!
//! assert_eq!(Ok(6.0), parsed_expr.eval_with(&scope));
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

pub use ast::{Evaluate, Node, Scope};
pub use functions::CustomFn;
pub use parser::parse;

//! A mathematical expression parsing and evaluation library.
//!
//! # Typical Use
//!
//! ```
//! extern crate asciimath;
//! use asciimath::{parse,Scope,Evaluate};
//!
//! let parsed_expr = parse("(x+y*4)^3").unwrap();
//! let mut scope = Scope::new();
//! scope.set_var("x", 8);
//! scope.set_var("y", 12.25);
//!
//! assert_eq!(Ok(185193.0), parsed_expr.eval_with(&scope));
//! ```
#[macro_use]
mod logger;
mod ast;
mod lexer;
mod parser;
mod tokens;

pub const DEBUG: bool = true;

pub use ast::{Evaluate, Node, Scope};
pub use parser::parse;

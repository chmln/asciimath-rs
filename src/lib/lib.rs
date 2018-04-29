mod lexer;
mod parser;
mod tokens;

pub use parser::{parse, Evaluate, Scope};

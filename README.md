# asciimath-rs

Parses mathematical (infix) expressions into an Abstract Syntax Tree using  Dijkstra's "shunting yard" algorithm. 

Simple, bare-bones, and efficient.

## Features
- evaluation
- support for variables 
  - with artibrary number of characters (e.g. abcde)

## Goals
- [ ] functions
  - [ ] basic (sin, cos, etc)
  - [ ] user-defined
- [ ] matrices (maybe)
- [ ] documentation

## Non-goals

- non-mathematical expressions
- php-esque abominations like `a=func(x) && b=func(y) && 'what'`

## How-To

```rust
extern crate asciimath;
use asciimath::Evaluate;

fn main() {
    let expression = "x + 4 * 2 / ( 1 - 5 ) ^ (2 - 2 ^ 3)";

    let mut scope = asciimath::Scope::new();
    scope.set_var("x", 3);

    println!(
        "{:?}",
        asciimath::parse(expression).eval_with(&scope)
    );
}
```

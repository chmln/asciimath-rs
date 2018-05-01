
[![Crates.io](https://img.shields.io/crates/v/asciimath.svg)](https://crates.io/crates/asciimath)
[![](https://docs.rs/asciimath/badge.svg)](https://docs.rs/asciimath)


# asciimath-rs

Parses mathematical (infix) expressions into an Abstract Syntax Tree using  Dijkstra's "shunting yard" algorithm. 

Simple, bare-bones, and efficient.

## Features
- evaluation
- support for variables 
  - with artibrary number of characters (e.g. abcde)

## Goals
- [x] functions
  - [x] basic (sin, cos, etc)
  - [ ] user-defined
- [ ] matrices (maybe)
- [x] documentation

## Non-goals

- non-mathematical expressions
- php-esque abominations like `a=func(x) && b=func(y) && 'what'`


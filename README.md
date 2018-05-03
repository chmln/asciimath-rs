
[![Crates.io](https://img.shields.io/crates/v/asciimath.svg)](https://crates.io/crates/asciimath)
[![](https://docs.rs/asciimath/badge.svg)](https://docs.rs/asciimath)
[![CI](https://circleci.com/gh/chmln/asciimath-rs.svg?style=svg)](https://circleci.com/gh/chmln/asciimath-rs)


# asciimath-rs

Parses mathematical (infix) expressions into an Abstract Syntax Tree using  Dijkstra's "shunting yard" algorithm. 

Simple, bare-bones, and efficient.

## Roadmap
- [x] evaluation
- [x] support for variables 
  - [ ] support for word-variables (right now only single-letter variables are supported)
- [x] functions
  - [x] basic (sin, cos, etc)
  - [ ] user-defined
- [ ] error handling and propogation
- [ ] matrices (maybe)
- [x] documentation

## Goals


## Non-goals

- non-mathematical expressions, at least for now
- php-esque abominations like `a=func(x) && b=func(y) && 'what'`


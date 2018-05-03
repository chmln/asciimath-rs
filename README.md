
[![Crates.io](https://img.shields.io/crates/v/asciimath.svg)](https://crates.io/crates/asciimath)
[![](https://docs.rs/asciimath/badge.svg)](https://docs.rs/asciimath)
[![CI](https://circleci.com/gh/chmln/asciimath-rs.svg?style=svg)](https://circleci.com/gh/chmln/asciimath-rs)


# asciimath-rs

Parses mathematical (infix) expressions into an Abstract Syntax Tree using  Dijkstra's "shunting yard" algorithm. 

Simple, bare-bones, and efficient.

## Features

- support for variables 
- evaluation
  - compile the expression just once, evaluate with a different set of variables
- implicit multiplication


## Roadmap

- [ ] Error handling and propogation 
- [ ] Embedded constants like `pi` and `e`
- [x] Functions
  - [x] basic (sin, cos, etc)
  - [ ] user-defined
- [ ] Support for word-variables (right now only single-letter variables are supported)
- [ ] Thorough documentation
  - [x] basic use
  - [ ] available functions

## Future Goals

The items below will be considered after ABI stabilization:

- Non-mathematical expressions, like strings
- More operators (e.g. ternary ? : )
- Ability to simplify expressions
- Derivatives, incl. second-order and third-order
- Integration
- Partial differentiation
- Vector calculus
- Matrices and vector spaces

## Non-goals

- php-esque abominations like `a=func(x) && b=func(y) && 'what'`

## Motivation

Created for the University of Toronto Mathematical Assessment Tool, to facilitate generation of randomized tests and quizzes, which are then automatically graded. 


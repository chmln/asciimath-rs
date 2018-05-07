
[![Crates.io](https://img.shields.io/crates/v/asciimath.svg)](https://crates.io/crates/asciimath)
[![](https://docs.rs/asciimath/badge.svg)](https://docs.rs/asciimath)
[![CI](https://circleci.com/gh/chmln/asciimath-rs.svg?style=svg)](https://circleci.com/gh/chmln/asciimath-rs)

# asciimath-rs

Parses mathematical (infix) expressions into an Abstract Syntax Tree using  Dijkstra's "shunting yard" algorithm.

Simple, bare-bones, and efficient.

## Features

- evaluation
- implicit multiplication
- support for variables, both single-letter and word variables
- easily-defined custom functions
- compiling expressions and evaluating with different sets of variables
- f64 output
- Baked-in essential functions and constants

## High-Level Goals

**Ease of use**

This means that e.g. passing in variables to expressions and defining custom functions must be possible with minimum knowledge of this library's internals and abstractions. Errors must be helpful and relevant.

**Minimalism**

Focusing just on mathematical expressions will make it easy for this library to remain slim and deliver superior ergonomics.

**Accuracy**

Extensive testing and maximum precision must be a part of all the modules to prevent bugs and ensure consistency.

## Future

The items below will be considered after ABI stabilization:

- Non-mathematical expressions, like strings
- More operators (e.g. ternary ? : )
- Ability to simplify expressions
- Derivatives, incl. second-order and third-order
- Integration
- Partial differentiation
- Vector calculus
- Matrices and vector spaces

## Motivation

While some great libraries aiming for similar goals do exist, they wouldn't reward me with such a fruitful Rust learning experience and imo sorely lack ergonomics.


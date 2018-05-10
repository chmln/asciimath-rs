# Changelog

## 0.8.3

### Internals

- Turned one-field structs into enum fields, shedding ~80 loc
- rustfmt tweaks

## 0.8.2

Remove unused imports.

## 0.8.1

### Internals

Lexer:
- Rewritten with peekable iterators
- Simplifed further
- Split tests into a separate file


## 0.8.0

### Added

Seven new comparison operators:
  - `>`
  - `<`
  - `!`
  - `==`
  - `!=`
  - `>=`
  - `<=`

### Internals

- Lexer - simplify parsing of operators with equality

## 0.7.1

- Removed unused import

## 0.7.0

- New built-in functions
  - `log(base, x)`
  - `log_10(x)`
  - `ln(x)`
  - `floor(x)`
  - `ceil(x)`
- Implemented new constants
  - `PI`
  - `E` (Euler's number)
  - `INFINITY`
  - `NEG_INFINITY`
- Support for word variables
  - e.g. a var named `"quantity"`

## 0.6.0

- Breaking: parsing now requires scope for disambiguation
- Added `scope!` macro for easier variable setup
- Implemented Error types and messages
- Fixed errors when evaluating expr with multiple scopes
- Cleaned up macros

## 0.5.1

- Nuked `unwrap()` across codebase
- Improved error handling

## 0.5.0

- Added support for custom functions
- Set up CI
- Fixed implicit multiplication edge-cases

## 0.4.0

- Implemented unary negative number support, like `-x`
- Added eight builtin functions


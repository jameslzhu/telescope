# Telescope

[![Build Status][travis-badge]][travis-link]
[![Appveyor Status][appveyor-badge]][appveyor-link]
[![Coverage Status][coveralls-badge]][coveralls-link]

[![XKCD][xkcd-img]][xkcd-link]

[travis-badge]: https://img.shields.io/travis/jzhu98/telescope.svg?style=flat-square
[travis-link]: https://travis-ci.org/jzhu98/telescope
[appveyor-badge]:https://img.shields.io/appveyor/ci/jzhu98/lrs.svg?style=flat-square
[appveyor-link]: https://ci.appveyor.com/project/jzhu98/lrs
[coveralls-badge]: https://img.shields.io/coveralls/jzhu98/telescope.svg?style=flat-square
[coveralls-link]: https://coveralls.io/github/jzhu98/telescope?branch=develop
[xkcd-img]: https://imgs.xkcd.com/comics/lisp_cycles.png
[xkcd-link]: https://xkcd.com/297/

Inspired by [Build Your Own Lisp](https://buildyourownlisp.com) and
[Crafting Interpreters](https://craftinginterpreters.com), Telescope is a Lisp-family
language, which I built to better understand the world of
programming languages.

## Specification

### Interpreter

#### Source
  - Read from stdin (assumed UTF-8)

#### Lexical analysis
  - Divide source into token sequence
  - Replace special tokens with other tokens

#### Syntax analysis
  - Parse token sequence into syntax tree

#### Semantic analysis
  - Traverse syntax trees
  - Declare variables
  - Load symbol tables
  - Assign types
  - Determine program meaning

#### Code execution
  - Code is evaluated (as everything is an expression) and outputted to stdout

### Syntax

#### Expressions

As a functional language, everything is an expression, which may be an *atom*, *sexpr*, or *qexpr*.

#### Data Types

- ()    (nil)
- int   (i32)
- flt   (f32)
- bool  (boolean)
- str   (string)
- fn    (function)

An expression appears as follows:

```lisp
(operator operands ...)
```

A list literal is denoted by `[]`:

```scheme
> [1 2 3]
(1 2 3)
```

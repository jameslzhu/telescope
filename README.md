# Telescope

[![Build
Status](https://travis-ci.org/jzhu98/telescope.svg?style=flat-square&branch=develop)](https://travis-ci.org/jzhu98/telescope)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/rlhd2gyjcmdkdxc7/branch/develop?svg=true)](https://ci.appveyor.com/project/jzhu98/telescope/branch/develop)
[![Coverage Status](https://coveralls.io/repos/github/jzhu98/telescope/badge.svg?style=flat-square&branch=develop)](https://coveralls.io/github/jzhu98/telescope?branch=develop)

![XKCD](https://imgs.xkcd.com/comics/lisp_cycles.png)

A minimal, extensible language.

Inspired by [Build Your Own Lisp](https://buildyourownlisp.com) and
[Crafting Interpreters](https://craftinginterpreters.com), Telescope is a Lisp-family
language, which I built to better understand the world of
programming languages.

## Specification

### Interpreter

1. Source
  - Read from stdin (assumed UTF-8)

2. Lexical analysis
  - Divide source into token sequence
  - Replace special tokens with other tokens

3. Syntax analysis
  - Parse token sequence into syntax tree

4. Semantic analysis
  - Traverse syntax trees
  - Declare variables
  - Load symbol tables
  - Assign types
  - Determine program meaning

5. Code execution
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

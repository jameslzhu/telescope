# Telescope

[![Build
Status](https://travis-ci.org/jzhu98/lrs.svg?style=flat-square&branch=develop)](https://travis-ci.org/jzhu98/lrs)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/rlhd2gyjcmdkdxc7/branch/develop?svg=true)](https://ci.appveyor.com/project/jzhu98/lrs/branch/develop)
[![Coverage Status](https://coveralls.io/repos/github/jzhu98/lrs/badge.svg?style=flat-square&branch=develop)](https://coveralls.io/github/jzhu98/lrs?branch=develop)
[![Clippy Linting Result](https://clippy.bashy.io/github/jzhu98/lrs/develop/badge.svg?style=flat-square)](https://clippy.bashy.io/github/jzhu98/lrs/develop/log)

**Telescope** is a Lisp interpreter, based on [Build Your Own Lisp](buildyourownlisp.com).
It attempts to be minimal, well-defined, performant, and extensible.

Most notably, Telescope lists are not backed by singly-linked lists, but by vectors.

## Design

### Syntax

#### Expressions

As a functional language, everything is an expression that
evaluates to either a value, which is either  are referred to as an *atom*, or a *list*.

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

### Data layout
There are five major data structures in Telescope.


lrs
-------
[![Build
Status](https://travis-ci.org/jzhu98/lrs.svg?style=flat-square&branch=develop)](https://travis-ci.org/jzhu98/lrs)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/rlhd2gyjcmdkdxc7/branch/develop?svg=true)](https://ci.appveyor.com/project/jzhu98/lrs/branch/develop)
[![Coverage Status](https://coveralls.io/repos/github/jzhu98/lrs/badge.svg?style=flat-square&branch=develop)](https://coveralls.io/github/jzhu98/lrs?branch=develop)
[![Clippy Linting Result](https://clippy.bashy.io/github/jzhu98/lrs/develop/badge.svg?style=flat-square)](https://clippy.bashy.io/github/jzhu98/lrs/develop/log)

Lisp interpreter, built with Rust and based on [Build Your Own Lisp](buildyourownlisp.com).

## Design

### Expressions
As a functional language, everything is an expression that
evaluates to either a value, which is either  are referred to as an *atom*, or a *list*.

- ()    (unit)
- int   (i64)
- flt   (f64)
- bool  (boolean)
- str   (string)
- fn    (function)

An expression appears as follows:
```
(operator operands ...)
```

A list literal is denoted by a '(), as in Scheme:
'(elements ...)

### Data layout

lrs
-------
[![Build
Status](https://travis-ci.org/jzhu98/lisp-rs.svg?style=flat-square&branch=develop)](https://travis-ci.org/jzhu98/lisp-rs)
[![Coverage Status](https://coveralls.io/repos/github/jzhu98/lisp-rs/badge.svg?style=flat-square&branch=develop)](https://coveralls.io/github/jzhu98/lisp-rs?branch=feature%2Fatom)
[![Clippy Linting Result](https://clippy.bashy.io/github/jzhu98/lisp-rs/develop/badge.svg?style=flat-square)](https://clippy.bashy.io/github/jzhu98/lisp-rs/develop/log)

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

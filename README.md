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
[Crafting Interpreters](https://craftinginterpreters.com), **Telescope** is a
Lisp-family language, which I built to better understand the world of programming
languages (and learn Rust).

Because the world needs more civilized languages.

**Telescope** is modelled after the simplicity of Scheme, with the sensibility
and syntax of Clojure. Vectors (resizeable arrays) are a first-class object,
and maps (hashmaps) are planned for the future.

(At the moment, lists are implemented as vectors for simplicity and fewer
borrow-checker headaches, but I'll change them to singly-linked lists later.)

## Building

This project requires a recent version of stable Rust (around 1.12+). It's been
tested and developed on Windows and Arch Linux.

Compile and run with Cargo, the world's greatest build tool ever (seriously why
can't every build tool be this easy to use):

```sh
$ cargo run
```

There's even tests! (With a rock-solid 33% code coverage.)

```sh
$ cargo test
```

## Contributing

This is a private project. It's mine to goof up, break, and learn from. I
hope to design and implement all major features of this language myself,
because I learn best with practical experience.

That said, if I'm actually so far off the mark that I'm hitting the wall instead of the dartboard, feel free to fork and pull request minor changes. I will appreciate any constructive criticism you have.

## Syntax

### Data Types

Telescope comes with 7 primitive data types (maps WIP):

repr       | type
---------- | ----
`()`       | nil
`int`      | i64
`flt`      | f64
`#t`, `#f` | boolean
`str`      | string
`fn`       | function
`(...)`    | list
`[...]`    | vector

The first six (nil, int, flt, bool, str, fn) are considered
atoms. Lists and vectors are collections.

A function call looks like this, in prefix notation:

```clj
(operator operands ...)
```

So adding numbers looks like:

```clj
(+ 1 2 3 4 5 6 7 8 9 10)
=> 55
```

A vector literal is denoted by `[]`:

```clj
> [1 2 3]
```

### Built-in Functions

(See `src/ops.rs` for the implementation.)

#### Mathematical Operators

The usual suspects: arithmetic and comparison.

```
+ - * / = < <= > >=
```

#### Logical Operators

(At some point I should turn `and` and `or` into special forms, for short
circuiting. I am *not* simply lazy.)

```clj
(not #f)
#t
```

```clj
(and (= 0 0) (= 1 0))
=> #f
```

```clj
(or (= 0 0) (= 1 0))
=> #t
```

#### List Operations

Because it wouldn't be a Lisp without them.

```clj
(first [1 2 3])
=> 1
```

```clj
(rest [1 2 3])
=> [2 3]
```

```clj
(cons 0 1)
=> (0 1)
```

### Special Forms

(See `src/forms.rs` for the implementation.)

#### `(def symbol init)`

Binds `symbol` in the current scope to the (evaluated)
value of `init`.

#### `(if cond then else?)`

Checks if `cond` is truthy (i.e. not `nil` or `false`). If so, executes the
`then` clause. Otherwise the `else?` clause is executed (if present).

#### `(let [bindings*] exprs*)`

*(Not yet implemented.)*

Creates a new scope with the stated `bindings` before
executing the `exprs`.

#### `(do exprs*)`

Executes `exprs` in order, returning the last value.

#### `(quote form)`

Returns the un-evaluated `form`.

#### `(fn name? [params* ] exprs*)`

Defines a named (or anonymous) function.

#### `(defmacro name [params*] body)`

Defines a macro, which performs text substitution. Pretty much how the entire
standard library of Telescope gets defined.

(Only a faint idea how to do this at the moment.)
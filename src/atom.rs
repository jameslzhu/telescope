use std::str::FromStr;
use token::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Atom {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use token::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtExpr_2a(::std::vec::Vec<Expr>),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtNum(Atom),
        NtOp(Atom),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Num(Atom),
        Nt____Op(Atom),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        5, // on r#"(?:\\+|-|\\*|/)"#, goto 4
        6, // on r#"[0-9]+"#, goto 5
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -12, // on EOF, reduce `__Atom = Atom => ActionFn(1);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(7);`
        -1, // on EOF, reduce `Atom = Op => ActionFn(6);`
        -11, // on EOF, reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        -10, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        3, // on Num, goto 2
        4, // on Op, goto 3
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
    ];
    pub fn parse_Atom<
        'input,
    >(
        input: &'input str,
    ) -> Result<Atom, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Atom,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Op => ActionFn(6);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(7);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", ")" => ActionFn(14);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Expr+, ")" => ActionFn(15);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Expr = Atom => ActionFn(5);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            6 => {
                // Expr* =  => ActionFn(10);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action10(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            7 => {
                // Expr* = Expr+ => ActionFn(11);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            8 => {
                // Expr+ = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            9 => {
                // Expr+ = Expr+, Expr => ActionFn(13);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            10 => {
                // Num = r#"[0-9]+"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            11 => {
                // Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            12 => {
                // __Atom = Atom => ActionFn(1);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            13 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                7
            }
            14 => {
                // __Num = Num => ActionFn(3);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                8
            }
            15 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                9
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Atom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Atom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Atom::parse_Atom;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use token::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtExpr_2a(::std::vec::Vec<Expr>),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtNum(Atom),
        NtOp(Atom),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Num(Atom),
        Nt____Op(Atom),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, // on "(", goto 5
        0, // on ")", error
        7, // on r#"(?:\\+|-|\\*|/)"#, goto 6
        8, // on r#"[0-9]+"#, goto 7
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 5
        14, // on "(", goto 13
        15, // on ")", goto 14
        16, // on r#"(?:\\+|-|\\*|/)"#, goto 15
        17, // on r#"[0-9]+"#, goto 16
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 8
        -5, // on "(", reduce `Expr = Atom => ActionFn(5);`
        -5, // on ")", reduce `Expr = Atom => ActionFn(5);`
        -5, // on r#"(?:\\+|-|\\*|/)"#, reduce `Expr = Atom => ActionFn(5);`
        -5, // on r#"[0-9]+"#, reduce `Expr = Atom => ActionFn(5);`
        // State 9
        -8, // on "(", reduce `Expr+ = Expr => ActionFn(12);`
        -8, // on ")", reduce `Expr+ = Expr => ActionFn(12);`
        -8, // on r#"(?:\\+|-|\\*|/)"#, reduce `Expr+ = Expr => ActionFn(12);`
        -8, // on r#"[0-9]+"#, reduce `Expr+ = Expr => ActionFn(12);`
        // State 10
        14, // on "(", goto 13
        19, // on ")", goto 18
        16, // on r#"(?:\\+|-|\\*|/)"#, goto 15
        17, // on r#"[0-9]+"#, goto 16
        // State 11
        -2, // on "(", reduce `Atom = Num => ActionFn(7);`
        -2, // on ")", reduce `Atom = Num => ActionFn(7);`
        -2, // on r#"(?:\\+|-|\\*|/)"#, reduce `Atom = Num => ActionFn(7);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(7);`
        // State 12
        -1, // on "(", reduce `Atom = Op => ActionFn(6);`
        -1, // on ")", reduce `Atom = Op => ActionFn(6);`
        -1, // on r#"(?:\\+|-|\\*|/)"#, reduce `Atom = Op => ActionFn(6);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Op => ActionFn(6);`
        // State 13
        14, // on "(", goto 13
        21, // on ")", goto 20
        16, // on r#"(?:\\+|-|\\*|/)"#, goto 15
        17, // on r#"[0-9]+"#, goto 16
        // State 14
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 15
        -11, // on "(", reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        -11, // on ")", reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        -11, // on r#"(?:\\+|-|\\*|/)"#, reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        -11, // on r#"[0-9]+"#, reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        // State 16
        -10, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
        -10, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
        -10, // on r#"(?:\\+|-|\\*|/)"#, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
        -10, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
        // State 17
        -9, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(13);`
        -9, // on ")", reduce `Expr+ = Expr+, Expr => ActionFn(13);`
        -9, // on r#"(?:\\+|-|\\*|/)"#, reduce `Expr+ = Expr+, Expr => ActionFn(13);`
        -9, // on r#"[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(13);`
        // State 18
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 19
        14, // on "(", goto 13
        22, // on ")", goto 21
        16, // on r#"(?:\\+|-|\\*|/)"#, goto 15
        17, // on r#"[0-9]+"#, goto 16
        // State 20
        -3, // on "(", reduce `Expr = "(", ")" => ActionFn(14);`
        -3, // on ")", reduce `Expr = "(", ")" => ActionFn(14);`
        -3, // on r#"(?:\\+|-|\\*|/)"#, reduce `Expr = "(", ")" => ActionFn(14);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", ")" => ActionFn(14);`
        // State 21
        -4, // on "(", reduce `Expr = "(", Expr+, ")" => ActionFn(15);`
        -4, // on ")", reduce `Expr = "(", Expr+, ")" => ActionFn(15);`
        -4, // on r#"(?:\\+|-|\\*|/)"#, reduce `Expr = "(", Expr+, ")" => ActionFn(15);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Expr+, ")" => ActionFn(15);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -5, // on EOF, reduce `Expr = Atom => ActionFn(5);`
        -13, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(7);`
        -1, // on EOF, reduce `Atom = Op => ActionFn(6);`
        0, // on EOF, error
        -11, // on EOF, reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
        -10, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = "(", ")" => ActionFn(14);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Expr+, ")" => ActionFn(15);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        3, // on Expr, goto 2
        0, // on Expr*, error
        0, // on Expr+, error
        4, // on Num, goto 3
        5, // on Op, goto 4
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 5
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Expr*, error
        11, // on Expr+, goto 10
        12, // on Num, goto 11
        13, // on Op, goto 12
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 10
        9, // on Atom, goto 8
        18, // on Expr, goto 17
        0, // on Expr*, error
        0, // on Expr+, error
        12, // on Num, goto 11
        13, // on Op, goto 12
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 12
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 13
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Expr*, error
        20, // on Expr+, goto 19
        12, // on Num, goto 11
        13, // on Op, goto 12
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 15
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 19
        9, // on Atom, goto 8
        18, // on Expr, goto 17
        0, // on Expr*, error
        0, // on Expr+, error
        12, // on Num, goto 11
        13, // on Op, goto 12
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 21
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
    ];
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expr, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Op => ActionFn(6);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(7);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", ")" => ActionFn(14);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Expr+, ")" => ActionFn(15);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Expr = Atom => ActionFn(5);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            6 => {
                // Expr* =  => ActionFn(10);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action10(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            7 => {
                // Expr* = Expr+ => ActionFn(11);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            8 => {
                // Expr+ = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            9 => {
                // Expr+ = Expr+, Expr => ActionFn(13);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            10 => {
                // Num = r#"[0-9]+"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            11 => {
                // Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            12 => {
                // __Atom = Atom => ActionFn(1);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                6
            }
            13 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            14 => {
                // __Num = Num => ActionFn(3);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                8
            }
            15 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                9
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Atom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Atom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

mod __parse__Num {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use token::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtExpr_2a(::std::vec::Vec<Expr>),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtNum(Atom),
        NtOp(Atom),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Num(Atom),
        Nt____Op(Atom),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        3, // on r#"[0-9]+"#, goto 2
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -14, // on EOF, reduce `__Num = Num => ActionFn(3);`
        -10, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        2, // on Num, goto 1
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
    ];
    pub fn parse_Num<
        'input,
    >(
        input: &'input str,
    ) -> Result<Atom, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Atom,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Op => ActionFn(6);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(7);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", ")" => ActionFn(14);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Expr+, ")" => ActionFn(15);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Expr = Atom => ActionFn(5);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            6 => {
                // Expr* =  => ActionFn(10);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action10(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            7 => {
                // Expr* = Expr+ => ActionFn(11);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            8 => {
                // Expr+ = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            9 => {
                // Expr+ = Expr+, Expr => ActionFn(13);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            10 => {
                // Num = r#"[0-9]+"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            11 => {
                // Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            12 => {
                // __Atom = Atom => ActionFn(1);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                6
            }
            13 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                7
            }
            14 => {
                // __Num = Num => ActionFn(3);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            15 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                9
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Atom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Atom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Num::parse_Num;

mod __parse__Op {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use token::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtExpr_2a(::std::vec::Vec<Expr>),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtNum(Atom),
        NtOp(Atom),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Num(Atom),
        Nt____Op(Atom),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        3, // on r#"(?:\\+|-|\\*|/)"#, goto 2
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on r#"(?:\\+|-|\\*|/)"#, error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -15, // on EOF, reduce `__Op = Op => ActionFn(2);`
        -11, // on EOF, reduce `Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        2, // on Op, goto 1
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Expr*, error
        0, // on Expr+, error
        0, // on Num, error
        0, // on Op, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Num, error
        0, // on __Op, error
    ];
    pub fn parse_Op<
        'input,
    >(
        input: &'input str,
    ) -> Result<Atom, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Atom,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Op => ActionFn(6);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(7);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", ")" => ActionFn(14);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Expr+, ")" => ActionFn(15);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Expr = Atom => ActionFn(5);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            6 => {
                // Expr* =  => ActionFn(10);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action10(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            7 => {
                // Expr* = Expr+ => ActionFn(11);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2a(__nt), __end));
                2
            }
            8 => {
                // Expr+ = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            9 => {
                // Expr+ = Expr+, Expr => ActionFn(13);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                3
            }
            10 => {
                // Num = r#"[0-9]+"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            11 => {
                // Op = r#"(?:\\+|-|\\*|/)"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                5
            }
            12 => {
                // __Atom = Atom => ActionFn(1);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                6
            }
            13 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                7
            }
            14 => {
                // __Num = Num => ActionFn(3);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                8
            }
            15 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3f_3a_5c_5c_2b_7c_2d_7c_5c_5c_2a_7c_2f_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Atom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Atom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Atom, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Op::parse_Op;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 4;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    Expr::List(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Expr
{
    Expr::from(__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Atom
{
    Atom::from(Operator::parse(__0).unwrap())
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Atom
{
    Atom::from(i64::from_str(__0).unwrap())
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action10(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action11(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}

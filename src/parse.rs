use std::str::FromStr;
use atom::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Atom {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        9, // on r#"[0-9]+"#, goto 8
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 6
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 7
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 8
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -19, // on EOF, reduce `__Atom = Atom => ActionFn(3);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(12);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(11);`
        -17, // on EOF, reduce `Sym = "*" => ActionFn(15);`
        -15, // on EOF, reduce `Sym = "+" => ActionFn(13);`
        -16, // on EOF, reduce `Sym = "-" => ActionFn(14);`
        -18, // on EOF, reduce `Sym = "/" => ActionFn(16);`
        -14, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        3, // on Num, goto 2
        4, // on Sym, goto 3
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                9
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                10
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                11
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                12
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Atom::parse_Atom;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "\'(", error
        3, // on "(", goto 2
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on r#"[0-9]+"#, error
        // State 3
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        18, // on ")", goto 17
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 4
        -17, // on "\'(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on ")", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "*", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "+", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "-", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "/", reduce `Sym = "*" => ActionFn(15);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(15);`
        // State 5
        -15, // on "\'(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on ")", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "*", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "+", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "-", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "/", reduce `Sym = "+" => ActionFn(13);`
        -15, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(13);`
        // State 6
        -16, // on "\'(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on ")", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "*", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "+", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "-", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "/", reduce `Sym = "-" => ActionFn(14);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(14);`
        // State 7
        -18, // on "\'(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on ")", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "*", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "+", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "-", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "/", reduce `Sym = "/" => ActionFn(16);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(16);`
        // State 8
        -7, // on "\'(", reduce `Node = Atom => ActionFn(8);`
        -7, // on "(", reduce `Node = Atom => ActionFn(8);`
        -7, // on ")", reduce `Node = Atom => ActionFn(8);`
        -7, // on "*", reduce `Node = Atom => ActionFn(8);`
        -7, // on "+", reduce `Node = Atom => ActionFn(8);`
        -7, // on "-", reduce `Node = Atom => ActionFn(8);`
        -7, // on "/", reduce `Node = Atom => ActionFn(8);`
        -7, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(8);`
        // State 9
        -9, // on "\'(", reduce `Node = Expr => ActionFn(10);`
        -9, // on "(", reduce `Node = Expr => ActionFn(10);`
        -9, // on ")", reduce `Node = Expr => ActionFn(10);`
        -9, // on "*", reduce `Node = Expr => ActionFn(10);`
        -9, // on "+", reduce `Node = Expr => ActionFn(10);`
        -9, // on "-", reduce `Node = Expr => ActionFn(10);`
        -9, // on "/", reduce `Node = Expr => ActionFn(10);`
        -9, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(10);`
        // State 10
        -8, // on "\'(", reduce `Node = List => ActionFn(9);`
        -8, // on "(", reduce `Node = List => ActionFn(9);`
        -8, // on ")", reduce `Node = List => ActionFn(9);`
        -8, // on "*", reduce `Node = List => ActionFn(9);`
        -8, // on "+", reduce `Node = List => ActionFn(9);`
        -8, // on "-", reduce `Node = List => ActionFn(9);`
        -8, // on "/", reduce `Node = List => ActionFn(9);`
        -8, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(9);`
        // State 11
        -12, // on "\'(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on ")", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "*", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "+", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "-", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "/", reduce `Node+ = Node => ActionFn(20);`
        -12, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(20);`
        // State 12
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        21, // on ")", goto 20
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 13
        -2, // on "\'(", reduce `Atom = Num => ActionFn(12);`
        -2, // on "(", reduce `Atom = Num => ActionFn(12);`
        -2, // on ")", reduce `Atom = Num => ActionFn(12);`
        -2, // on "*", reduce `Atom = Num => ActionFn(12);`
        -2, // on "+", reduce `Atom = Num => ActionFn(12);`
        -2, // on "-", reduce `Atom = Num => ActionFn(12);`
        -2, // on "/", reduce `Atom = Num => ActionFn(12);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(12);`
        // State 14
        -1, // on "\'(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(11);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(11);`
        // State 15
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        23, // on ")", goto 22
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 16
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on r#"[0-9]+"#, error
        // State 17
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 18
        -14, // on "\'(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        // State 19
        -13, // on "\'(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on ")", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "*", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "+", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "-", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "/", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(21);`
        // State 20
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 21
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        25, // on ")", goto 24
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 22
        -5, // on "\'(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on ")", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "*", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "+", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "-", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "/", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on r#"[0-9]+"#, reduce `List = "\'(", ")" => ActionFn(24);`
        // State 23
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        27, // on ")", goto 26
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 24
        -6, // on "\'(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on ")", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "*", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "+", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "-", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "/", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on r#"[0-9]+"#, reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        // State 25
        16, // on "\'(", goto 15
        17, // on "(", goto 16
        28, // on ")", goto 27
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        19, // on r#"[0-9]+"#, goto 18
        // State 26
        -3, // on "\'(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        // State 27
        -4, // on "\'(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -20, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        2, // on Expr, goto 1
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        4, // on Sym, goto 3
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        13, // on Node+, goto 12
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        22, // on Node+, goto 21
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        24, // on Sym, goto 23
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        26, // on Node+, goto 25
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                8
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                10
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                11
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                12
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

mod __parse__List {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, // on "\'(", goto 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        13, // on ")", goto 12
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 3
        -7, // on "\'(", reduce `Node = Atom => ActionFn(8);`
        -7, // on "(", reduce `Node = Atom => ActionFn(8);`
        -7, // on ")", reduce `Node = Atom => ActionFn(8);`
        -7, // on "*", reduce `Node = Atom => ActionFn(8);`
        -7, // on "+", reduce `Node = Atom => ActionFn(8);`
        -7, // on "-", reduce `Node = Atom => ActionFn(8);`
        -7, // on "/", reduce `Node = Atom => ActionFn(8);`
        -7, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(8);`
        // State 4
        -9, // on "\'(", reduce `Node = Expr => ActionFn(10);`
        -9, // on "(", reduce `Node = Expr => ActionFn(10);`
        -9, // on ")", reduce `Node = Expr => ActionFn(10);`
        -9, // on "*", reduce `Node = Expr => ActionFn(10);`
        -9, // on "+", reduce `Node = Expr => ActionFn(10);`
        -9, // on "-", reduce `Node = Expr => ActionFn(10);`
        -9, // on "/", reduce `Node = Expr => ActionFn(10);`
        -9, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(10);`
        // State 5
        -8, // on "\'(", reduce `Node = List => ActionFn(9);`
        -8, // on "(", reduce `Node = List => ActionFn(9);`
        -8, // on ")", reduce `Node = List => ActionFn(9);`
        -8, // on "*", reduce `Node = List => ActionFn(9);`
        -8, // on "+", reduce `Node = List => ActionFn(9);`
        -8, // on "-", reduce `Node = List => ActionFn(9);`
        -8, // on "/", reduce `Node = List => ActionFn(9);`
        -8, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(9);`
        // State 6
        -12, // on "\'(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on ")", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "*", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "+", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "-", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "/", reduce `Node+ = Node => ActionFn(20);`
        -12, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(20);`
        // State 7
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        20, // on ")", goto 19
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 8
        -2, // on "\'(", reduce `Atom = Num => ActionFn(12);`
        -2, // on "(", reduce `Atom = Num => ActionFn(12);`
        -2, // on ")", reduce `Atom = Num => ActionFn(12);`
        -2, // on "*", reduce `Atom = Num => ActionFn(12);`
        -2, // on "+", reduce `Atom = Num => ActionFn(12);`
        -2, // on "-", reduce `Atom = Num => ActionFn(12);`
        -2, // on "/", reduce `Atom = Num => ActionFn(12);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(12);`
        // State 9
        -1, // on "\'(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(11);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(11);`
        // State 10
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        22, // on ")", goto 21
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 11
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        0, // on r#"[0-9]+"#, error
        // State 12
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 13
        -17, // on "\'(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on ")", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "*", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "+", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "-", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "/", reduce `Sym = "*" => ActionFn(15);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(15);`
        // State 14
        -15, // on "\'(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on ")", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "*", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "+", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "-", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "/", reduce `Sym = "+" => ActionFn(13);`
        -15, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(13);`
        // State 15
        -16, // on "\'(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on ")", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "*", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "+", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "-", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "/", reduce `Sym = "-" => ActionFn(14);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(14);`
        // State 16
        -18, // on "\'(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on ")", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "*", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "+", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "-", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "/", reduce `Sym = "/" => ActionFn(16);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(16);`
        // State 17
        -14, // on "\'(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        // State 18
        -13, // on "\'(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on ")", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "*", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "+", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "-", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "/", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(21);`
        // State 19
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 20
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        24, // on ")", goto 23
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 21
        -5, // on "\'(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on ")", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "*", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "+", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "-", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "/", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on r#"[0-9]+"#, reduce `List = "\'(", ")" => ActionFn(24);`
        // State 22
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        26, // on ")", goto 25
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 23
        -6, // on "\'(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on ")", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "*", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "+", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "-", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "/", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on r#"[0-9]+"#, reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        // State 24
        11, // on "\'(", goto 10
        12, // on "(", goto 11
        27, // on ")", goto 26
        14, // on "*", goto 13
        15, // on "+", goto 14
        16, // on "-", goto 15
        17, // on "/", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 25
        -3, // on "\'(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        // State 26
        -4, // on "\'(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -21, // on EOF, reduce `__List = List => ActionFn(1);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `List = "\'(", ")" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        2, // on List, goto 1
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        8, // on Node+, goto 7
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        21, // on Node+, goto 20
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        23, // on Sym, goto 22
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        25, // on Node+, goto 24
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
    ];
    pub fn parse_List<
        'input,
    >(
        input: &'input str,
    ) -> Result<List, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
    ) -> Option<Result<List,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                8
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                9
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                11
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                12
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__List::parse_List;

mod __parse__Node {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        8, // on "\'(", goto 7
        9, // on "(", goto 8
        0, // on ")", error
        10, // on "*", goto 9
        11, // on "+", goto 10
        12, // on "-", goto 11
        13, // on "/", goto 12
        14, // on r#"[0-9]+"#, goto 13
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 6
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 7
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        24, // on ")", goto 23
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 8
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        0, // on r#"[0-9]+"#, error
        // State 9
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 10
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 11
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 12
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 13
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 14
        -7, // on "\'(", reduce `Node = Atom => ActionFn(8);`
        -7, // on "(", reduce `Node = Atom => ActionFn(8);`
        -7, // on ")", reduce `Node = Atom => ActionFn(8);`
        -7, // on "*", reduce `Node = Atom => ActionFn(8);`
        -7, // on "+", reduce `Node = Atom => ActionFn(8);`
        -7, // on "-", reduce `Node = Atom => ActionFn(8);`
        -7, // on "/", reduce `Node = Atom => ActionFn(8);`
        -7, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(8);`
        // State 15
        -9, // on "\'(", reduce `Node = Expr => ActionFn(10);`
        -9, // on "(", reduce `Node = Expr => ActionFn(10);`
        -9, // on ")", reduce `Node = Expr => ActionFn(10);`
        -9, // on "*", reduce `Node = Expr => ActionFn(10);`
        -9, // on "+", reduce `Node = Expr => ActionFn(10);`
        -9, // on "-", reduce `Node = Expr => ActionFn(10);`
        -9, // on "/", reduce `Node = Expr => ActionFn(10);`
        -9, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(10);`
        // State 16
        -8, // on "\'(", reduce `Node = List => ActionFn(9);`
        -8, // on "(", reduce `Node = List => ActionFn(9);`
        -8, // on ")", reduce `Node = List => ActionFn(9);`
        -8, // on "*", reduce `Node = List => ActionFn(9);`
        -8, // on "+", reduce `Node = List => ActionFn(9);`
        -8, // on "-", reduce `Node = List => ActionFn(9);`
        -8, // on "/", reduce `Node = List => ActionFn(9);`
        -8, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(9);`
        // State 17
        -12, // on "\'(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "(", reduce `Node+ = Node => ActionFn(20);`
        -12, // on ")", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "*", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "+", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "-", reduce `Node+ = Node => ActionFn(20);`
        -12, // on "/", reduce `Node+ = Node => ActionFn(20);`
        -12, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(20);`
        // State 18
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        32, // on ")", goto 31
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 19
        -2, // on "\'(", reduce `Atom = Num => ActionFn(12);`
        -2, // on "(", reduce `Atom = Num => ActionFn(12);`
        -2, // on ")", reduce `Atom = Num => ActionFn(12);`
        -2, // on "*", reduce `Atom = Num => ActionFn(12);`
        -2, // on "+", reduce `Atom = Num => ActionFn(12);`
        -2, // on "-", reduce `Atom = Num => ActionFn(12);`
        -2, // on "/", reduce `Atom = Num => ActionFn(12);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(12);`
        // State 20
        -1, // on "\'(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(11);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(11);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(11);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(11);`
        // State 21
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        34, // on ")", goto 33
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 22
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        0, // on r#"[0-9]+"#, error
        // State 23
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 24
        -17, // on "\'(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "(", reduce `Sym = "*" => ActionFn(15);`
        -17, // on ")", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "*", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "+", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "-", reduce `Sym = "*" => ActionFn(15);`
        -17, // on "/", reduce `Sym = "*" => ActionFn(15);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(15);`
        // State 25
        -15, // on "\'(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "(", reduce `Sym = "+" => ActionFn(13);`
        -15, // on ")", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "*", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "+", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "-", reduce `Sym = "+" => ActionFn(13);`
        -15, // on "/", reduce `Sym = "+" => ActionFn(13);`
        -15, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(13);`
        // State 26
        -16, // on "\'(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "(", reduce `Sym = "-" => ActionFn(14);`
        -16, // on ")", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "*", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "+", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "-", reduce `Sym = "-" => ActionFn(14);`
        -16, // on "/", reduce `Sym = "-" => ActionFn(14);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(14);`
        // State 27
        -18, // on "\'(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "(", reduce `Sym = "/" => ActionFn(16);`
        -18, // on ")", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "*", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "+", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "-", reduce `Sym = "/" => ActionFn(16);`
        -18, // on "/", reduce `Sym = "/" => ActionFn(16);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(16);`
        // State 28
        -14, // on "\'(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        -14, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        // State 29
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        37, // on ")", goto 36
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 30
        -13, // on "\'(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "(", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on ")", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "*", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "+", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "-", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on "/", reduce `Node+ = Node+, Node => ActionFn(21);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(21);`
        // State 31
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 32
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        38, // on ")", goto 37
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 33
        -5, // on "\'(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "(", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on ")", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "*", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "+", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "-", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on "/", reduce `List = "\'(", ")" => ActionFn(24);`
        -5, // on r#"[0-9]+"#, reduce `List = "\'(", ")" => ActionFn(24);`
        // State 34
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        40, // on ")", goto 39
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 35
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        41, // on ")", goto 40
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 36
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 37
        -6, // on "\'(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "(", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on ")", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "*", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "+", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "-", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on "/", reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        -6, // on r#"[0-9]+"#, reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        // State 38
        22, // on "\'(", goto 21
        23, // on "(", goto 22
        42, // on ")", goto 41
        25, // on "*", goto 24
        26, // on "+", goto 25
        27, // on "-", goto 26
        28, // on "/", goto 27
        29, // on r#"[0-9]+"#, goto 28
        // State 39
        -3, // on "\'(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        // State 40
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 41
        -4, // on "\'(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -7, // on EOF, reduce `Node = Atom => ActionFn(8);`
        -9, // on EOF, reduce `Node = Expr => ActionFn(10);`
        -8, // on EOF, reduce `Node = List => ActionFn(9);`
        -22, // on EOF, reduce `__Node = Node => ActionFn(2);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(12);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(11);`
        0, // on EOF, error
        0, // on EOF, error
        -17, // on EOF, reduce `Sym = "*" => ActionFn(15);`
        -15, // on EOF, reduce `Sym = "+" => ActionFn(13);`
        -16, // on EOF, reduce `Sym = "-" => ActionFn(14);`
        -18, // on EOF, reduce `Sym = "/" => ActionFn(16);`
        -14, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `List = "\'(", ")" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `List = "\'(", Node+, ")" => ActionFn(25);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(22);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(23);`
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        3, // on Expr, goto 2
        4, // on List, goto 3
        5, // on Node, goto 4
        0, // on Node*, error
        0, // on Node+, error
        6, // on Num, goto 5
        7, // on Sym, goto 6
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        18, // on Node, goto 17
        0, // on Node*, error
        19, // on Node+, goto 18
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        30, // on Sym, goto 29
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        31, // on Node, goto 30
        0, // on Node*, error
        0, // on Node+, error
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        18, // on Node, goto 17
        0, // on Node*, error
        33, // on Node+, goto 32
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        35, // on Sym, goto 34
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 28
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 29
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        18, // on Node, goto 17
        0, // on Node*, error
        36, // on Node+, goto 35
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 30
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 31
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 32
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        31, // on Node, goto 30
        0, // on Node*, error
        0, // on Node+, error
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 33
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 34
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        18, // on Node, goto 17
        0, // on Node*, error
        39, // on Node+, goto 38
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 35
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        31, // on Node, goto 30
        0, // on Node*, error
        0, // on Node+, error
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 36
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 37
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 38
        15, // on Atom, goto 14
        16, // on Expr, goto 15
        17, // on List, goto 16
        31, // on Node, goto 30
        0, // on Node*, error
        0, // on Node+, error
        20, // on Num, goto 19
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 39
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 40
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 41
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
    ];
    pub fn parse_Node<
        'input,
    >(
        input: &'input str,
    ) -> Result<Node, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
    ) -> Option<Result<Node,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                8
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                9
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                10
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                12
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Node::parse_Node;

mod __parse__Num {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        3, // on r#"[0-9]+"#, goto 2
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -23, // on EOF, reduce `__Num = Num => ActionFn(5);`
        -14, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(17);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        2, // on Num, goto 1
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
    ];
    pub fn parse_Num<
        'input,
    >(
        input: &'input str,
    ) -> Result<i64, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
    ) -> Option<Result<i64,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                8
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                9
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                10
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                11
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                return Some(Ok(__nt));
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Num::parse_Num;

mod __parse__Sym {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27_28_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        3, // on "*", goto 2
        4, // on "+", goto 3
        5, // on "-", goto 4
        6, // on "/", goto 5
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "\'(", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -24, // on EOF, reduce `__Sym = Sym => ActionFn(4);`
        -17, // on EOF, reduce `Sym = "*" => ActionFn(15);`
        -15, // on EOF, reduce `Sym = "+" => ActionFn(13);`
        -16, // on EOF, reduce `Sym = "-" => ActionFn(14);`
        -18, // on EOF, reduce `Sym = "/" => ActionFn(16);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        2, // on Sym, goto 1
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
    ];
    pub fn parse_Sym<
        'input,
    >(
        input: &'input str,
    ) -> Result<Symbol, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_5c_27_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
    ) -> Option<Result<Symbol,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Sym => ActionFn(11);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(12);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(23);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // List = "\'(", ")" => ActionFn(24);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            6 => {
                // List = "\'(", Node+, ")" => ActionFn(25);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5c_27_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                2
            }
            7 => {
                // Node = Atom => ActionFn(8);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            8 => {
                // Node = List => ActionFn(9);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            9 => {
                // Node = Expr => ActionFn(10);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                3
            }
            10 => {
                // Node* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            11 => {
                // Node* = Node+ => ActionFn(19);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                4
            }
            12 => {
                // Node+ = Node => ActionFn(20);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node+, Node => ActionFn(21);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                5
            }
            14 => {
                // Num = r#"[0-9]+"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                6
            }
            15 => {
                // Sym = "+" => ActionFn(13);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            16 => {
                // Sym = "-" => ActionFn(14);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            17 => {
                // Sym = "*" => ActionFn(15);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            18 => {
                // Sym = "/" => ActionFn(16);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                7
            }
            19 => {
                // __Atom = Atom => ActionFn(3);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                8
            }
            20 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                9
            }
            21 => {
                // __List = List => ActionFn(1);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                10
            }
            22 => {
                // __Node = Node => ActionFn(2);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                11
            }
            23 => {
                // __Num = Num => ActionFn(5);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                12
            }
            24 => {
                // __Sym = Sym => ActionFn(4);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNode_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Node>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNode_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSym(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____List<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, List, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____List(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Node<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Node(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Sym<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Symbol, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Sym(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Sym::parse_Sym;
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
                        39 => /* '\'' */ {
                            __current_state = 1;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 10;
                            continue;
                        }
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
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
    (_, __0, _): (usize, List, usize),
) -> List
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
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
    (_, __0, _): (usize, Symbol, usize),
) -> Symbol
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> i64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Symbol, usize),
    (_, __1, _): (usize, ::std::vec::Vec<Node>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    Expr::new(__0, __1)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<Node>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> List
{
    List::from(__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Node
{
    Node::Atom(__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, List, usize),
) -> Node
{
    Node::List(__0)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Node
{
    Node::Expr(__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
) -> Atom
{
    Atom::from(__0)
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Atom
{
    Atom::from(__0)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Add
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Sub
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Mul
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Div
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Node>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Node>, usize),
) -> ::std::vec::Vec<Node>
{
    v
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> ::std::vec::Vec<Node>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Node>, usize),
    (_, e, _): (usize, Node, usize),
) -> ::std::vec::Vec<Node>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Symbol, usize),
    __2: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Symbol, usize),
    __2: (usize, ::std::vec::Vec<Node>, usize),
    __3: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action19(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> List
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Node>, usize),
    __2: (usize, &'input str, usize),
) -> List
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action19(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
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

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
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on "[", error
        0, // on "]", error
        9, // on r#"[0-9]+"#, goto 8
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 8
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -20, // on EOF, reduce `__Atom = Atom => ActionFn(4);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        -15, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        3, // on Num, goto 2
        4, // on Sym, goto 3
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                return Some(Ok(__nt));
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, // on "(", goto 2
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 3
        16, // on "(", goto 15
        17, // on ")", goto 16
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 4
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 5
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 6
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 7
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 8
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 9
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 10
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 11
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        0, // on "]", error
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 12
        16, // on "(", goto 15
        21, // on ")", goto 20
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 13
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 14
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 15
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 16
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 17
        30, // on "(", goto 29
        0, // on ")", error
        31, // on "*", goto 30
        32, // on "+", goto 31
        33, // on "-", goto 32
        34, // on "/", goto 33
        35, // on "[", goto 34
        36, // on "]", goto 35
        37, // on r#"[0-9]+"#, goto 36
        // State 18
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on "]", error
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 19
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on "]", error
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 20
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 21
        16, // on "(", goto 15
        39, // on ")", goto 38
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 22
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 23
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 24
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 25
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(22);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 26
        30, // on "(", goto 29
        0, // on ")", error
        31, // on "*", goto 30
        32, // on "+", goto 31
        33, // on "-", goto 32
        34, // on "/", goto 33
        35, // on "[", goto 34
        41, // on "]", goto 40
        37, // on r#"[0-9]+"#, goto 36
        // State 27
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 28
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 29
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 30
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 31
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 32
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 33
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 34
        30, // on "(", goto 29
        0, // on ")", error
        31, // on "*", goto 30
        32, // on "+", goto 31
        33, // on "-", goto 32
        34, // on "/", goto 33
        35, // on "[", goto 34
        44, // on "]", goto 43
        37, // on r#"[0-9]+"#, goto 36
        // State 35
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        0, // on "]", error
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 36
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "]", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 37
        16, // on "(", goto 15
        45, // on ")", goto 44
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 38
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on "]", error
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 39
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 40
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on "]", error
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 41
        16, // on "(", goto 15
        47, // on ")", goto 46
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 42
        30, // on "(", goto 29
        0, // on ")", error
        31, // on "*", goto 30
        32, // on "+", goto 31
        33, // on "-", goto 32
        34, // on "/", goto 33
        35, // on "[", goto 34
        48, // on "]", goto 47
        37, // on r#"[0-9]+"#, goto 36
        // State 43
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 44
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on "]", error
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        // State 45
        16, // on "(", goto 15
        49, // on ")", goto 48
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        18, // on "[", goto 17
        0, // on "]", error
        19, // on r#"[0-9]+"#, goto 18
        // State 46
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 47
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 48
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -21, // on EOF, reduce `__Expr = Expr => ActionFn(1);`
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
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
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
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        2, // on Expr, goto 1
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        4, // on Sym, goto 3
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        13, // on Node+, goto 12
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        22, // on Sym, goto 21
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        23, // on Atom, goto 22
        24, // on Expr, goto 23
        0, // on Lang, error
        25, // on List, goto 24
        26, // on Node, goto 25
        0, // on Node*, error
        27, // on Node+, goto 26
        28, // on Num, goto 27
        29, // on Sym, goto 28
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        38, // on Node+, goto 37
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        23, // on Atom, goto 22
        24, // on Expr, goto 23
        0, // on Lang, error
        25, // on List, goto 24
        40, // on Node, goto 39
        0, // on Node*, error
        0, // on Node+, error
        28, // on Num, goto 27
        29, // on Sym, goto 28
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 28
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 29
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 30
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 31
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 32
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 33
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 34
        23, // on Atom, goto 22
        24, // on Expr, goto 23
        0, // on Lang, error
        25, // on List, goto 24
        26, // on Node, goto 25
        0, // on Node*, error
        43, // on Node+, goto 42
        28, // on Num, goto 27
        29, // on Sym, goto 28
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 35
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 36
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 37
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 38
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 39
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 40
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 41
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        12, // on Node, goto 11
        0, // on Node*, error
        46, // on Node+, goto 45
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 42
        23, // on Atom, goto 22
        24, // on Expr, goto 23
        0, // on Lang, error
        25, // on List, goto 24
        40, // on Node, goto 39
        0, // on Node*, error
        0, // on Node+, error
        28, // on Num, goto 27
        29, // on Sym, goto 28
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 43
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 44
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 45
        9, // on Atom, goto 8
        10, // on Expr, goto 9
        0, // on Lang, error
        11, // on List, goto 10
        20, // on Node, goto 19
        0, // on Node*, error
        0, // on Node+, error
        14, // on Num, goto 13
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 46
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 47
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 48
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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

mod __parse__Lang {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        9, // on "(", goto 8
        0, // on ")", error
        10, // on "*", goto 9
        11, // on "+", goto 10
        12, // on "-", goto 11
        13, // on "/", goto 12
        14, // on "[", goto 13
        0, // on "]", error
        15, // on r#"[0-9]+"#, goto 14
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 8
        0, // on "(", error
        0, // on ")", error
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 9
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 10
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 12
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 13
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        34, // on "]", goto 33
        35, // on r#"[0-9]+"#, goto 34
        // State 14
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 15
        43, // on "(", goto 42
        44, // on ")", goto 43
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 16
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 17
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 18
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 19
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 20
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 21
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 22
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 23
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(22);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 24
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        48, // on "]", goto 47
        35, // on r#"[0-9]+"#, goto 34
        // State 25
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 26
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 27
        0, // on "(", error
        0, // on ")", error
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 28
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 29
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 30
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 31
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 32
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        51, // on "]", goto 50
        35, // on r#"[0-9]+"#, goto 34
        // State 33
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 34
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "]", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 35
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 36
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 37
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 38
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        0, // on "]", error
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 39
        43, // on "(", goto 42
        53, // on ")", goto 52
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 40
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 41
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 42
        0, // on "(", error
        0, // on ")", error
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 43
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 44
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        56, // on "]", goto 55
        35, // on r#"[0-9]+"#, goto 34
        // State 45
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on "]", error
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 46
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 47
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 48
        43, // on "(", goto 42
        58, // on ")", goto 57
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 49
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        59, // on "]", goto 58
        35, // on r#"[0-9]+"#, goto 34
        // State 50
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 51
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on "]", error
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 52
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 53
        43, // on "(", goto 42
        61, // on ")", goto 60
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 54
        28, // on "(", goto 27
        0, // on ")", error
        29, // on "*", goto 28
        30, // on "+", goto 29
        31, // on "-", goto 30
        32, // on "/", goto 31
        33, // on "[", goto 32
        62, // on "]", goto 61
        35, // on r#"[0-9]+"#, goto 34
        // State 55
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        0, // on "]", error
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 56
        43, // on "(", goto 42
        63, // on ")", goto 62
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 57
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 58
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 59
        43, // on "(", goto 42
        64, // on ")", goto 63
        17, // on "*", goto 16
        18, // on "+", goto 17
        19, // on "-", goto 18
        20, // on "/", goto 19
        45, // on "[", goto 44
        0, // on "]", error
        46, // on r#"[0-9]+"#, goto 45
        // State 60
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on "]", error
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 61
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on "]", error
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 62
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        // State 63
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on "]", error
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -8, // on EOF, reduce `Node = Atom => ActionFn(10);`
        -10, // on EOF, reduce `Node = Expr => ActionFn(12);`
        -22, // on EOF, reduce `__Lang = Lang => ActionFn(0);`
        -9, // on EOF, reduce `Node = List => ActionFn(11);`
        -5, // on EOF, reduce `Lang = Node => ActionFn(7);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        0, // on EOF, error
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        0, // on EOF, error
        -15, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
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
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(26);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
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
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        3, // on Expr, goto 2
        4, // on Lang, goto 3
        5, // on List, goto 4
        6, // on Node, goto 5
        0, // on Node*, error
        0, // on Node+, error
        7, // on Num, goto 6
        8, // on Sym, goto 7
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        16, // on Sym, goto 15
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        24, // on Node, goto 23
        0, // on Node*, error
        25, // on Node+, goto 24
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        39, // on Node, goto 38
        0, // on Node*, error
        40, // on Node+, goto 39
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        47, // on Node, goto 46
        0, // on Node*, error
        0, // on Node+, error
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        49, // on Sym, goto 48
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 28
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 29
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 30
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 31
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 32
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        24, // on Node, goto 23
        0, // on Node*, error
        50, // on Node+, goto 49
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 33
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 34
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 35
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 36
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 37
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 38
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 39
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        52, // on Node, goto 51
        0, // on Node*, error
        0, // on Node+, error
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 40
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 41
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 42
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        54, // on Sym, goto 53
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 43
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 44
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        24, // on Node, goto 23
        0, // on Node*, error
        55, // on Node+, goto 54
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 45
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 46
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 47
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 48
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        39, // on Node, goto 38
        0, // on Node*, error
        57, // on Node+, goto 56
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 49
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        47, // on Node, goto 46
        0, // on Node*, error
        0, // on Node+, error
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 50
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 51
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 52
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 53
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        39, // on Node, goto 38
        0, // on Node*, error
        60, // on Node+, goto 59
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 54
        21, // on Atom, goto 20
        22, // on Expr, goto 21
        0, // on Lang, error
        23, // on List, goto 22
        47, // on Node, goto 46
        0, // on Node*, error
        0, // on Node+, error
        26, // on Num, goto 25
        27, // on Sym, goto 26
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 55
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 56
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        52, // on Node, goto 51
        0, // on Node*, error
        0, // on Node+, error
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 57
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 58
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 59
        36, // on Atom, goto 35
        37, // on Expr, goto 36
        0, // on Lang, error
        38, // on List, goto 37
        52, // on Node, goto 51
        0, // on Node*, error
        0, // on Node+, error
        41, // on Num, goto 40
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 60
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 61
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 62
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 63
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
    ];
    pub fn parse_Lang<
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
pub use self::__parse__Lang::parse_Lang;

mod __parse__List {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use atom::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        3, // on "[", goto 2
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        17, // on "]", goto 16
        18, // on r#"[0-9]+"#, goto 17
        // State 3
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 4
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 5
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 6
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(22);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 7
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        20, // on "]", goto 19
        18, // on r#"[0-9]+"#, goto 17
        // State 8
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 9
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 10
        0, // on "(", error
        0, // on ")", error
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 11
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 12
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 13
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 14
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 15
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        27, // on "]", goto 26
        18, // on r#"[0-9]+"#, goto 17
        // State 16
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 17
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "]", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 18
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 19
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 20
        35, // on "(", goto 34
        36, // on ")", goto 35
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        37, // on "[", goto 36
        0, // on "]", error
        38, // on r#"[0-9]+"#, goto 37
        // State 21
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 22
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 23
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 24
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 25
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        39, // on "]", goto 38
        18, // on r#"[0-9]+"#, goto 17
        // State 26
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 27
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 28
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 29
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 30
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        0, // on "]", error
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 31
        35, // on "(", goto 34
        41, // on ")", goto 40
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        37, // on "[", goto 36
        0, // on "]", error
        38, // on r#"[0-9]+"#, goto 37
        // State 32
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 33
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 34
        0, // on "(", error
        0, // on ")", error
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 35
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 36
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        44, // on "]", goto 43
        18, // on r#"[0-9]+"#, goto 17
        // State 37
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on "]", error
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 38
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 39
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on "]", error
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 40
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        // State 41
        35, // on "(", goto 34
        46, // on ")", goto 45
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        37, // on "[", goto 36
        0, // on "]", error
        38, // on r#"[0-9]+"#, goto 37
        // State 42
        11, // on "(", goto 10
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        16, // on "[", goto 15
        47, // on "]", goto 46
        18, // on r#"[0-9]+"#, goto 17
        // State 43
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        0, // on "]", error
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 44
        35, // on "(", goto 34
        48, // on ")", goto 47
        22, // on "*", goto 21
        23, // on "+", goto 22
        24, // on "-", goto 23
        25, // on "/", goto 24
        37, // on "[", goto 36
        0, // on "]", error
        38, // on r#"[0-9]+"#, goto 37
        // State 45
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on "]", error
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 46
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on "]", error
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 47
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on "]", error
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -23, // on EOF, reduce `__List = List => ActionFn(2);`
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
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(26);`
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(27);`
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
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        2, // on List, goto 1
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        8, // on Node+, goto 7
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        21, // on Sym, goto 20
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        26, // on Node+, goto 25
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        28, // on Atom, goto 27
        29, // on Expr, goto 28
        0, // on Lang, error
        30, // on List, goto 29
        31, // on Node, goto 30
        0, // on Node*, error
        32, // on Node+, goto 31
        33, // on Num, goto 32
        34, // on Sym, goto 33
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 28
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 29
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 30
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 31
        28, // on Atom, goto 27
        29, // on Expr, goto 28
        0, // on Lang, error
        30, // on List, goto 29
        40, // on Node, goto 39
        0, // on Node*, error
        0, // on Node+, error
        33, // on Num, goto 32
        34, // on Sym, goto 33
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 32
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 33
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 34
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        42, // on Sym, goto 41
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 35
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 36
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        43, // on Node+, goto 42
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 37
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 38
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 39
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 40
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 41
        28, // on Atom, goto 27
        29, // on Expr, goto 28
        0, // on Lang, error
        30, // on List, goto 29
        31, // on Node, goto 30
        0, // on Node*, error
        45, // on Node+, goto 44
        33, // on Num, goto 32
        34, // on Sym, goto 33
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 42
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        19, // on Node, goto 18
        0, // on Node*, error
        0, // on Node+, error
        9, // on Num, goto 8
        10, // on Sym, goto 9
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 43
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 44
        28, // on Atom, goto 27
        29, // on Expr, goto 28
        0, // on Lang, error
        30, // on List, goto 29
        40, // on Node, goto 39
        0, // on Node*, error
        0, // on Node+, error
        33, // on Num, goto 32
        34, // on Sym, goto 33
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 45
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 46
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 47
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        8, // on "(", goto 7
        0, // on ")", error
        9, // on "*", goto 8
        10, // on "+", goto 9
        11, // on "-", goto 10
        12, // on "/", goto 11
        13, // on "[", goto 12
        0, // on "]", error
        14, // on r#"[0-9]+"#, goto 13
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 8
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 9
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 10
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 12
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        33, // on "]", goto 32
        34, // on r#"[0-9]+"#, goto 33
        // State 13
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 14
        42, // on "(", goto 41
        43, // on ")", goto 42
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 15
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 16
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 17
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 18
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 19
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 20
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 21
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 22
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(22);`
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 23
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        47, // on "]", goto 46
        34, // on r#"[0-9]+"#, goto 33
        // State 24
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 25
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 26
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 27
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 28
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 29
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 30
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 31
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        50, // on "]", goto 49
        34, // on r#"[0-9]+"#, goto 33
        // State 32
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 33
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "]", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 34
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on r#"[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 35
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on r#"[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 36
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on r#"[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 37
        -13, // on "(", reduce `Node+ = Node => ActionFn(22);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(22);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(22);`
        0, // on "]", error
        -13, // on r#"[0-9]+"#, reduce `Node+ = Node => ActionFn(22);`
        // State 38
        42, // on "(", goto 41
        52, // on ")", goto 51
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 39
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on r#"[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 40
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on r#"[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 41
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 42
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 43
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        55, // on "]", goto 54
        34, // on r#"[0-9]+"#, goto 33
        // State 44
        -15, // on "(", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        -15, // on "[", reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        0, // on "]", error
        -15, // on r#"[0-9]+"#, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
        // State 45
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 46
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 47
        42, // on "(", goto 41
        57, // on ")", goto 56
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 48
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        58, // on "]", goto 57
        34, // on r#"[0-9]+"#, goto 33
        // State 49
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 50
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(23);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(23);`
        0, // on "]", error
        -14, // on r#"[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(23);`
        // State 51
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 52
        42, // on "(", goto 41
        60, // on ")", goto 59
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 53
        27, // on "(", goto 26
        0, // on ")", error
        28, // on "*", goto 27
        29, // on "+", goto 28
        30, // on "-", goto 29
        31, // on "/", goto 30
        32, // on "[", goto 31
        61, // on "]", goto 60
        34, // on r#"[0-9]+"#, goto 33
        // State 54
        -6, // on "(", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(26);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(26);`
        0, // on "]", error
        -6, // on r#"[0-9]+"#, reduce `List = "[", "]" => ActionFn(26);`
        // State 55
        42, // on "(", goto 41
        62, // on ")", goto 61
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 56
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 57
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 58
        42, // on "(", goto 41
        63, // on ")", goto 62
        16, // on "*", goto 15
        17, // on "+", goto 16
        18, // on "-", goto 17
        19, // on "/", goto 18
        44, // on "[", goto 43
        0, // on "]", error
        45, // on r#"[0-9]+"#, goto 44
        // State 59
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on "]", error
        -3, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        // State 60
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(27);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on "]", error
        -7, // on r#"[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(27);`
        // State 61
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        // State 62
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
        0, // on "]", error
        -4, // on r#"[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -8, // on EOF, reduce `Node = Atom => ActionFn(10);`
        -10, // on EOF, reduce `Node = Expr => ActionFn(12);`
        -9, // on EOF, reduce `Node = List => ActionFn(11);`
        -24, // on EOF, reduce `__Node = Node => ActionFn(3);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        0, // on EOF, error
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        0, // on EOF, error
        -15, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
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
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(26);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(27);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(25);`
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
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        3, // on Expr, goto 2
        0, // on Lang, error
        4, // on List, goto 3
        5, // on Node, goto 4
        0, // on Node*, error
        0, // on Node+, error
        6, // on Num, goto 5
        7, // on Sym, goto 6
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 6
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 7
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        15, // on Sym, goto 14
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 8
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 9
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 10
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 11
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 12
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        23, // on Node, goto 22
        0, // on Node*, error
        24, // on Node+, goto 23
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 13
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 14
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        38, // on Node, goto 37
        0, // on Node*, error
        39, // on Node+, goto 38
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 15
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 16
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 17
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 18
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 19
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 20
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 21
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 22
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 23
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 24
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 25
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 26
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        48, // on Sym, goto 47
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 27
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 28
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 29
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 30
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 31
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        23, // on Node, goto 22
        0, // on Node*, error
        49, // on Node+, goto 48
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 32
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 33
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 34
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 35
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 36
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 37
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 38
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        51, // on Node, goto 50
        0, // on Node*, error
        0, // on Node+, error
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 39
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 40
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 41
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        53, // on Sym, goto 52
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 42
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 43
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        23, // on Node, goto 22
        0, // on Node*, error
        54, // on Node+, goto 53
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 44
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 45
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 46
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 47
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        38, // on Node, goto 37
        0, // on Node*, error
        56, // on Node+, goto 55
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 48
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 49
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 50
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 51
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 52
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        38, // on Node, goto 37
        0, // on Node*, error
        59, // on Node+, goto 58
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 53
        20, // on Atom, goto 19
        21, // on Expr, goto 20
        0, // on Lang, error
        22, // on List, goto 21
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        25, // on Num, goto 24
        26, // on Sym, goto 25
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 54
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 55
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        51, // on Node, goto 50
        0, // on Node*, error
        0, // on Node+, error
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 56
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 57
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 58
        35, // on Atom, goto 34
        36, // on Expr, goto 35
        0, // on Lang, error
        37, // on List, goto 36
        51, // on Node, goto 50
        0, // on Node*, error
        0, // on Node+, error
        40, // on Num, goto 39
        41, // on Sym, goto 40
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 59
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 60
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 61
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 62
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        3, // on r#"[0-9]+"#, goto 2
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -25, // on EOF, reduce `__Num = Num => ActionFn(6);`
        -15, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(19);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        2, // on Num, goto 1
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                return Some(Ok(__nt));
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Sym(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i64),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i64),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        3, // on "*", goto 2
        4, // on "+", goto 3
        5, // on "-", goto 4
        6, // on "/", goto 5
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on r#"[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -26, // on EOF, reduce `__Sym = Sym => ActionFn(5);`
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        2, // on Sym, goto 1
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 1
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 2
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 3
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 4
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 5
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
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
                (_, (8, _), _) if true => 8,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Atom = Sym => ActionFn(13);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Num => ActionFn(14);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Expr = "(", Sym, ")" => ActionFn(24);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(25);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lang = Node => ActionFn(7);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLang(__nt), __end));
                2
            }
            6 => {
                // List = "[", "]" => ActionFn(26);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            8 => {
                // Node = Atom => ActionFn(10);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            9 => {
                // Node = List => ActionFn(11);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            10 => {
                // Node = Expr => ActionFn(12);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode(__nt), __end));
                4
            }
            11 => {
                // Node* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(21);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(22);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(23);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"[0-9]+"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                7
            }
            16 => {
                // Sym = "+" => ActionFn(15);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            17 => {
                // Sym = "-" => ActionFn(16);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            18 => {
                // Sym = "*" => ActionFn(17);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            19 => {
                // Sym = "/" => ActionFn(18);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            20 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Atom(__nt), __end));
                9
            }
            21 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                10
            }
            22 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Lang(__nt), __end));
                11
            }
            23 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____List(__nt), __end));
                12
            }
            24 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Node(__nt), __end));
                13
            }
            25 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                14
            }
            26 => {
                // __Sym = Sym => ActionFn(5);
                let __sym0 = __pop_NtSym(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
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
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtLang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLang(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____Lang<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Lang(__v), __r) => (__l, __v, __r),
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
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((8, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
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
                        48 ... 57 => {
                            __current_match = Some((8, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
                            __current_match = Some((8, __index + __ch.len_utf8()));
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
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, List, usize),
) -> List
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Atom
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
) -> Symbol
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> i64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
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
pub fn __action9<
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
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Atom, usize),
) -> Node
{
    Node::Atom(__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, List, usize),
) -> Node
{
    Node::List(__0)
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Node
{
    Node::Expr(__0)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Symbol, usize),
) -> Atom
{
    Atom::from(__0)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Atom
{
    Atom::from(__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Add
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Sub
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Mul
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Div
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action20<
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
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Node>, usize),
) -> ::std::vec::Vec<Node>
{
    v
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> ::std::vec::Vec<Node>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action23<
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
pub fn __action24<
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
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action25<
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
    let __temp0 = __action21(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> List
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action27<
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
    let __temp0 = __action21(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
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

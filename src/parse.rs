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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        5, // on "%", goto 4
        0, // on "(", error
        0, // on ")", error
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        0, // on "[", error
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        12, // on r#"-?[0-9]+"#, goto 11
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 6
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 7
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 8
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 9
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 10
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 11
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -23, // on EOF, reduce `__Atom = Atom => ActionFn(4);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        -20, // on EOF, reduce `Sym = "%" => ActionFn(19);`
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        -21, // on EOF, reduce `Sym = "head" => ActionFn(20);`
        -22, // on EOF, reduce `Sym = "tail" => ActionFn(21);`
        -15, // on EOF, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
                // __Atom = Atom => ActionFn(4);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                return Some(Ok(__nt));
            }
            24 => {
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
            25 => {
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
            26 => {
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
            27 => {
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
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "%", error
        3, // on "(", goto 2
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        5, // on "%", goto 4
        0, // on "(", error
        0, // on ")", error
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        0, // on "[", error
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        0, // on r#"-?[0-9]+"#, error
        // State 3
        5, // on "%", goto 4
        19, // on "(", goto 18
        20, // on ")", goto 19
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 4
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        -20, // on ")", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        0, // on "]", error
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 5
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 6
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 7
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 8
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 9
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        -21, // on ")", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        0, // on "]", error
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 10
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on ")", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        0, // on "]", error
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 11
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 12
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 13
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 14
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        0, // on "]", error
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 15
        5, // on "%", goto 4
        19, // on "(", goto 18
        24, // on ")", goto 23
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 16
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 17
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 18
        5, // on "%", goto 4
        0, // on "(", error
        0, // on ")", error
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        0, // on "[", error
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        0, // on r#"-?[0-9]+"#, error
        // State 19
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 20
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        40, // on "]", goto 39
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 21
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on ")", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on "]", error
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 22
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on "]", error
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 23
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 24
        5, // on "%", goto 4
        19, // on "(", goto 18
        45, // on ")", goto 44
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 25
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 26
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 27
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 28
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 29
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        47, // on "]", goto 46
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 30
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 31
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 32
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        0, // on ")", error
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "]", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 33
        5, // on "%", goto 4
        0, // on "(", error
        0, // on ")", error
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        0, // on "[", error
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        0, // on r#"-?[0-9]+"#, error
        // State 34
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 35
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 36
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 37
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 38
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        50, // on "]", goto 49
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 39
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        0, // on "]", error
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 40
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        0, // on ")", error
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "]", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 41
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        0, // on ")", error
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "]", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 42
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "]", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 43
        5, // on "%", goto 4
        19, // on "(", goto 18
        51, // on ")", goto 50
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 44
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on "]", error
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 45
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 46
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on "]", error
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 47
        5, // on "%", goto 4
        19, // on "(", goto 18
        53, // on ")", goto 52
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 48
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        54, // on "]", goto 53
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 49
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 50
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on "]", error
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        // State 51
        5, // on "%", goto 4
        19, // on "(", goto 18
        55, // on ")", goto 54
        6, // on "*", goto 5
        7, // on "+", goto 6
        8, // on "-", goto 7
        9, // on "/", goto 8
        21, // on "[", goto 20
        0, // on "]", error
        10, // on "head", goto 9
        11, // on "tail", goto 10
        22, // on r#"-?[0-9]+"#, goto 21
        // State 52
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 53
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 54
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -24, // on EOF, reduce `__Expr = Expr => ActionFn(1);`
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
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
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
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        15, // on Node, goto 14
        0, // on Node*, error
        16, // on Node+, goto 15
        17, // on Num, goto 16
        18, // on Sym, goto 17
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
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        23, // on Node, goto 22
        0, // on Node*, error
        0, // on Node+, error
        17, // on Num, goto 16
        18, // on Sym, goto 17
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
        25, // on Sym, goto 24
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        29, // on Node, goto 28
        0, // on Node*, error
        30, // on Node+, goto 29
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        15, // on Node, goto 14
        0, // on Node*, error
        44, // on Node+, goto 43
        17, // on Num, goto 16
        18, // on Sym, goto 17
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        48, // on Sym, goto 47
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        29, // on Node, goto 28
        0, // on Node*, error
        49, // on Node+, goto 48
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        0, // on Sym, error
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 43
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        23, // on Node, goto 22
        0, // on Node*, error
        0, // on Node+, error
        17, // on Num, goto 16
        18, // on Sym, goto 17
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
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        15, // on Node, goto 14
        0, // on Node*, error
        52, // on Node+, goto 51
        17, // on Num, goto 16
        18, // on Sym, goto 17
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 48
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        12, // on Atom, goto 11
        13, // on Expr, goto 12
        0, // on Lang, error
        14, // on List, goto 13
        23, // on Node, goto 22
        0, // on Node*, error
        0, // on Node+, error
        17, // on Num, goto 16
        18, // on Sym, goto 17
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            25 => {
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
            26 => {
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
            27 => {
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
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        9, // on "%", goto 8
        10, // on "(", goto 9
        0, // on ")", error
        11, // on "*", goto 10
        12, // on "+", goto 11
        13, // on "-", goto 12
        14, // on "/", goto 13
        15, // on "[", goto 14
        0, // on "]", error
        16, // on "head", goto 15
        17, // on "tail", goto 16
        18, // on r#"-?[0-9]+"#, goto 17
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 6
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 7
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 8
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 9
        20, // on "%", goto 19
        0, // on "(", error
        0, // on ")", error
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        0, // on "[", error
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        0, // on r#"-?[0-9]+"#, error
        // State 10
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 11
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 12
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 13
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 14
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        41, // on "]", goto 40
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 15
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 16
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 17
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 18
        20, // on "%", goto 19
        52, // on "(", goto 51
        53, // on ")", goto 52
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 19
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        -20, // on ")", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        0, // on "]", error
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 20
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 21
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 22
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 23
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 24
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        -21, // on ")", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        0, // on "]", error
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 25
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on ")", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        0, // on "]", error
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 26
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 27
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 28
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 29
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 30
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        57, // on "]", goto 56
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 31
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 32
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 33
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        0, // on ")", error
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "]", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 34
        20, // on "%", goto 19
        0, // on "(", error
        0, // on ")", error
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        0, // on "[", error
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        0, // on r#"-?[0-9]+"#, error
        // State 35
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 36
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 37
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 38
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 39
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        60, // on "]", goto 59
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 40
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 41
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        0, // on ")", error
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "]", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 42
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        0, // on ")", error
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "]", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 43
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "]", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 44
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 45
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 46
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 47
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        0, // on "]", error
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 48
        20, // on "%", goto 19
        52, // on "(", goto 51
        62, // on ")", goto 61
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 49
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 50
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 51
        20, // on "%", goto 19
        0, // on "(", error
        0, // on ")", error
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        0, // on "[", error
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        0, // on r#"-?[0-9]+"#, error
        // State 52
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 53
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        65, // on "]", goto 64
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 54
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on ")", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on "]", error
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 55
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 56
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 57
        20, // on "%", goto 19
        52, // on "(", goto 51
        67, // on ")", goto 66
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 58
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        68, // on "]", goto 67
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 59
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 60
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on "]", error
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 61
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 62
        20, // on "%", goto 19
        52, // on "(", goto 51
        70, // on ")", goto 69
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 63
        34, // on "%", goto 33
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        37, // on "+", goto 36
        38, // on "-", goto 37
        39, // on "/", goto 38
        40, // on "[", goto 39
        71, // on "]", goto 70
        42, // on "head", goto 41
        43, // on "tail", goto 42
        44, // on r#"-?[0-9]+"#, goto 43
        // State 64
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        0, // on "]", error
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 65
        20, // on "%", goto 19
        52, // on "(", goto 51
        72, // on ")", goto 71
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 66
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 67
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 68
        20, // on "%", goto 19
        52, // on "(", goto 51
        73, // on ")", goto 72
        21, // on "*", goto 20
        22, // on "+", goto 21
        23, // on "-", goto 22
        24, // on "/", goto 23
        54, // on "[", goto 53
        0, // on "]", error
        25, // on "head", goto 24
        26, // on "tail", goto 25
        55, // on r#"-?[0-9]+"#, goto 54
        // State 69
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on "]", error
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 70
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on "]", error
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 71
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        // State 72
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on "]", error
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -8, // on EOF, reduce `Node = Atom => ActionFn(10);`
        -10, // on EOF, reduce `Node = Expr => ActionFn(12);`
        -25, // on EOF, reduce `__Lang = Lang => ActionFn(0);`
        -9, // on EOF, reduce `Node = List => ActionFn(11);`
        -5, // on EOF, reduce `Lang = Node => ActionFn(7);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        -20, // on EOF, reduce `Sym = "%" => ActionFn(19);`
        0, // on EOF, error
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        0, // on EOF, error
        -21, // on EOF, reduce `Sym = "head" => ActionFn(20);`
        -22, // on EOF, reduce `Sym = "tail" => ActionFn(21);`
        -15, // on EOF, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
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
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(29);`
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
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
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
        19, // on Sym, goto 18
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
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        30, // on Node, goto 29
        0, // on Node*, error
        31, // on Node+, goto 30
        32, // on Num, goto 31
        33, // on Sym, goto 32
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
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        48, // on Node, goto 47
        0, // on Node*, error
        49, // on Node+, goto 48
        50, // on Num, goto 49
        51, // on Sym, goto 50
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
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        56, // on Node, goto 55
        0, // on Node*, error
        0, // on Node+, error
        32, // on Num, goto 31
        33, // on Sym, goto 32
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
        0, // on Atom, error
        0, // on Expr, error
        0, // on Lang, error
        0, // on List, error
        0, // on Node, error
        0, // on Node*, error
        0, // on Node+, error
        0, // on Num, error
        58, // on Sym, goto 57
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
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        30, // on Node, goto 29
        0, // on Node*, error
        59, // on Node+, goto 58
        32, // on Num, goto 31
        33, // on Sym, goto 32
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
        0, // on Sym, error
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
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        61, // on Node, goto 60
        0, // on Node*, error
        0, // on Node+, error
        50, // on Num, goto 49
        51, // on Sym, goto 50
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
        63, // on Sym, goto 62
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
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        30, // on Node, goto 29
        0, // on Node*, error
        64, // on Node+, goto 63
        32, // on Num, goto 31
        33, // on Sym, goto 32
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
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        48, // on Node, goto 47
        0, // on Node*, error
        66, // on Node+, goto 65
        50, // on Num, goto 49
        51, // on Sym, goto 50
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 58
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        56, // on Node, goto 55
        0, // on Node*, error
        0, // on Node+, error
        32, // on Num, goto 31
        33, // on Sym, goto 32
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
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        48, // on Node, goto 47
        0, // on Node*, error
        69, // on Node+, goto 68
        50, // on Num, goto 49
        51, // on Sym, goto 50
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 63
        27, // on Atom, goto 26
        28, // on Expr, goto 27
        0, // on Lang, error
        29, // on List, goto 28
        56, // on Node, goto 55
        0, // on Node*, error
        0, // on Node+, error
        32, // on Num, goto 31
        33, // on Sym, goto 32
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 64
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
        // State 65
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        61, // on Node, goto 60
        0, // on Node*, error
        0, // on Node+, error
        50, // on Num, goto 49
        51, // on Sym, goto 50
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 66
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
        // State 67
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
        // State 68
        45, // on Atom, goto 44
        46, // on Expr, goto 45
        0, // on Lang, error
        47, // on List, goto 46
        61, // on Node, goto 60
        0, // on Node*, error
        0, // on Node+, error
        50, // on Num, goto 49
        51, // on Sym, goto 50
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 69
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
        // State 70
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
        // State 71
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
        // State 72
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
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
            25 => {
                // __Lang = Lang => ActionFn(0);
                let __sym0 = __pop_NtLang(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            26 => {
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
            27 => {
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
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        3, // on "[", goto 2
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        18, // on "]", goto 17
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 3
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 4
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 5
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 6
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 7
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        23, // on "]", goto 22
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 8
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 9
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 10
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        0, // on ")", error
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "]", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 11
        25, // on "%", goto 24
        0, // on "(", error
        0, // on ")", error
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        0, // on "[", error
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        0, // on r#"-?[0-9]+"#, error
        // State 12
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 13
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 14
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 15
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 16
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        33, // on "]", goto 32
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 17
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 18
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        0, // on ")", error
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "]", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 19
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        0, // on ")", error
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "]", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 20
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "]", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 21
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 22
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 23
        25, // on "%", goto 24
        41, // on "(", goto 40
        42, // on ")", goto 41
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        43, // on "[", goto 42
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        44, // on r#"-?[0-9]+"#, goto 43
        // State 24
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        -20, // on ")", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        0, // on "]", error
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 25
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 26
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 27
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 28
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 29
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        -21, // on ")", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        0, // on "]", error
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 30
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on ")", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        0, // on "]", error
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 31
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        45, // on "]", goto 44
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 32
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 33
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 34
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 35
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 36
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        0, // on "]", error
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 37
        25, // on "%", goto 24
        41, // on "(", goto 40
        47, // on ")", goto 46
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        43, // on "[", goto 42
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        44, // on r#"-?[0-9]+"#, goto 43
        // State 38
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 39
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 40
        25, // on "%", goto 24
        0, // on "(", error
        0, // on ")", error
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        0, // on "[", error
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        0, // on r#"-?[0-9]+"#, error
        // State 41
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 42
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        50, // on "]", goto 49
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 43
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on ")", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on "]", error
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 44
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 45
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on "]", error
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 46
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        // State 47
        25, // on "%", goto 24
        41, // on "(", goto 40
        52, // on ")", goto 51
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        43, // on "[", goto 42
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        44, // on r#"-?[0-9]+"#, goto 43
        // State 48
        11, // on "%", goto 10
        12, // on "(", goto 11
        0, // on ")", error
        13, // on "*", goto 12
        14, // on "+", goto 13
        15, // on "-", goto 14
        16, // on "/", goto 15
        17, // on "[", goto 16
        53, // on "]", goto 52
        19, // on "head", goto 18
        20, // on "tail", goto 19
        21, // on r#"-?[0-9]+"#, goto 20
        // State 49
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        0, // on "]", error
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 50
        25, // on "%", goto 24
        41, // on "(", goto 40
        54, // on ")", goto 53
        26, // on "*", goto 25
        27, // on "+", goto 26
        28, // on "-", goto 27
        29, // on "/", goto 28
        43, // on "[", goto 42
        0, // on "]", error
        30, // on "head", goto 29
        31, // on "tail", goto 30
        44, // on r#"-?[0-9]+"#, goto 43
        // State 51
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on "]", error
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 52
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on "]", error
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 53
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on "]", error
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -26, // on EOF, reduce `__List = List => ActionFn(2);`
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
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(29);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(30);`
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
        22, // on Node, goto 21
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
        24, // on Sym, goto 23
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
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        32, // on Node+, goto 31
        9, // on Num, goto 8
        10, // on Sym, goto 9
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
        34, // on Atom, goto 33
        35, // on Expr, goto 34
        0, // on Lang, error
        36, // on List, goto 35
        37, // on Node, goto 36
        0, // on Node*, error
        38, // on Node+, goto 37
        39, // on Num, goto 38
        40, // on Sym, goto 39
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
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        22, // on Node, goto 21
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
        34, // on Atom, goto 33
        35, // on Expr, goto 34
        0, // on Lang, error
        36, // on List, goto 35
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        39, // on Num, goto 38
        40, // on Sym, goto 39
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
        48, // on Sym, goto 47
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
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        7, // on Node, goto 6
        0, // on Node*, error
        49, // on Node+, goto 48
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
        34, // on Atom, goto 33
        35, // on Expr, goto 34
        0, // on Lang, error
        36, // on List, goto 35
        37, // on Node, goto 36
        0, // on Node*, error
        51, // on Node+, goto 50
        39, // on Num, goto 38
        40, // on Sym, goto 39
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 48
        4, // on Atom, goto 3
        5, // on Expr, goto 4
        0, // on Lang, error
        6, // on List, goto 5
        22, // on Node, goto 21
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
        34, // on Atom, goto 33
        35, // on Expr, goto 34
        0, // on Lang, error
        36, // on List, goto 35
        46, // on Node, goto 45
        0, // on Node*, error
        0, // on Node+, error
        39, // on Num, goto 38
        40, // on Sym, goto 39
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
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
            25 => {
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
            26 => {
                // __List = List => ActionFn(2);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            27 => {
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
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        8, // on "%", goto 7
        9, // on "(", goto 8
        0, // on ")", error
        10, // on "*", goto 9
        11, // on "+", goto 10
        12, // on "-", goto 11
        13, // on "/", goto 12
        14, // on "[", goto 13
        0, // on "]", error
        15, // on "head", goto 14
        16, // on "tail", goto 15
        17, // on r#"-?[0-9]+"#, goto 16
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 6
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 7
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 8
        19, // on "%", goto 18
        0, // on "(", error
        0, // on ")", error
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        0, // on "[", error
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        0, // on r#"-?[0-9]+"#, error
        // State 9
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 10
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 11
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 12
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 13
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        40, // on "]", goto 39
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 14
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 15
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 16
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 17
        19, // on "%", goto 18
        51, // on "(", goto 50
        52, // on ")", goto 51
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 18
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        -20, // on ")", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        0, // on "]", error
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 19
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        -18, // on ")", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        0, // on "]", error
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 20
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        -16, // on ")", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        0, // on "]", error
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 21
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        -17, // on ")", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        0, // on "]", error
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 22
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        -19, // on ")", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        0, // on "]", error
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 23
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        -21, // on ")", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        0, // on "]", error
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 24
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on ")", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        0, // on "]", error
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 25
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        0, // on ")", error
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        -8, // on "]", reduce `Node = Atom => ActionFn(10);`
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 26
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        0, // on ")", error
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        -10, // on "]", reduce `Node = Expr => ActionFn(12);`
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 27
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        0, // on ")", error
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        -9, // on "]", reduce `Node = List => ActionFn(11);`
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 28
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        0, // on ")", error
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "]", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 29
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        56, // on "]", goto 55
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 30
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        0, // on ")", error
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        -2, // on "]", reduce `Atom = Num => ActionFn(14);`
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 31
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        0, // on ")", error
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "]", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 32
        -20, // on "%", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "(", reduce `Sym = "%" => ActionFn(19);`
        0, // on ")", error
        -20, // on "*", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "+", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "-", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "/", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "[", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "]", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "head", reduce `Sym = "%" => ActionFn(19);`
        -20, // on "tail", reduce `Sym = "%" => ActionFn(19);`
        -20, // on r#"-?[0-9]+"#, reduce `Sym = "%" => ActionFn(19);`
        // State 33
        19, // on "%", goto 18
        0, // on "(", error
        0, // on ")", error
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        0, // on "[", error
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        0, // on r#"-?[0-9]+"#, error
        // State 34
        -18, // on "%", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "(", reduce `Sym = "*" => ActionFn(17);`
        0, // on ")", error
        -18, // on "*", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "+", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "-", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "/", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "[", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "]", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "head", reduce `Sym = "*" => ActionFn(17);`
        -18, // on "tail", reduce `Sym = "*" => ActionFn(17);`
        -18, // on r#"-?[0-9]+"#, reduce `Sym = "*" => ActionFn(17);`
        // State 35
        -16, // on "%", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "(", reduce `Sym = "+" => ActionFn(15);`
        0, // on ")", error
        -16, // on "*", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "+", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "-", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "/", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "[", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "]", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "head", reduce `Sym = "+" => ActionFn(15);`
        -16, // on "tail", reduce `Sym = "+" => ActionFn(15);`
        -16, // on r#"-?[0-9]+"#, reduce `Sym = "+" => ActionFn(15);`
        // State 36
        -17, // on "%", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "(", reduce `Sym = "-" => ActionFn(16);`
        0, // on ")", error
        -17, // on "*", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "+", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "-", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "/", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "[", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "]", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "head", reduce `Sym = "-" => ActionFn(16);`
        -17, // on "tail", reduce `Sym = "-" => ActionFn(16);`
        -17, // on r#"-?[0-9]+"#, reduce `Sym = "-" => ActionFn(16);`
        // State 37
        -19, // on "%", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "(", reduce `Sym = "/" => ActionFn(18);`
        0, // on ")", error
        -19, // on "*", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "+", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "-", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "/", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "[", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "]", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "head", reduce `Sym = "/" => ActionFn(18);`
        -19, // on "tail", reduce `Sym = "/" => ActionFn(18);`
        -19, // on r#"-?[0-9]+"#, reduce `Sym = "/" => ActionFn(18);`
        // State 38
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        59, // on "]", goto 58
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 39
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 40
        -21, // on "%", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "(", reduce `Sym = "head" => ActionFn(20);`
        0, // on ")", error
        -21, // on "*", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "+", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "-", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "/", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "[", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "]", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "head", reduce `Sym = "head" => ActionFn(20);`
        -21, // on "tail", reduce `Sym = "head" => ActionFn(20);`
        -21, // on r#"-?[0-9]+"#, reduce `Sym = "head" => ActionFn(20);`
        // State 41
        -22, // on "%", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "(", reduce `Sym = "tail" => ActionFn(21);`
        0, // on ")", error
        -22, // on "*", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "+", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "-", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "/", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "[", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "]", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "head", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on "tail", reduce `Sym = "tail" => ActionFn(21);`
        -22, // on r#"-?[0-9]+"#, reduce `Sym = "tail" => ActionFn(21);`
        // State 42
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on ")", error
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "]", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 43
        -8, // on "%", reduce `Node = Atom => ActionFn(10);`
        -8, // on "(", reduce `Node = Atom => ActionFn(10);`
        -8, // on ")", reduce `Node = Atom => ActionFn(10);`
        -8, // on "*", reduce `Node = Atom => ActionFn(10);`
        -8, // on "+", reduce `Node = Atom => ActionFn(10);`
        -8, // on "-", reduce `Node = Atom => ActionFn(10);`
        -8, // on "/", reduce `Node = Atom => ActionFn(10);`
        -8, // on "[", reduce `Node = Atom => ActionFn(10);`
        0, // on "]", error
        -8, // on "head", reduce `Node = Atom => ActionFn(10);`
        -8, // on "tail", reduce `Node = Atom => ActionFn(10);`
        -8, // on r#"-?[0-9]+"#, reduce `Node = Atom => ActionFn(10);`
        // State 44
        -10, // on "%", reduce `Node = Expr => ActionFn(12);`
        -10, // on "(", reduce `Node = Expr => ActionFn(12);`
        -10, // on ")", reduce `Node = Expr => ActionFn(12);`
        -10, // on "*", reduce `Node = Expr => ActionFn(12);`
        -10, // on "+", reduce `Node = Expr => ActionFn(12);`
        -10, // on "-", reduce `Node = Expr => ActionFn(12);`
        -10, // on "/", reduce `Node = Expr => ActionFn(12);`
        -10, // on "[", reduce `Node = Expr => ActionFn(12);`
        0, // on "]", error
        -10, // on "head", reduce `Node = Expr => ActionFn(12);`
        -10, // on "tail", reduce `Node = Expr => ActionFn(12);`
        -10, // on r#"-?[0-9]+"#, reduce `Node = Expr => ActionFn(12);`
        // State 45
        -9, // on "%", reduce `Node = List => ActionFn(11);`
        -9, // on "(", reduce `Node = List => ActionFn(11);`
        -9, // on ")", reduce `Node = List => ActionFn(11);`
        -9, // on "*", reduce `Node = List => ActionFn(11);`
        -9, // on "+", reduce `Node = List => ActionFn(11);`
        -9, // on "-", reduce `Node = List => ActionFn(11);`
        -9, // on "/", reduce `Node = List => ActionFn(11);`
        -9, // on "[", reduce `Node = List => ActionFn(11);`
        0, // on "]", error
        -9, // on "head", reduce `Node = List => ActionFn(11);`
        -9, // on "tail", reduce `Node = List => ActionFn(11);`
        -9, // on r#"-?[0-9]+"#, reduce `Node = List => ActionFn(11);`
        // State 46
        -13, // on "%", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "(", reduce `Node+ = Node => ActionFn(25);`
        -13, // on ")", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "*", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "+", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "-", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "/", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "[", reduce `Node+ = Node => ActionFn(25);`
        0, // on "]", error
        -13, // on "head", reduce `Node+ = Node => ActionFn(25);`
        -13, // on "tail", reduce `Node+ = Node => ActionFn(25);`
        -13, // on r#"-?[0-9]+"#, reduce `Node+ = Node => ActionFn(25);`
        // State 47
        19, // on "%", goto 18
        51, // on "(", goto 50
        61, // on ")", goto 60
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 48
        -2, // on "%", reduce `Atom = Num => ActionFn(14);`
        -2, // on "(", reduce `Atom = Num => ActionFn(14);`
        -2, // on ")", reduce `Atom = Num => ActionFn(14);`
        -2, // on "*", reduce `Atom = Num => ActionFn(14);`
        -2, // on "+", reduce `Atom = Num => ActionFn(14);`
        -2, // on "-", reduce `Atom = Num => ActionFn(14);`
        -2, // on "/", reduce `Atom = Num => ActionFn(14);`
        -2, // on "[", reduce `Atom = Num => ActionFn(14);`
        0, // on "]", error
        -2, // on "head", reduce `Atom = Num => ActionFn(14);`
        -2, // on "tail", reduce `Atom = Num => ActionFn(14);`
        -2, // on r#"-?[0-9]+"#, reduce `Atom = Num => ActionFn(14);`
        // State 49
        -1, // on "%", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "(", reduce `Atom = Sym => ActionFn(13);`
        -1, // on ")", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "*", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "+", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "-", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "/", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "[", reduce `Atom = Sym => ActionFn(13);`
        0, // on "]", error
        -1, // on "head", reduce `Atom = Sym => ActionFn(13);`
        -1, // on "tail", reduce `Atom = Sym => ActionFn(13);`
        -1, // on r#"-?[0-9]+"#, reduce `Atom = Sym => ActionFn(13);`
        // State 50
        19, // on "%", goto 18
        0, // on "(", error
        0, // on ")", error
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        0, // on "[", error
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        0, // on r#"-?[0-9]+"#, error
        // State 51
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 52
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        64, // on "]", goto 63
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 53
        -15, // on "%", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "(", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on ")", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "*", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "+", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "-", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "/", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "[", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        0, // on "]", error
        -15, // on "head", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on "tail", reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        -15, // on r#"-?[0-9]+"#, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
        // State 54
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on ")", error
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "]", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 55
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 56
        19, // on "%", goto 18
        51, // on "(", goto 50
        66, // on ")", goto 65
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 57
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        67, // on "]", goto 66
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 58
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        0, // on ")", error
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "]", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 59
        -14, // on "%", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "(", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on ")", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "*", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "+", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "-", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "/", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "[", reduce `Node+ = Node+, Node => ActionFn(26);`
        0, // on "]", error
        -14, // on "head", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on "tail", reduce `Node+ = Node+, Node => ActionFn(26);`
        -14, // on r#"-?[0-9]+"#, reduce `Node+ = Node+, Node => ActionFn(26);`
        // State 60
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 61
        19, // on "%", goto 18
        51, // on "(", goto 50
        69, // on ")", goto 68
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 62
        33, // on "%", goto 32
        34, // on "(", goto 33
        0, // on ")", error
        35, // on "*", goto 34
        36, // on "+", goto 35
        37, // on "-", goto 36
        38, // on "/", goto 37
        39, // on "[", goto 38
        70, // on "]", goto 69
        41, // on "head", goto 40
        42, // on "tail", goto 41
        43, // on r#"-?[0-9]+"#, goto 42
        // State 63
        -6, // on "%", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "(", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on ")", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "*", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "+", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "-", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "/", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "[", reduce `List = "[", "]" => ActionFn(29);`
        0, // on "]", error
        -6, // on "head", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on "tail", reduce `List = "[", "]" => ActionFn(29);`
        -6, // on r#"-?[0-9]+"#, reduce `List = "[", "]" => ActionFn(29);`
        // State 64
        19, // on "%", goto 18
        51, // on "(", goto 50
        71, // on ")", goto 70
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 65
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on ")", error
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "]", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 66
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on ")", error
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "]", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 67
        19, // on "%", goto 18
        51, // on "(", goto 50
        72, // on ")", goto 71
        20, // on "*", goto 19
        21, // on "+", goto 20
        22, // on "-", goto 21
        23, // on "/", goto 22
        53, // on "[", goto 52
        0, // on "]", error
        24, // on "head", goto 23
        25, // on "tail", goto 24
        54, // on r#"-?[0-9]+"#, goto 53
        // State 68
        -3, // on "%", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "(", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on ")", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "*", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "+", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "-", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "/", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "[", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on "]", error
        -3, // on "head", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on "tail", reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        -3, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        // State 69
        -7, // on "%", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "(", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on ")", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "*", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "+", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "-", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "/", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "[", reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on "]", error
        -7, // on "head", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on "tail", reduce `List = "[", Node+, "]" => ActionFn(30);`
        -7, // on r#"-?[0-9]+"#, reduce `List = "[", Node+, "]" => ActionFn(30);`
        // State 70
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on ")", error
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "]", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        // State 71
        -4, // on "%", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "(", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on ")", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "*", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "+", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "-", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "/", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "[", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        0, // on "]", error
        -4, // on "head", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on "tail", reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
        -4, // on r#"-?[0-9]+"#, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -8, // on EOF, reduce `Node = Atom => ActionFn(10);`
        -10, // on EOF, reduce `Node = Expr => ActionFn(12);`
        -9, // on EOF, reduce `Node = List => ActionFn(11);`
        -27, // on EOF, reduce `__Node = Node => ActionFn(3);`
        -2, // on EOF, reduce `Atom = Num => ActionFn(14);`
        -1, // on EOF, reduce `Atom = Sym => ActionFn(13);`
        -20, // on EOF, reduce `Sym = "%" => ActionFn(19);`
        0, // on EOF, error
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        0, // on EOF, error
        -21, // on EOF, reduce `Sym = "head" => ActionFn(20);`
        -22, // on EOF, reduce `Sym = "tail" => ActionFn(21);`
        -15, // on EOF, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
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
        -6, // on EOF, reduce `List = "[", "]" => ActionFn(29);`
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
        -3, // on EOF, reduce `Expr = "(", Sym, ")" => ActionFn(27);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `List = "[", Node+, "]" => ActionFn(30);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = "(", Sym, Node+, ")" => ActionFn(28);`
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
        18, // on Sym, goto 17
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        29, // on Node, goto 28
        0, // on Node*, error
        30, // on Node+, goto 29
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        47, // on Node, goto 46
        0, // on Node*, error
        48, // on Node+, goto 47
        49, // on Num, goto 48
        50, // on Sym, goto 49
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        55, // on Node, goto 54
        0, // on Node*, error
        0, // on Node+, error
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        57, // on Sym, goto 56
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        29, // on Node, goto 28
        0, // on Node*, error
        58, // on Node+, goto 57
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        0, // on Sym, error
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
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        60, // on Node, goto 59
        0, // on Node*, error
        0, // on Node+, error
        49, // on Num, goto 48
        50, // on Sym, goto 49
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
        62, // on Sym, goto 61
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
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        29, // on Node, goto 28
        0, // on Node*, error
        63, // on Node+, goto 62
        31, // on Num, goto 30
        32, // on Sym, goto 31
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 53
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
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        47, // on Node, goto 46
        0, // on Node*, error
        65, // on Node+, goto 64
        49, // on Num, goto 48
        50, // on Sym, goto 49
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 57
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        55, // on Node, goto 54
        0, // on Node*, error
        0, // on Node+, error
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        47, // on Node, goto 46
        0, // on Node*, error
        68, // on Node+, goto 67
        49, // on Num, goto 48
        50, // on Sym, goto 49
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 62
        26, // on Atom, goto 25
        27, // on Expr, goto 26
        0, // on Lang, error
        28, // on List, goto 27
        55, // on Node, goto 54
        0, // on Node*, error
        0, // on Node+, error
        31, // on Num, goto 30
        32, // on Sym, goto 31
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
        // State 64
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        60, // on Node, goto 59
        0, // on Node*, error
        0, // on Node+, error
        49, // on Num, goto 48
        50, // on Sym, goto 49
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 65
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
        // State 66
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
        // State 67
        44, // on Atom, goto 43
        45, // on Expr, goto 44
        0, // on Lang, error
        46, // on List, goto 45
        60, // on Node, goto 59
        0, // on Node*, error
        0, // on Node+, error
        49, // on Num, goto 48
        50, // on Sym, goto 49
        0, // on __Atom, error
        0, // on __Expr, error
        0, // on __Lang, error
        0, // on __List, error
        0, // on __Node, error
        0, // on __Num, error
        0, // on __Sym, error
        // State 68
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
        // State 69
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
        // State 70
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
        // State 71
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
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
            25 => {
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
            26 => {
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
            27 => {
                // __Node = Node => ActionFn(3);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        3, // on r#"-?[0-9]+"#, goto 2
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -28, // on EOF, reduce `__Num = Num => ActionFn(6);`
        -15, // on EOF, reduce `Num = r#"-?[0-9]+"# => ActionFn(22);`
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
    ) -> Result<i32, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
    ) -> Option<Result<i32,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
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
            25 => {
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
            26 => {
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
            27 => {
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
            28 => {
                // __Num = Num => ActionFn(6);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                return Some(Ok(__nt));
            }
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
        Term_22_25_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22head_22(&'input str),
        Term_22tail_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtAtom(Atom),
        NtExpr(Expr),
        NtLang(Node),
        NtList(List),
        NtNode(Node),
        NtNode_2a(::std::vec::Vec<Node>),
        NtNode_2b(::std::vec::Vec<Node>),
        NtNum(i32),
        NtSym(Symbol),
        Nt____Atom(Atom),
        Nt____Expr(Expr),
        Nt____Lang(Node),
        Nt____List(List),
        Nt____Node(Node),
        Nt____Num(i32),
        Nt____Sym(Symbol),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, // on "%", goto 2
        0, // on "(", error
        0, // on ")", error
        4, // on "*", goto 3
        5, // on "+", goto 4
        6, // on "-", goto 5
        7, // on "/", goto 6
        0, // on "[", error
        0, // on "]", error
        8, // on "head", goto 7
        9, // on "tail", goto 8
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 6
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 7
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
        // State 8
        0, // on "%", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "[", error
        0, // on "]", error
        0, // on "head", error
        0, // on "tail", error
        0, // on r#"-?[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -29, // on EOF, reduce `__Sym = Sym => ActionFn(5);`
        -20, // on EOF, reduce `Sym = "%" => ActionFn(19);`
        -18, // on EOF, reduce `Sym = "*" => ActionFn(17);`
        -16, // on EOF, reduce `Sym = "+" => ActionFn(15);`
        -17, // on EOF, reduce `Sym = "-" => ActionFn(16);`
        -19, // on EOF, reduce `Sym = "/" => ActionFn(18);`
        -21, // on EOF, reduce `Sym = "head" => ActionFn(20);`
        -22, // on EOF, reduce `Sym = "tail" => ActionFn(21);`
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
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_25_22(__tok0),
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
                            (7, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22head_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22tail_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
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
                // Expr = "(", Sym, ")" => ActionFn(27);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = "(", Sym, Node+, ")" => ActionFn(28);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtNode_2b(__symbols);
                let __sym1 = __pop_NtSym(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action28(input, __sym0, __sym1, __sym2, __sym3);
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
                // List = "[", "]" => ActionFn(29);
                let __sym1 = __pop_Term_22_5d_22(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                3
            }
            7 => {
                // List = "[", Node+, "]" => ActionFn(30);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtNode_2b(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1, __sym2);
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
                // Node* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            12 => {
                // Node* = Node+ => ActionFn(24);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2a(__nt), __end));
                5
            }
            13 => {
                // Node+ = Node => ActionFn(25);
                let __sym0 = __pop_NtNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            14 => {
                // Node+ = Node+, Node => ActionFn(26);
                let __sym1 = __pop_NtNode(__symbols);
                let __sym0 = __pop_NtNode_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNode_2b(__nt), __end));
                6
            }
            15 => {
                // Num = r#"-?[0-9]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
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
                // Sym = "%" => ActionFn(19);
                let __sym0 = __pop_Term_22_25_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            21 => {
                // Sym = "head" => ActionFn(20);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            22 => {
                // Sym = "tail" => ActionFn(21);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSym(__nt), __end));
                8
            }
            23 => {
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
            24 => {
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
            25 => {
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
            26 => {
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
            27 => {
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
            28 => {
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
            29 => {
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
    fn __pop_Term_22_25_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_25_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, i32, usize) {
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
    ) -> (usize, i32, usize) {
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
                        37 => /* '%' */ {
                            __current_match = Some((0, __index + 1));
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
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 11;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 12;
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
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
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
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        101 => /* 'e' */ {
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        97 => /* 'a' */ {
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        97 => /* 'a' */ {
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        100 => /* 'd' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        108 => /* 'l' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
    (_, __0, _): (usize, i32, usize),
) -> i32
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
    (_, __0, _): (usize, i32, usize),
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
) -> Symbol
{
    Symbol::Mod
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Head
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Symbol
{
    Symbol::Tail
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action23<
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
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Node>, usize),
) -> ::std::vec::Vec<Node>
{
    v
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> ::std::vec::Vec<Node>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action26<
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
pub fn __action27<
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
    let __temp0 = __action23(
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
pub fn __action28<
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
    let __temp0 = __action24(
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
pub fn __action29<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> List
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action23(
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
pub fn __action30<
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
    let __temp0 = __action24(
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

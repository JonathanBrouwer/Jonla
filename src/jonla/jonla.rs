use std::hash::Hash;
use std::rc::Rc;
use crate::lambday::lambday::LambdayTerm;
use crate::{Parser, ParseSuccess};
use crate::peg::input::Input;
use crate::peg::parsers::alt::alt;
use crate::peg::parsers::complete_input::complete_input;
use crate::peg::parsers::exact::exact;
use crate::peg::parsers::ignore_whitespace::ignore_whitespace;
use crate::peg::parsers::repeat_m_n::{repeat_m_n, repeat_m_n_matching};
use crate::peg::parsers::seq::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum JonlaTerm<Sym: Eq + Hash + Clone> {
    LambdayTerm(LambdayTerm<Sym>)
}

pub fn parse_jonla_program<I: Input<InputElement=char>>() -> impl Parser<I, Vec<JonlaTerm<String>>> {
    complete_input(repeat_m_n(parse_jonla_term(), 0, usize::MAX))
}

pub fn parse_jonla_term<I: Input<InputElement=char>>() -> impl Parser<I, JonlaTerm<String>> {
    move |pos: I| {
        let ok = parse_lamday_term().parse(pos)?;
        Ok(ParseSuccess { result: JonlaTerm::LambdayTerm(ok.result), best_error: ok.best_error, pos: ok.pos })
    }
}

pub fn parse_lamday_term<I: Input<InputElement=char>>() -> impl Parser<I, LambdayTerm<String>> {
    move |pos: I| {
        let parsers: Vec<Box<dyn Parser<I, LambdayTerm<String>>>> = vec![
            //Fun type
            Box::new(|pos: I| {
                let ok = seq6(
                    ignore_whitespace(exact("#ft".chars().collect())),
                    ignore_whitespace(exact("(".chars().collect())),
                    ignore_whitespace(parse_lamday_term()),
                    ignore_whitespace(exact("->".chars().collect())),
                    ignore_whitespace(parse_lamday_term()),
                    ignore_whitespace(exact(")".chars().collect()))
                ).parse(pos)?;
                let (_, _, t1, _, t2, _) = ok.result;

                Ok(ParseSuccess {
                    result: LambdayTerm::FunType(Rc::new(t1), Rc::new(t2)),
                    best_error: ok.best_error,
                    pos: ok.pos
                })
            }),
            //Fun construct
            Box::new(|pos: I| {
                let ok = seq9(
                    ignore_whitespace(exact("#fc".chars().collect())),
                    ignore_whitespace(exact("(".chars().collect())),
                    ignore_whitespace(parse_name()),
                    ignore_whitespace(exact(":".chars().collect())),
                    ignore_whitespace(parse_lamday_term()),
                    ignore_whitespace(exact(")".chars().collect())),
                    ignore_whitespace(exact("(".chars().collect())),
                    ignore_whitespace(parse_lamday_term()),
                    ignore_whitespace(exact(")".chars().collect()))
                ).parse(pos)?;
                let (_, _, t1, _, t2, _, _, t3, _) = ok.result;

                Ok(ParseSuccess {
                    result: LambdayTerm::FunConstr(t1, Rc::new(t2), Rc::new(t3)),
                    best_error: ok.best_error,
                    pos: ok.pos
                })
            }),
            //Type type
            Box::new(|input: I| {
                let ok = ignore_whitespace(exact("#tt".chars().collect())).parse(input)?;
                Ok(ParseSuccess {
                    result: LambdayTerm::TypeType(),
                    best_error: ok.best_error,
                    pos: ok.pos
                })
            }),
            //Var
            Box::new(|input: I| {
                ignore_whitespace(parse_name()).parse(input)
                    .map(|s| s.map(|r| LambdayTerm::Var(r)))
            }),
        ];
        alt(parsers).parse(pos.clone())
    }
}

pub fn parse_name<I: Input<InputElement=char>>() -> impl Parser<I, String> {
    move |pos: I| {
        repeat_m_n_matching(
            "name".to_string(),
            Box::new(|c: char| c.is_alphabetic()),
            1, usize::MAX).parse(pos)
            .map(|s| ParseSuccess { result: s.result.into_iter().collect(), best_error: s.best_error, pos: s.pos })
    }
}

// impl<'a, I: Input<InputElement=char>> Parser<I, LambdayTerm<&'a str>> for JonlaLambdayParser {
//     fn parse(&self, input: I) -> Result<ParseSuccess<I, LambdayTerm<&'a str>>, ParseError<I>> {
//         Choice { parsers: vec![
//             Box::new(LambdayVarParser {}),
//         ] }.parse(input)
//     }
// }
//
// struct LambdayVarParser {}
// impl<'a, I: Input<InputElement=char>> Parser<I, LambdayTerm<&'a str>> for LambdayVarParser {
//     fn parse(&self, input: I) -> Result<ParseSuccess<I, LambdayTerm<&'a str>>, ParseError<I>> {
//
//     }
// }

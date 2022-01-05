#![feature(split_array)]
#![feature(box_syntax)]

pub mod peg;
mod lambday;
mod jonla;

use crate::jonla::jonla::parse_jonla_program;
use crate::peg::input::Input;
// use crate::jonla::jonla::{parse_jonla_program};
use crate::peg::parser::*;
use crate::peg::parser_success::*;

fn main() {
    let input = include_str!("../resources/example.jnl");
    let parsed = match parse_jonla_program().parse(Input { src: input, pos: 0 }) {
        Ok(ok) => ok,
        Err(err) => {
            err.0.print(input.to_string());
            return
        }
    }.result;
    let name_checked = match parsed.transform_names() {
        Ok(ok) => ok,
        Err(err) => {
            err.print(input.to_string());
            return
        }
    };
    let type_checked = match name_checked.0.type_check() {
        Ok(ok) => ok,
        Err(err) => {
            err.print(input.to_string());
            return
        }
    };
    println!("{:?}", type_checked)



}


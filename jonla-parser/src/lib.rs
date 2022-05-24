#![feature(box_syntax)]
#![feature(let_chains)]
#![allow(clippy::needless_lifetimes)]

pub mod character_class;
pub mod input;
pub mod parse_error;
pub mod parse_pair;
pub mod parse_result;
pub mod parser_core;
pub mod parser_core_ast;
pub mod parser_core_expression;
pub mod parser_core_file;
pub mod span;

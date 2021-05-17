use crate::parser::parser::{JonlaParser, compose};
use crate::lexer::lexer::LexerTokenType;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::parser::parser_stmt_datadefinition::DataDefinition;
use crate::parser::parser_expr::Expression;

#[derive(Debug)]
pub enum Statement<'a> {
    DataDefinition(DataDefinition<'a>),
    Expression(Expression<'a>)
}
impl<'a> JonlaParser<'a> {
    pub fn parse_stmt(&mut self) -> Result<Statement<'a>, String> {
        self.or(vec![
            |p| Self::parse_stmt_data(p).map(Statement::DataDefinition),
            |p| Self::parse_expr(p).map(Statement::Expression)
        ])
    }
}



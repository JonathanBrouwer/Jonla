use logos::{Lexer, Logos, Source};

use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogosToken {
    #[regex(r"[[\p{Letter}\p{Mark}\p{Symbol}\p{Number}\p{Dash_Punctuation}\p{Connector_Punctuation}\p{Other_Punctuation}]+\p{Open_Punctuation}\p{Close_Punctuation}]")]
    Name,

    #[token("\n")]
    Line,

    #[error]
    #[regex(r"[\p{Separator}\r]+", logos::skip)]
    Error,
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LexerToken<'a> {
    pub span: Range<usize>,
    pub token: &'a str
}

pub struct LexerLine<'a> {
    pub tokens: Vec<LexerToken<'a>>,
    pub indent: usize,
}

pub struct FinalLexer<'a> {
    pub lexer1: Lexer<'a, LogosToken>,
    pub errors: Vec<Range<usize>>,
}

impl<'a> FinalLexer<'a> {
    pub fn new(source: &'a str) -> FinalLexer {
        FinalLexer { lexer1: LogosToken::lexer(source), errors: Vec::new() }
    }
}

impl<'a> Iterator for FinalLexer<'a> {
    type Item = LexerLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let indent: usize = self.lexer1.remainder().chars().take_while(|&c| c == ' ').count();
        let mut tokens: Vec<LexerToken<'a>> = Vec::new();
        loop {
            match self.lexer1.next() {
                None => if tokens.len() == 0 {return None} else {break},
                Some(LogosToken::Line) => break,
                Some(LogosToken::Name) => {
                    tokens.push(LexerToken { span: self.lexer1.span(), token: self.lexer1.slice() })
                }
                Some(LogosToken::Error) => {
                    self.errors.push(self.lexer1.span());
                }
            }
        }
        Some(LexerLine {indent, tokens})
    }
}
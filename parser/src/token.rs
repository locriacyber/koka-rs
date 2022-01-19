use crate::{Special, Keyword};

pub type TokenSpan = u16;

#[derive(Debug, PartialEq)]
pub struct TokenSourceInfo {
    /// row of token, zero indexed
    row: u16, // should be enough. if your program size is larger than this, you have a problem
    /// column of start of token in source code, zero indexed
    col: u16,
    /// length of token
    span: TokenSpan,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Whitespace, // line comment, block comment (consume newline), linedirective, space and tab
    Newline,
    Special(Special),
    Keyword(Keyword),
    LiteralDecimal(f64),
    LiteralInteger(isize),        // maybe use big int?
    LiteralByteSequence(Vec<u8>), // parsed, not including the quotes
    Identifier(&'a [u8]),
    Operator(&'a [u8]),
}

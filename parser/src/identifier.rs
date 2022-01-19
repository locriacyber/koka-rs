use crate::util::concat_nearby;
use crate::Keyword;
use crate::Token;

use std::str::FromStr;

use nom::bytes::complete::take_while_m_n;
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

pub fn identifier_or_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (remain, token_raw) = identifier(input)?;
    let token = match Keyword::from_str(&String::from_utf8_lossy(&token_raw)) {
        Ok(kw) => Token::Keyword(kw),
        Err(_) => Token::Identifier(token_raw),
    };
    Ok((remain, token))
}

fn identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((
            take_while_m_n(1, 1, is_preceding),
            take_while_m_n(0, usize::MAX, is_after),
        )),
        |(prec, after)| concat_nearby(prec, after).expect("two slices should be adjacent but they are not"),
    )(input)
}

fn is_preceding(c: u8) -> bool {
    is_alphabetic(c) || is_underscore(c)
}

fn is_underscore(c: u8) -> bool {
    c == b'_'
}

fn is_after(c: u8) -> bool {
    is_alphanumeric(c) || is_underscore(c)
}

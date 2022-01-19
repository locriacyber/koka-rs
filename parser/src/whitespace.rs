use crate::Token;
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;

pub fn whitespace(input: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(is_whitespace), |_| Token::Whitespace)(input)
}

fn is_whitespace(c: u8) -> bool {
    c == b' ' // todo add more
}

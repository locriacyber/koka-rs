mod keyword;
mod identifier;
mod token;
mod whitespace;
mod util;

#[cfg(test)]
mod tests;

pub use keyword::*;
pub use identifier::*;
pub use token::*;
pub use whitespace::*;

use anyhow::{Context, Error};
use nom::{branch::alt, IResult};

pub fn parse_one(input: &[u8]) -> IResult<&[u8], token::Token> {
    alt((identifier_or_keyword, whitespace))(input)
}

pub fn parse_all(input: &[u8]) -> Result<Vec<(token::Token, token::TokenSpan)>, Error> {
    let mut output = vec![];
    let mut remaining: &[u8] = input;
    while !remaining.is_empty() {
        let (remaining_new, token) = parse_one(remaining).map_err(|e| e.to_owned()).with_context(|| "Invalid character")?;
        let span = remaining.len() - remaining_new.len();
        let span: token::TokenSpan = span
            .try_into()
            .with_context(|| "Token longer than u16, what are you doing?")?;
        output.push((token, span));
        remaining = remaining_new;
    }
    Ok(output)
}

use crate::{parse_all, Keyword, Special, token::Token};

macro_rules! assert_eq_parse {
    ($input: expr, $expected_seq: expr) => {
        let result: Vec<Token> = parse_all($input)
            .unwrap()
            .into_iter()
            .map(|x| x.0)
            .collect();
        assert_eq!(result, $expected_seq);
    };
}

macro_rules! assert_eq_parse_no_white {
    ($input: expr, $expected_seq: expr) => {
        let result: Vec<Token> = parse_all($input)
            .unwrap()
            .into_iter()
            .map(|x| x.0)
            .collect();
        assert_eq!(result, $expected_seq);
    };
}

#[test]
fn empty() {
    assert_eq_parse!(b"", vec![]);
}

#[test]
fn space() {
    assert_eq_parse!(b" ", vec![Token::Whitespace]);
}

#[test]
fn simple1() {
    assert_eq_parse_no_white!(
        b"fun a() { }",
        vec![
            Token::Keyword(Keyword::Fun),
            Token::Identifier(b"a"),
            Token::Special(Special::LRound),
            Token::Special(Special::RRound),
            Token::Special(Special::LCurly),
            Token::Special(Special::RCurly),
        ]
    );
}

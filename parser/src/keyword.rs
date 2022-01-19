
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Display, EnumString)]
pub enum Special {
    #[strum(serialize = "=")]
    Assign,
    #[strum(serialize = ".")]
    Period,
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = "->")]
    RArrow,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "|")]
    Bar,
    #[strum(serialize = "{")]
    LCurly,
    #[strum(serialize = "}")]
    RCurly,
    #[strum(serialize = "(")]
    LRound,
    #[strum(serialize = ")")]
    RRound,
    #[strum(serialize = "[")]
    LSquare,
    #[strum(serialize = "]")]
    RSquare,
    #[strum(serialize = "<")]
    LAngle,
    #[strum(serialize = ">")]
    RAngle,
}

#[derive(Debug, PartialEq, Display, EnumString)]
pub enum Keyword {
    #[strum(serialize = "infix")]
    Infix,
    #[strum(serialize = "infixr")]
    Infixr,
    #[strum(serialize = "infixl")]
    Infixl,
    #[strum(serialize = "module")]
    Module,
    #[strum(serialize = "import")]
    Import,
    #[strum(serialize = "as")]
    As,
    #[strum(serialize = "pub")]
    Pub,
    #[strum(serialize = "abstract")]
    Abstract,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "struct")]
    Struct,
    #[strum(serialize = "alias")]
    Alias,
    #[strum(serialize = "effect")]
    Effect,
    #[strum(serialize = "con")]
    Con,
    #[strum(serialize = "forall")]
    Forall,
    #[strum(serialize = "exists")]
    Exists,
    #[strum(serialize = "some")]
    Some,
    #[strum(serialize = "fun")]
    Fun,
    #[strum(serialize = "fn")]
    Fn,
    #[strum(serialize = "val")]
    Val,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "extern")]
    Extern,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "then")]
    Then,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "elif")]
    Elif,
    #[strum(serialize = "match")]
    Match,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "with")]
    With,
    #[strum(serialize = "in")]
    In,
    #[strum(serialize = "handle")]
    Handle,
    #[strum(serialize = "handler")]
    Handler,
    #[strum(serialize = "mask")]
    Mask,
    #[strum(serialize = "ctl")]
    Ctl,
    #[strum(serialize = "final")]
    Final,
    #[strum(serialize = "raw")]
    Raw,
    #[strum(serialize = "override")]
    Override,
    #[strum(serialize = "named")]
    Named,
    #[strum(serialize = "interface")]
    Interface,
    #[strum(serialize = "break")]
    Break,
    #[strum(serialize = "continue")]
    Continue,
    #[strum(serialize = "unsafe")]
    Unsafe,
}

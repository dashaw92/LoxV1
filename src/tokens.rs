#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum TTy {
    LParen, RParen, LBrace, RBrace, Comma, Period, Minus, Plus, Semicolon, FSlash, Asterisk,

    Bang, BangEq, Eq, EqEq, Gt, GtEq, Lt, LtEq,

    Ident, String, Number,

    And, Class, Else, False, Fn, For, If, Null, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug)]
pub(crate) enum TLit {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Token {
    ty: TTy,
    lexeme: String,
    literal: TLit,
    line: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(ty: TTy, lexeme: impl ToString, literal: TLit, line: usize) -> Self {
        Self { ty, lexeme: lexeme.to_string(), literal, line }
    }
}
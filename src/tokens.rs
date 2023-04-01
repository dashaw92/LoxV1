/// All accepted token types in the language
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TTy {
    //Single char
    LParen, RParen, LBrace, RBrace, Comma, Period, Minus, Plus, Semicolon, FSlash, Asterisk,

    //1+ char
    Bang, BangEq, Eq, EqEq, Gt, GtEq, Lt, LtEq,

    //Many chars
    Ident, String, Number,

    //Reserved keywords
    And, Class, Else, False, Fn, For, If, Null, Or,
    Print, Return, Super, This, True, Var, While,

    //The end of the script
    EOF,
}

/// Associated literals for some tokens
// TODO: Place these in TTy variants
#[derive(Debug)]
pub(crate) enum TLit {
    //Literal `null`
    Null,
    //Any number (always floating point)
    Number(f64),
    //String literals: "hello world"
    String(String),
    //Boolean literals: true, false
    Bool(bool),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Token {
    //The type of this token
    ty: TTy,
    //Literal source code that mapped to this token
    lexeme: String,
    //The interpreted literal value of this token, or TLit::Null.
    // TODO: Merge this with respective TTy variants.
    literal: TLit,
    //Error reporting: what line in the code this token was parsed from.
    line: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(ty: TTy, lexeme: impl ToString, literal: TLit, line: usize) -> Self {
        Self { ty, lexeme: lexeme.to_string(), literal, line }
    }
}
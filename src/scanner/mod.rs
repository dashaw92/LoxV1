use crate::{tokens::{Token, TTy, TLit}, error_log::error};

pub(crate) struct Scanner {
    //Note: using chars makes this code UTF-8 aware, meaning the input
    //code can contain non-ASCII codepoints, such as funky accented chars,
    //or potentially even emojis.
    buf: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    //Becomes (scanner is consumed to produce this):
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            buf: source.chars().collect(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.reached_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TTy::EOF, "", TLit::Null, self.line));
        self.tokens
    }

    fn reached_eof(&self) -> bool {
        self.current >= self.buf.len()
    }

    fn scan_token(&mut self) {
        use crate::tokens::TTy::*;

        let ch = self.advance();
        let token = match ch {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            ',' => Comma,
            '.' => Period,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '*' => Asterisk,
            '!' => self.expect_many(&['='], BangEq, Bang),
            '=' => self.expect_many(&['='], EqEq, Eq),
            '<' => self.expect_many(&['='], LtEq, Lt),
            '>' => self.expect_many(&['='], GtEq, Gt),
            '/' => {
                let ty = self.expect_many(&['/'], Null, FSlash);
                if ty == Null {
                    while self.peek() != '\n' && !self.reached_eof() {
                        self.advance();
                    }
                    return
                }

                ty
            }
            '"' => {
                self.expect_string();
                return;
            },
            ' ' | '\r' | '\t' => return,
            '\n' => {
                self.line += 1;
                return
            }
            _ => {
                if ch.is_digit(10) {
                    self.expect_number();
                    return;
                } else if ch.is_alphabetic() {
                    self.expect_ident();
                    return;
                }

                error(self.line, "Unexpected char.");
                return;
            }
        };

        self.add_token(token);
    }

    fn expect_many(&mut self, expected: &[char], yes: TTy, no: TTy) -> TTy {
        if self.current + expected.len() >= self.buf.len() {
            return no;
        }

        if self.buf[self.current .. self.current + expected.len()] != *expected {
            return no;
        }

        self.current += expected.len();
        yes
    }

    fn span_string(&self) -> String {
        self.buf[self.start .. self.current].iter().collect()
    }

    fn expect_string(&mut self) {
        while self.peek() != '"' && !self.reached_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.reached_eof() {
            error(self.line, "Unterminated string literal.");
            return;
        }

        self.advance();

        let lit: String = {
            let span = self.span_string();
            span[1 .. span.len() - 1].to_owned()
        };
        self.add_token_lit(TTy::String, TLit::String(lit));
    }

    fn expect_number(&mut self) {
        while self.peek().is_digit(10) && !self.reached_eof() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_ahead(1).is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let lit = self.span_string();
        self.add_token_lit(TTy::Number, TLit::Number(lit.parse().expect("Invalid digit")));
    }

    fn expect_ident(&mut self) {
        use TTy::*;

        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let span = self.span_string();
        let ty = match span.as_str() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "for" => For,
            "fn" => Fn,
            "if" => If,
            "null" => Null,
            "or" => Or,
            "print" => Print,
            "ret" => Return,
            "super" => Super,
            "self" => This,
            "var" => Var,
            "while" => While,
            "True" => {
                self.add_token_lit(True, TLit::Bool(true));
                return;
            },
            "False" => {
                self.add_token_lit(False, TLit::Bool(false));
                return;
            },
            _ => Ident,
        };
        self.add_token(ty);
    }

    fn peek(&self) -> char {
        if self.reached_eof() {
            return '\0';
        }

        self.buf[self.current]
    }

    fn peek_ahead(&self, offset: usize) -> char {
        if self.current + offset >= self.buf.len() {
            return '\0';
        }

        self.buf[self.current + offset]
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.buf[self.current - 1]
    }

    fn add_token(&mut self, ty: TTy) {
        self.add_token_lit(ty, TLit::Null);
    }

    fn add_token_lit(&mut self, ty: TTy, lit: TLit) {
        let src: String = self.span_string();
        self.tokens.push(Token::new(ty, src, lit, self.line));
    }
}
use crate::{tokens::{Token, TTy, TLit}, error_log::error};

//Represents a lexer for the language, maintaining position and spans
//within the provided source code. The only public method on this struct
//consumes the instance, lexing the code from start to end to construct
//a list of tokens.
pub(crate) struct Scanner {
    //Note: using chars makes this code UTF-8 aware, meaning the input
    //code can contain non-ASCII codepoints, such as funky accented chars,
    //or potentially even emojis.
    buf: Vec<char>,
    //Marks the beginning of the current span.
    //Is incremented in the scan_tokens loop.
    start: usize,
    //Marks the current position, or the end, of the current span.
    current: usize,
    //The current line number of the script.
    //Has no meaning in context of the read tokens, and is only
    //used for error reporting.
    line: usize,
    //Holds the list of already parsed tokens.
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

    /// Consumes the source code from start to finish,
    /// yielding the complete list of lexed tokens.
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.reached_eof() {
            self.start = self.current;
            self.scan_token();
        }

        //Manually insert the EOF marker once the scanner is at the end.
        self.tokens.push(Token::new(TTy::EOF, "", TLit::Null, self.line));
        //Consumes self, effectively mapping Scanner to Vec<Token>
        self.tokens
    }

    fn reached_eof(&self) -> bool {
        self.current >= self.buf.len()
    }

    //Responsible for actually generating a Token from the current span.
    fn scan_token(&mut self) {
        use crate::tokens::TTy::*;

        //Read the current char and advance the position
        let ch = self.advance();
        let token = match ch {
            //Unambiguous cases:
            //These tokens are always exactly 1 char long (for now),
            //meaning if these are read, we can definitively know what token type it is.
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
            //Potentially ambiguous cases:
            //These tokens may be one or more distinct token types.
            '!' => self.expect_many(&['='], BangEq, Bang),
            '=' => self.expect_many(&['='], EqEq, Eq),
            '<' => self.expect_many(&['='], LtEq, Lt),
            '>' => self.expect_many(&['='], GtEq, Gt),
            //Could potentially be a FSlash or a line comment.
            '/' => {
                //If expect_many returns Null for this, the current buffer is ['/', '/'],
                //AKA it's a line comment. Otherwise, it's an FSlash.
                let ty = self.expect_many(&['/'], Null, FSlash);
                if ty == Null {
                    //Consume the buffer until we reach a newline, ending the line comment.
                    while self.peek() != '\n' && !self.reached_eof() {
                        self.advance();
                    }
                    //Discard everything we read; Comments are not useful
                    return
                }

                FSlash
            }
            '"' => {
                self.expect_string();
                return;
            },
            //Ignore whitespace
            ' ' | '\r' | '\t' => return,
            //Increment the line counter and continue
            '\n' => {
                self.line += 1;
                return
            }
            //Edge cases:
            _ => {
                //Parse numbers
                if ch.is_digit(10) {
                    self.expect_number();
                    return;
                }
                //Parse identifiers (and keywords)
                else if ch.is_alphabetic() {
                    self.expect_ident();
                    return;
                }

                //Unhandled chars: report it and continue.
                error(self.line, "Unexpected char.");
                return;
            }
        };

        self.add_token(token);
    }

    //Expects the next n chars to match the expected slice.
    //true => yes
    //false => no
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

    //Builds the span represented by self.start up to self.current as a String
    //['n', 'u', 'l', 'l'], start = 0, current = 4:
    //span_string() => "null"
    fn span_string(&self) -> String {
        self.buf[self.start .. self.current].iter().collect()
    }

    //Consumes the buffer until a matching end quote (") is found.
    fn expect_string(&mut self) {
        //While the end quote hasn't been found and we're not at the end
        while self.peek() != '"' && !self.reached_eof() {
            //Track newlines (meaning string literals are multiline enabled)
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.reached_eof() {
            error(self.line, "Unterminated string literal.");
            return;
        }

        //Consume the end quote: it's not part of the string literal, it's just syntax.
        self.advance();

        //Get the current span (includes the quotes), and then manually truncate them out.
        let lit: String = {
            let span = self.span_string();
            span[1 .. span.len() - 1].to_owned()
        };
        self.add_token_lit(TTy::String, TLit::String(lit));
    }

    //Parses a f64 literal
    fn expect_number(&mut self) {
        while self.peek().is_digit(10) && !self.reached_eof() {
            self.advance();
        }

        //Handle the fractional part
        if self.peek() == '.' && self.peek_ahead(1).is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let lit = self.span_string();
        self.add_token_lit(TTy::Number, TLit::Number(lit.parse().expect("Invalid digit")));
    }

    //Reads in an identifier.
    //Handles reserved keywords (if found)
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
            //True and False are intentionally proper cased- I think it looks better.
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

    //Read the next char or return null if it's out of bounds.
    fn peek(&self) -> char {
        if self.reached_eof() {
            return '\0';
        }

        self.buf[self.current]
    }

    //Read the char n ahead of the current position, or return null if it's out of bounds.
    fn peek_ahead(&self, offset: usize) -> char {
        if self.current + offset >= self.buf.len() {
            return '\0';
        }

        self.buf[self.current + offset]
    }

    //Read the next char and advance the position
    fn advance(&mut self) -> char {
        self.current += 1;
        self.buf[self.current - 1]
    }

    //Add a token to the list.
    //Automatically inserts a literal Null into the token.
    fn add_token(&mut self, ty: TTy) {
        self.add_token_lit(ty, TLit::Null);
    }

    //Add a token and associated literal to the list
    fn add_token_lit(&mut self, ty: TTy, lit: TLit) {
        let src: String = self.span_string();
        self.tokens.push(Token::new(ty, src, lit, self.line));
    }
}
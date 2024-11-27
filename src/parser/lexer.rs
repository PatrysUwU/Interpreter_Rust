use bytes::{Buf, BytesMut};
use crate::parser::{Token, TokenType, print_error, Value};

pub struct Lexer {
    source: BytesMut,
    pub tokens: Vec<Token>,
    line: i32,
    curr: usize,
    start: usize,
    pub exit_code: i32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            source: BytesMut::from(source.as_str()),
            line: 1,
            curr: 0,
            start: 1,
            exit_code: 0,
        }
    }


    pub fn tokenize(&mut self) {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), Value::Nil, self.line));
    }

    fn is_at_end(&self) -> bool {
        if self.curr >= self.source.len() {
            true
        } else {
            false
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => { self.add_token(TokenType::LEFT_PAREN, Value::Nil) }
            b')' => { self.add_token(TokenType::RIGHT_PAREN, Value::Nil) }
            b'{' => { self.add_token(TokenType::LEFT_BRACE, Value::Nil) }
            b'}' => { self.add_token(TokenType::RIGHT_BRACE, Value::Nil) }
            b',' => { self.add_token(TokenType::COMMA, Value::Nil) }
            b'.' => { self.add_token(TokenType::DOT, Value::Nil) }
            b'-' => { self.add_token(TokenType::MINUS, Value::Nil) }
            b'+' => { self.add_token(TokenType::PLUS, Value::Nil) }
            b';' => { self.add_token(TokenType::SEMICOLON, Value::Nil) }
            b'*' => { self.add_token(TokenType::STAR, Value::Nil) }
            b'"' => { self.string() }
            b'!' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::BANG_EQUAL, Value::Nil) }
                    false => { self.add_token(TokenType::BANG, Value::Nil) }
                }
            }
            b'=' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::EQUAL_EQUAL, Value::Nil) }
                    false => { self.add_token(TokenType::EQUAL, Value::Nil) }
                }
            }
            b'<' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::LESS_EQUAL, Value::Nil) }
                    false => { self.add_token(TokenType::LESS, Value::Nil) }
                }
            }
            b'>' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::GREATER_EQUAL, Value::Nil) }
                    false => { self.add_token(TokenType::GREATER, Value::Nil) }
                }
            }
            b'/' => {
                match self.match_next(b'/') {
                    true => { self.skip_line() }
                    false => { self.add_token(TokenType::SLASH, Value::Nil) }
                }
            }
            b' ' | b'\t' | b'\r' => {}
            b'\n' => { self.line += 1 }
            other => {
                if { other.is_ascii_digit() } {
                    self.number();
                } else if other.is_ascii_alphabetic() || other == b'_' {
                    self.identifier();
                } else {
                    print_error(self.line, format!("Unexpected character: {}", other as char));
                    self.exit_code = 65
                }
            }
        }
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let val = std::str::from_utf8(&self.source[self.start..self.curr]).unwrap();
        match val {
            "and" => self.add_token(TokenType::AND, Value::Nil),
            "class" => self.add_token(TokenType::CLASS, Value::Nil),
            "else" => self.add_token(TokenType::ELSE, Value::Nil),
            "false" => self.add_token(TokenType::FALSE, Value::Nil),
            "for" => self.add_token(TokenType::FOR, Value::Nil),
            "fun" => self.add_token(TokenType::FUN, Value::Nil),
            "if" => self.add_token(TokenType::IF, Value::Nil),
            "nil" => self.add_token(TokenType::NIL, Value::Nil),
            "or" => self.add_token(TokenType::OR, Value::Nil),
            "print" => self.add_token(TokenType::PRINT, Value::Nil),
            "return" => self.add_token(TokenType::RETURN, Value::Nil),
            "super" => self.add_token(TokenType::SUPER, Value::Nil),
            "this" => self.add_token(TokenType::THIS, Value::Nil),
            "true" => self.add_token(TokenType::TRUE, Value::Nil),
            "var" => self.add_token(TokenType::VAR, Value::Nil),
            "while" => self.add_token(TokenType::WHILE, Value::Nil),
            _ => {
                self.add_token(TokenType::IDENTIFIER, Value::Nil)
            }
        }
    }
    fn string(&mut self) {
        loop {
            if let Some(x) = self.peek() {
                if x == b'"' || self.is_at_end() {
                    break;
                }
                if x == b'\n' {
                    self.line += 1;
                }
                self.advance();

                if self.is_at_end() {
                    self.exit_code = 65;
                    print_error(self.line, "Unterminated string.".to_string());
                    return;
                }
            } else {
                break;
            }
        }
        self.advance();
        let val = &self.source[self.start + 1..self.curr - 1];
        self.add_token(TokenType::STRING, Value::String(String::from_utf8(val.to_vec()).unwrap().parse().unwrap()));
    }
    fn number(&mut self) {
        while let Some(x) = self.peek() {
            if !x.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        if let Some(x) = self.peek() {
            if let Some(y) = self.peek_next() {
                if x == b'.' && y.is_ascii_digit() {
                    self.advance();
                    while let Some(x) = self.peek() {
                        if !x.is_ascii_digit() {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }
        let val = &self.source[self.start..self.curr];
        self.add_token(TokenType::NUMBER, Value::Number(String::from_utf8(val.to_vec()).unwrap().parse().unwrap()));
    }

    fn peek_next(&mut self) -> Option<u8> {
        self.curr += 1;
        if !self.is_at_end() {
            self.curr -= 1;
            Some(self.source[self.curr + 1])
        } else {
            self.curr -= 1;
            None
        }
    }

    fn skip_line(&mut self) {
        while let Some(x) = self.peek() {
            if x == b'\n' || self.is_at_end() {
                break;
            }
            self.advance();
        }
    }

    fn add_token(&mut self, token_type: TokenType, token_value: Value) {
        let text = &self.source[self.start..self.curr];
        let text: String = String::from_utf8(text.to_vec()).unwrap();
        self.tokens.push(Token::new(token_type, text, token_value, self.line))
    }

    fn advance(&mut self) -> u8 {
        let temp = self.source[self.curr];
        self.curr += 1;
        temp
    }

    fn peek(&self) -> Option<u8> {
        if !self.is_at_end() {
            Some(self.source[self.curr])
        } else {
            None
        }
    }

    fn match_next(&mut self, c: u8) -> bool {
        if let Some(x) = self.peek() {
            if x == c {
                self.curr += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
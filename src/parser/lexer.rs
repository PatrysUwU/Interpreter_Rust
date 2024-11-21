use bytes::BytesMut;
use crate::parser::{Token, TokenType, print_error};

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
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), self.line));
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
            b'(' => { self.add_token(TokenType::LEFT_PAREN) }
            b')' => { self.add_token(TokenType::RIGHT_PAREN) }
            b'{' => { self.add_token(TokenType::LEFT_BRACE) }
            b'}' => { self.add_token(TokenType::RIGHT_BRACE) }
            b',' => { self.add_token(TokenType::COMMA) }
            b'.' => { self.add_token(TokenType::DOT) }
            b'-' => { self.add_token(TokenType::MINUS) }
            b'+' => { self.add_token(TokenType::PLUS) }
            b';' => { self.add_token(TokenType::SEMICOLON) }
            b'*' => { self.add_token(TokenType::STAR) }
            b'"' => { self.string() }
            b'!' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::BANG_EQUAL) }
                    false => { self.add_token(TokenType::BANG) }
                }
            }
            b'=' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::EQUAL_EQUAL) }
                    false => { self.add_token(TokenType::EQUAL) }
                }
            }
            b'<' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::LESS_EQUAL) }
                    false => { self.add_token(TokenType::LESS) }
                }
            }
            b'>' => {
                match self.match_next(b'=') {
                    true => { self.add_token(TokenType::GREATER_EQUAL) }
                    false => { self.add_token(TokenType::GREATER) }
                }
            }
            b'/' => {
                match self.match_next(b'/') {
                    true => { self.skip_line() }
                    false => { self.add_token(TokenType::SLASH) }
                }
            }
            b' ' | b'\t' | b'\r' => {}
            b'\n' => { self.line += 1 }

            other => {
                print_error(self.line, format!("Unexpected character: {}", other as char));
                self.exit_code = 65
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

                self.advance();
            } else {
                break;
            }
        }
        let val = &self.source[self.start + 1..self.curr - 1];
        self.add_token(TokenType::STRING(String::from_utf8(val.to_vec()).unwrap()));
    }

    fn skip_line(&mut self) {
        while let Some(x) = self.peek() {
            if x == b'\n' || self.is_at_end() {
                break;
            }
            self.advance();
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.curr];
        let text: String = String::from_utf8(text.to_vec()).unwrap();
        self.tokens.push(Token::new(token_type, text, self.line))
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
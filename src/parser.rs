use std::fmt;
use std::fmt::{write, Formatter};

mod lexer;
use lexer::Lexer;

pub fn print_error(line: i32, message: String) {
    eprintln!("[line {}] Error :{}", line, message);
}
#[derive(Debug)]
#[allow(dead_code, non_camel_case_types)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // one or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,

    GREATER,
    GREATE_REQUAL,
    LESS,
    LESS_EQUAL,

    // literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    // keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

enum LiteralValue {
    Int(i32),
    String(String),
    Null,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            LiteralValue::Int(x) => { write!(f, "{x}") }
            LiteralValue::String(x) => { write!(f, "{x}") }
            LiteralValue::Null => { write!(f, "null") }
        }
    }
}
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: LiteralValue,
    line: i32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,

        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}


pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    pub exit_code: i32,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Parser {
            lexer: Lexer::new(source),
            tokens: Vec::new(),
            exit_code: 0,
        }
    }

    pub fn print_tokens(&mut self) {
        self.lexer.tokenize();
        for token in self.lexer.tokens.iter() {
            println!("{}", token);
        }
        self.exit_code = self.lexer.exit_code;
    }
}
use std::fmt;
use std::fmt::{write, Formatter};

mod lexer;
use lexer::Lexer;

pub fn print_error(line: i32, message: String) {
    eprintln!("[line {}] Error: {}", line, message);
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
    GREATER_EQUAL,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LEFT_PAREN => { write!(f, "LEFT_PAREN") }
            TokenType::RIGHT_PAREN => { write!(f, "RIGHT_PAREN") }
            TokenType::LEFT_BRACE => { write!(f, "LEFT_BRACE") }
            TokenType::RIGHT_BRACE => { write!(f, "RIGHT_BRACE") }
            TokenType::COMMA => { write!(f, "COMMA") }
            TokenType::DOT => { write!(f, "DOT") }
            TokenType::MINUS => { write!(f, "MINUS") }
            TokenType::PLUS => { write!(f, "PLUS") }
            TokenType::SEMICOLON => { write!(f, "SEMICOLON") }
            TokenType::SLASH => { write!(f, "SLASH") }
            TokenType::STAR => { write!(f, "STAR") }
            TokenType::BANG => { write!(f, "BANG") }
            TokenType::BANG_EQUAL => { write!(f, "BANG_EQUAL") }
            TokenType::EQUAL => { write!(f, "EQUAL") }
            TokenType::EQUAL_EQUAL => { write!(f, "EQUAL_EQUAL") }
            TokenType::GREATER => { write!(f, "GREATER") }
            TokenType::GREATER_EQUAL => { write!(f, "GREATER_EQUAL") }
            TokenType::LESS => { write!(f, "LESS") }
            TokenType::LESS_EQUAL => { write!(f, "LESS_EQUAL") }
            TokenType::IDENTIFIER(_) => { write!(f, "IDENTIFIER") }
            TokenType::STRING(_) => { write!(f, "STRING") }
            TokenType::NUMBER(_) => { write!(f, "NUMBER") }
            TokenType::AND => { write!(f, "AND") }
            TokenType::CLASS => { write!(f, "CLASS") }
            TokenType::ELSE => { write!(f, "ELSE") }
            TokenType::FALSE => { write!(f, "FALSE") }
            TokenType::FUN => { write!(f, "FUN") }
            TokenType::FOR => { write!(f, "FOR") }
            TokenType::IF => { write!(f, "IF") }
            TokenType::NIL => { write!(f, "NIL") }
            TokenType::OR => { write!(f, "OR") }
            TokenType::PRINT => { write!(f, "PRINT") }
            TokenType::RETURN => { write!(f, "RETURN") }
            TokenType::SUPER => { write!(f, "SUPER") }
            TokenType::THIS => { write!(f, "THIS") }
            TokenType::TRUE => { write!(f, "TRUE") }
            TokenType::VAR => { write!(f, "VAR") }
            TokenType::WHILE => { write!(f, "WHILE") }
            TokenType::EOF => { write!(f, "EOF") }
        }
    }
}


pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::STRING(x) => {
                write!(f, "{} {} {}", self.token_type, self.lexeme, x)
            }
            TokenType::NUMBER(x) => {
                write!(f, "{} {} {:?}", self.token_type, self.lexeme, x)
            }
            _ => {
                write!(f, "{} {} null", self.token_type, self.lexeme)
            }
        }
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
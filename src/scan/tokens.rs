use std::any::Any;
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Syntax / single char tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Comparaison
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    r#String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub struct Token<'a> {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: &'a dyn Any,
    pub line: usize,
}

impl<'a> Token<'a> {
    fn new(r#type: TokenType, lexeme: &'a str, literal: &'a dyn Any, line: usize) -> Self {
        Self {
            r#type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}

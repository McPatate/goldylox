use crate::ast::expr::Lit;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenKind {
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
    Lit,

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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Lit>,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<Lit>, line: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.kind, self.lexeme, self.literal)
    }
}

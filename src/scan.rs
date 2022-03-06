pub mod tokens;

use std::fmt;
use tokens::{Token, TokenKind};

#[derive(Debug)]
pub enum ScanErrorTypes {
    UnexpectedCharacter,
    UnterminatedString,
}

#[derive(Debug)]
pub struct ScanError {
    pub line: usize,
    pub kind: ScanErrorTypes,
}

impl ScanError {
    fn new(line: usize, kind: ScanErrorTypes) -> Self {
        ScanError { line, kind }
    }
}

impl fmt::Display for ScanErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type ScanTokenResult = std::result::Result<Option<Token>, ScanError>;
pub type ScanResult = std::result::Result<Vec<Token>, ScanError>;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Does not support utf8 characters
    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    //    fn add_token(&mut self, kind: TokenKind, literal: Option<&'a dyn Any>) {
    //        let text = &self.source[self.start..self.current].to_owned();
    //        self.tokens
    //            .push(Token::new(kind, text, literal, self.line));
    //    }

    fn match_next_char(&mut self, next: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != next {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn string(&mut self, text: String) -> ScanTokenResult {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanError::new(
                self.line,
                ScanErrorTypes::UnterminatedString,
            ));
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_owned();
        Ok(Some(Token::new(
            TokenKind::String,
            text,
            Some(Box::new(value)),
            self.line,
        )))
    }

    fn number(&mut self, text: String) -> ScanTokenResult {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        Ok(Some(Token::new(
            TokenKind::Number,
            text,
            Some(Box::new(value)),
            self.line,
        )))
    }

    fn identifier(&mut self, text: String) -> ScanTokenResult {
        while self.peek().is_ascii_digit() || self.peek().is_ascii_alphabetic() {
            self.advance();
        }
        let identifier = match &self.source[self.start..self.current] {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        };
        Ok(Some(Token::new(identifier, text, None, self.line)))
    }

    fn scan_token(&mut self) -> ScanTokenResult {
        let c = self.advance();
        let text = self.source[self.start..self.current].to_owned();
        let res = match c {
            '(' => Ok(Some(Token::new(
                TokenKind::LeftParenthesis,
                text,
                None,
                self.line,
            ))),
            ')' => Ok(Some(Token::new(
                TokenKind::RightParenthesis,
                text,
                None,
                self.line,
            ))),
            '{' => Ok(Some(Token::new(
                TokenKind::LeftBrace,
                text,
                None,
                self.line,
            ))),
            '}' => Ok(Some(Token::new(
                TokenKind::RightBrace,
                text,
                None,
                self.line,
            ))),
            ',' => Ok(Some(Token::new(TokenKind::Comma, text, None, self.line))),
            '.' => Ok(Some(Token::new(TokenKind::Dot, text, None, self.line))),
            '-' => Ok(Some(Token::new(TokenKind::Minus, text, None, self.line))),
            '+' => Ok(Some(Token::new(TokenKind::Plus, text, None, self.line))),
            ';' => Ok(Some(Token::new(
                TokenKind::Semicolon,
                text,
                None,
                self.line,
            ))),
            '*' => Ok(Some(Token::new(TokenKind::Star, text, None, self.line))),
            '!' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenKind::BangEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenKind::Bang, text, None, self.line)))
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenKind::EqualEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenKind::Equal, text, None, self.line)))
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenKind::LessEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenKind::Less, text, None, self.line)))
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenKind::GreaterEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenKind::Greater, text, None, self.line)))
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenKind::Slash, text, None, self.line)))
                }
            }
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            '"' => self.string(text),
            c => {
                if c.is_ascii_digit() {
                    self.number(text)
                } else if c.is_ascii_alphabetic() {
                    self.identifier(text)
                } else {
                    Err(ScanError::new(
                        self.line,
                        ScanErrorTypes::UnexpectedCharacter,
                    ))
                }
            }
        };
        self.start = self.current;
        res
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> ScanResult {
        let mut tokens = Vec::new();
        loop {
            if self.is_at_end() {
                break;
            }
            match self.scan_token()? {
                Some(token) => tokens.push(token),
                None => (),
            }
        }
        tokens.push(Token::new(TokenKind::EOF, "".to_owned(), None, self.line));
        Ok(tokens)
    }
}

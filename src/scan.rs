mod tokens;

use std::fmt;
use tokens::{Token, TokenType};

#[derive(Debug)]
pub enum ScanErrorTypes {
    UnexpectedCharacter,
    UnterminatedString,
}

#[derive(Debug)]
pub struct ScanError {
    pub line: usize,
    pub r#type: ScanErrorTypes,
}

impl ScanError {
    fn new(line: usize, r#type: ScanErrorTypes) -> Self {
        ScanError { line, r#type }
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

    // we do not support utf8 characters
    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    //    fn add_token(&mut self, r#type: TokenType, literal: Option<&'a dyn Any>) {
    //        let text = &self.source[self.start..self.current].to_owned();
    //        self.tokens
    //            .push(Token::new(r#type, text, literal, self.line));
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
            TokenType::String,
            text,
            Some(Box::new(value)),
            self.line,
        )))
    }

    fn scan_token(&mut self) -> ScanTokenResult {
        let c = self.advance();
        let text = self.source[self.start..self.current].to_owned();
        match c {
            '(' => Ok(Some(Token::new(
                TokenType::LeftParenthesis,
                text,
                None,
                self.line,
            ))),
            ')' => Ok(Some(Token::new(
                TokenType::RightParenthesis,
                text,
                None,
                self.line,
            ))),
            '{' => Ok(Some(Token::new(
                TokenType::LeftBrace,
                text,
                None,
                self.line,
            ))),
            '}' => Ok(Some(Token::new(
                TokenType::RightBrace,
                text,
                None,
                self.line,
            ))),
            ',' => Ok(Some(Token::new(TokenType::Comma, text, None, self.line))),
            '.' => Ok(Some(Token::new(TokenType::Dot, text, None, self.line))),
            '-' => Ok(Some(Token::new(TokenType::Minus, text, None, self.line))),
            '+' => Ok(Some(Token::new(TokenType::Plus, text, None, self.line))),
            ';' => Ok(Some(Token::new(
                TokenType::Semicolon,
                text,
                None,
                self.line,
            ))),
            '*' => Ok(Some(Token::new(TokenType::Star, text, None, self.line))),
            '!' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenType::BangEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenType::Bang, text, None, self.line)))
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenType::EqualEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenType::Equal, text, None, self.line)))
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenType::LessEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenType::Less, text, None, self.line)))
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    Ok(Some(Token::new(
                        TokenType::GreaterEqual,
                        text,
                        None,
                        self.line,
                    )))
                } else {
                    Ok(Some(Token::new(TokenType::Greater, text, None, self.line)))
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Slash, text, None, self.line)))
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
            _ => Err(ScanError::new(
                self.line,
                ScanErrorTypes::UnexpectedCharacter,
            )),
        }
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
            match self.scan_token()? {
                Some(token) => tokens.push(token),
                None => (),
            }
            if self.is_at_end() {
                break;
            }
        }
        tokens.push(Token::new(TokenType::EOF, "".to_owned(), None, self.line));
        Ok(tokens)
    }
}

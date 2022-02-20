mod tokens;

use tokens::Token;

pub struct Scanner {
    pub source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        for t in self.source.split(' ') {
            tokens.push(Token {
                value: t.to_string(),
            });
        }
        tokens
    }
}

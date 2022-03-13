use super::token::Token;

pub struct Expr {
    pub kind: ExprKind,
}

pub enum ExprKind {
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Unary(Token, Box<Expr>),
}

#[derive(Debug)]
pub struct Lit {
    pub kind: LitKind,
}

#[derive(Debug)]
pub enum LitKind {
    Str(String),
    Number(f64),
    Bool(bool),
    Nil,
}

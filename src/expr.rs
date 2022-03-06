pub struct Expr {}

pub enum ExprKind {
    Binary(Token, Expr, Expr),
}

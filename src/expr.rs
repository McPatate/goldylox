pub struct Expr {}

pub enum ExprKind {
    Binary(Token, Expr, Expr),
    Grouping(Expr),
    Literal(Box<dyn Any>),
    Unary(Token, Expr),
}

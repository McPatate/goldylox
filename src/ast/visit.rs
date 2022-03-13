use super::expr::{Expr, ExprKind, Lit};
use super::token::Token;

pub trait Visitor<R> {
    fn visit_binary(&self, operator: &Token, lhs: &Expr, rhs: &Expr) -> R;
    fn visit_grouping(&self, expr: &Expr) -> R;
    fn visit_literal(&self, value: &Lit) -> R;
    fn visit_unary(&self, operator: &Token, rhs: &Expr) -> R;
}

impl Expr {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &V) -> R {
        match self.kind {
            ExprKind::Binary(ref operator, ref lhs_expr, ref rhs_expr) => {
                visitor.visit_binary(operator, lhs_expr, rhs_expr)
            }
            ExprKind::Grouping(ref expr) => visitor.visit_grouping(expr),
            ExprKind::Literal(ref value) => visitor.visit_literal(value),
            ExprKind::Unary(ref operator, ref expr) => visitor.visit_unary(operator, expr),
        }
    }
}

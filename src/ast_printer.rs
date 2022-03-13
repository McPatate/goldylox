use super::ast::expr::{Expr, Lit, LitKind};
use super::ast::token::Token;
use super::ast::visit::Visitor;

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut out = vec!["(".to_owned(), name];
        for expr in exprs {
            out.push(" ".to_owned());
            out.push(expr.accept(self));
        }
        out.push(")".to_owned());
        out.join("").to_string()
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, operator: &Token, lhs: &Expr, rhs: &Expr) -> String {
        self.parenthesize(operator.lexeme.to_owned(), vec![lhs, rhs])
    }

    fn visit_grouping(&self, expr: &Expr) -> String {
        self.parenthesize("group".to_owned(), vec![expr])
    }

    fn visit_literal(&self, value: &Lit) -> String {
        match &value.kind {
            LitKind::Nil => "nil".to_string(),
            v => format!("{:?}", v),
        }
    }

    fn visit_unary(&self, operator: &Token, rhs: &Expr) -> String {
        self.parenthesize(operator.lexeme.to_owned(), vec![rhs])
    }
}

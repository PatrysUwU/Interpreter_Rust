use crate::parser::expr::*;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Binary) -> String {
        format!(
            "({} {} {})",
            binary.operator.lexeme,
            binary.left.accept(self),
            binary.right.accept(self)
        )
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        format!("(group {})", grouping.expression.accept(self))
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        literal.value.clone()
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        format!("({} {})", unary.operator.lexeme, unary.right.accept(self))
    }

    fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }
}
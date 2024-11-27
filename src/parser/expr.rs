use crate::parser::Token;
pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn print(&mut self, expr: Expr) -> T;
}

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

pub struct Grouping {
    pub expression: Expr,
}

pub struct Literal {
    pub value: String,
}

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}
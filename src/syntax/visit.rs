use super::ast::{
    BinaryOperation, DefArgument, Expr, ExprKind, Identifier, Item, ItemKind, Literal,
    UnaryOperation,
};

pub trait ItemVisitor {
    type Output;

    fn visit_item(&mut self, item: &Item) -> Self::Output {
        match item.data() {
            ItemKind::Def { name, args } => self.visit_def(item, name, args),
        }
    }

    fn visit_def(
        &mut self,
        item: &Item,
        name: &Identifier,
        args: &Vec<DefArgument>,
    ) -> Self::Output;
}

pub trait ExprVisitor {
    type Output;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
        match expr.data() {
            ExprKind::Literal(literal) => self.visit_literal(expr, literal),
            ExprKind::Binary { left, op, right } => self.visit_binary(expr, left, op, right),
            ExprKind::Unary { op, expr } => self.visit_unary(expr, op, expr),
            ExprKind::Call { name, args } => self.visit_call(expr, name, args),
            ExprKind::Grouping(expr) => self.visit_expr(expr),
        }
    }

    fn visit_literal(&mut self, expr: &Expr, literal: &Literal) -> Self::Output;

    fn visit_call(&mut self, expr: &Expr, name: &Identifier, args: &Vec<Expr>) -> Self::Output;

    fn visit_binary(
        &mut self,
        expr: &Expr,
        left: &Expr,
        op: &BinaryOperation,
        right: &Expr,
    ) -> Self::Output;

    fn visit_unary(&mut self, expr: &Expr, op: &UnaryOperation, right: &Expr) -> Self::Output;
}

use super::ast::{DefArgument, Expr, ExprKind, Identifier, Item, ItemKind};

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
            ExprKind::Int(_) => self.visit_literal(expr),
            ExprKind::String(_) => {}
            ExprKind::Bool(_) => {}
            ExprKind::Binary { left, op, right } => {}
            ExprKind::Unary { op, expr } => {}
            ExprKind::Call { name, args } => {}
            ExprKind::Grouping(_) => {}
        }
    }
}

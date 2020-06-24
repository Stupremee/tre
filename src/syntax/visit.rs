use super::ast::{
    BinaryOperation, Block, DefArgument, Expr, ExprKind, Identifier, Item, ItemKind, Literal, Stmt,
    StmtKind, Type, UnaryOperation,
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
            ExprKind::Variable(name) => self.visit_variable(expr, name),
        }
    }

    fn visit_variable(&mut self, expr: &Expr, name: &Identifier) -> Self::Output;

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

pub trait StmtVisitor: ExprVisitor
where
    <Self as ExprVisitor>::Output: Into<<Self as StmtVisitor>::Output>,
{
    type Output;

    fn visit_stmt(&mut self, stmt: &Stmt) -> <Self as StmtVisitor>::Output {
        match stmt.data() {
            StmtKind::Let { name, val } => self.visit_let(stmt, name, val),
            StmtKind::If {
                cond,
                then,
                otherwise,
            } => self.visit_if(stmt, cond, then, otherwise),
            StmtKind::While { cond, block } => self.visit_while(stmt, cond, block),
            StmtKind::Loop(block) => self.visit_loop(stmt, block),
            StmtKind::Expr(expr) => self.visit_expr(expr).into(),
        }
    }

    fn visit_loop(&mut self, stmt: &Stmt, block: &Block) -> <Self as StmtVisitor>::Output;

    fn visit_while(
        &mut self,
        stmt: &Stmt,
        cond: &Expr,
        block: &Block,
    ) -> <Self as StmtVisitor>::Output;

    fn visit_if(
        &mut self,
        stmt: &Stmt,
        cond: &Expr,
        then: &Block,
        otherwise: &Block,
    ) -> <Self as StmtVisitor>::Output;

    fn visit_let(
        &mut self,
        stmt: &Stmt,
        name: &Identifier,
        val: &Expr,
    ) -> <Self as StmtVisitor>::Output;
}

use crate::syntax::{
    ast::{BinaryOperation, Expr, Identifier, Literal, UnaryOperation},
    visit::ExprVisitor,
};
use crate::{
    diagnostic::{Diagnostic, FileId, Label},
    Result, Span,
};
use std::fmt;

#[allow(unused)]
macro_rules! typecheck {
    ($self:ident, $val:expr, $ty:ident, $span:expr, $expect:expr) => {{
        if let Value::$ty(x) = $val {
            Ok(x)
        } else {
            Err($self.type_error($val, $span, $expect))
        }
    }};
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(_) => write!(f, "int"),
            Value::String(_) => write!(f, "string"),
            Value::Bool(_) => write!(f, "bool"),
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    file: FileId,
}

impl Interpreter {
    pub fn new(file: FileId) -> Self {
        Self { file }
    }

    // Error utilities

    fn new_error(&self, msg: impl Into<String>, primary_label: Label) -> Diagnostic {
        Diagnostic::error()
            .with_message(msg)
            .with_labels(vec![primary_label])
    }

    fn new_label(&self, msg: impl Into<String>, span: Span) -> Label {
        Label::primary(self.file, span).with_message(msg)
    }

    fn type_error(&self, val: &Value, span: Span, expected: &str) -> Diagnostic {
        let label = self.new_label(format!("expected '{}' found '{}'", expected, val), span);
        self.new_error("invalid type", label)
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value>;

    fn visit_literal(&mut self, _expr: &Expr, literal: &Literal) -> Self::Output {
        Ok(match literal {
            Literal::Int(x) => Value::Int(*x),
            Literal::String(x) => Value::String(x.clone()),
            Literal::Bool(x) => Value::Bool(*x),
        })
    }

    fn visit_call(&mut self, expr: &Expr, name: &Identifier, args: &Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_binary(
        &mut self,
        expr: &Expr,
        left: &Expr,
        op: &BinaryOperation,
        right: &Expr,
    ) -> Self::Output {
        let left_val = self.visit_expr(left)?;
        let left = typecheck!(self, &left_val, Int, left.span(), "int")?;
        let right_val = self.visit_expr(right)?;
        let right = typecheck!(self, &right_val, Int, right.span(), "int")?;

        match op {
            BinaryOperation::Plus => Ok(Value::Int(left + right)),
            BinaryOperation::Minus => Ok(Value::Int(left - right)),
            BinaryOperation::Mul => Ok(Value::Int(left * right)),
            BinaryOperation::Div => Ok(Value::Int(left / right)),
            BinaryOperation::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOperation::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperation::Less => Ok(Value::Bool(left < right)),
            BinaryOperation::LessEqual => Ok(Value::Bool(left <= right)),
            BinaryOperation::Greater => Ok(Value::Bool(left > right)),
            BinaryOperation::GreaterEqual => Ok(Value::Bool(left >= right)),
        }
    }

    fn visit_unary(&mut self, _expr: &Expr, op: &UnaryOperation, right: &Expr) -> Self::Output {
        let val = self.visit_expr(right)?;
        match op {
            UnaryOperation::Negate => {
                let x = typecheck!(self, &val, Int, right.span(), "int")?;
                Ok(Value::Int(-x))
            }
            UnaryOperation::Not => {
                let x = typecheck!(self, &val, Bool, right.span(), "bool")?;
                Ok(Value::Bool(!x))
            }
        }
    }

    fn visit_variable(&mut self, expr: &Expr, name: &Identifier) -> Self::Output {
        todo!()
    }
}

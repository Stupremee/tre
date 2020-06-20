use crate::syntax::{
    ast::{BinaryOperation, Expr, Identifier, Literal, UnaryOperation},
    visit::ExprVisitor,
};
use crate::{
    diagnostic::{Diagnostic, FileId, Label},
    Result, Span,
};
use std::collections::HashMap;
use std::fmt;

#[allow(unused)]
macro_rules! typecheck_value {
    ($self:ident, $val:expr, $ty:ident) => {
        match $val {
            Value::$ty => {}
            val => {
                return Err($self.new_error(
                    "invalid type",
                    $self.new_label(format!("'{}' is invalid here", val)),
                ))
            }
        }
    };
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

pub type Environment = HashMap<String, Value>;

#[derive(Debug)]
pub struct Interpreter {
    env: Environment,
    file: FileId,
}

impl Interpreter {
    pub fn new(file: FileId) -> Self {
        Self {
            file,
            env: HashMap::new(),
        }
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
        todo!()
    }

    fn visit_unary(&mut self, _expr: &Expr, op: &UnaryOperation, right: &Expr) -> Self::Output {
        let val = self.visit_expr(right)?;
        match op {
            UnaryOperation::Negate => {
                if let Value::Int(x) = val {
                    Ok(Value::Int(-x))
                } else {
                    Err(self.type_error(&val, right.span(), "int"))
                }
            }
            UnaryOperation::Not => {
                if let Value::Bool(x) = val {
                    Ok(Value::Bool(!x))
                } else {
                    Err(self.type_error(&val, right.span(), "bool"))
                }
            }
        }
    }
}

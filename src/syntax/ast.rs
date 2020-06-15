use crate::{syntax::TokenType, Spanned};
use lasso::Spur;
use std::fmt;

pub type Identifier = Spanned<Spur>;
pub type Block = Vec<Stmt>;
pub type Type = Spanned<TypeKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    U64,
    F64,
    String,
    Bool,
}

pub type Item = Spanned<ItemKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    Def {
        name: Identifier,
        args: Vec<DefArgument>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefArgument {
    pub name: Identifier,
    pub ty: Type,
}

pub type Stmt = Spanned<StmtKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    Let {
        name: Identifier,
        ty: Type,
        val: Expr,
    },
    If {
        cond: Expr,
        then: Block,
        otherwise: Block,
    },
    While {
        cond: Expr,
        block: Block,
    },
    Loop(Block),
    Expr(Expr),
}

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Literal(Literal),
    // TODO: Add support for floats
    // Float(u64),
    Binary {
        left: Box<Expr>,
        op: BinaryOperation,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOperation,
        expr: Box<Expr>,
    },
    Call {
        name: Identifier,
        args: Vec<Expr>,
    },
    Grouping(Box<Expr>),
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Literal(val) => write!(f, "{}", val),
            ExprKind::Binary { left, op, right } => write!(f, "({} {} {})", op, left, right),
            ExprKind::Unary { op, expr } => write!(f, "{}{}", op, expr),
            ExprKind::Call { name: _, args: _ } => unimplemented!(),
            ExprKind::Grouping(expr) => write!(f, "({})", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Int(i64),
    String(String),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(x) => write!(f, "{}", x),
            Literal::String(x) => write!(f, "{}", x),
            Literal::Bool(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperation {
    Not,
    Negate,
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperation::Not => write!(f, "!"),
            UnaryOperation::Negate => write!(f, "-"),
        }
    }
}

impl From<TokenType> for UnaryOperation {
    fn from(ty: TokenType) -> Self {
        UnaryOperation::from(&ty)
    }
}

impl From<&TokenType> for UnaryOperation {
    fn from(ty: &TokenType) -> Self {
        match ty {
            TokenType::Bang => UnaryOperation::Not,
            TokenType::Minus => UnaryOperation::Negate,
            // TODO: Convert to TryFrom impl
            _ => panic!("failed to convert tokentype into binary operation."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Mul,
    Div,
    NotEqual,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperation::Plus => write!(f, "+"),
            BinaryOperation::Minus => write!(f, "-"),
            BinaryOperation::Mul => write!(f, "*"),
            BinaryOperation::Div => write!(f, "/"),
            BinaryOperation::NotEqual => write!(f, "!="),
            BinaryOperation::EqualEqual => write!(f, "=="),
            BinaryOperation::Less => write!(f, "<"),
            BinaryOperation::LessEqual => write!(f, "<="),
            BinaryOperation::Greater => write!(f, ">"),
            BinaryOperation::GreaterEqual => write!(f, ">="),
        }
    }
}

impl From<TokenType> for BinaryOperation {
    fn from(ty: TokenType) -> Self {
        BinaryOperation::from(&ty)
    }
}

impl From<&TokenType> for BinaryOperation {
    fn from(ty: &TokenType) -> Self {
        match ty {
            TokenType::Plus => BinaryOperation::Plus,
            TokenType::Minus => BinaryOperation::Minus,
            TokenType::Star => BinaryOperation::Mul,
            TokenType::Slash => BinaryOperation::Div,
            TokenType::NotEqual => BinaryOperation::NotEqual,
            TokenType::EqualEqual => BinaryOperation::EqualEqual,
            TokenType::Less => BinaryOperation::Less,
            TokenType::LessEqual => BinaryOperation::LessEqual,
            TokenType::Greater => BinaryOperation::Greater,
            TokenType::GreaterEqual => BinaryOperation::GreaterEqual,
            // TODO: Convert to TryFrom impl
            _ => panic!("failed to convert tokentype into binary operation."),
        }
    }
}

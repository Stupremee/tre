use crate::{syntax::TokenType, Spanned};
use lasso::Spur;

pub type Identifier = Spanned<Spur>;

pub type Block = Vec<Expr>;

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
    },
    If {
        cond: Box<Expr>,
        then: Block,
        otherwise: Block,
    },
    While {
        cond: Box<Expr>,
        block: Block,
    },
    Loop(Block),
    Expr(Box<Expr>),
}

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Int(i64),
    String(String),
    Bool(bool),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperation {
    Not,
    Negate,
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

use crate::Spanned;
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
    Int(u64),
    // TODO: Add support for floats
    // Float(u64),
    Binary {
        left: Box<Expr>,
        op: BinaryOperation,
        right: Box<Expr>,
    },
    Unary {
        op: BinaryOperation,
        expr: Box<Expr>,
    },
    Call {
        name: Identifier,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperation {
    Negate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

// use crate::Spanned;
// use lasso::Spur;

// pub type Identifier = Spanned<Spur>;

// #[derive(Debug, PartialEq, Clone)]
// pub enum Type {
//     U64,
//     F64,
//     String,
//     Bool,
//     Custom(Identifier),
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum UnaryOperation {
//     Negate,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum BinaryOperation {
//     Plus,
//     Minus,
//     Mul,
//     Div,
//     Pow,
//     EqualEqual,
//     Less,
//     LessEqual,
//     Greater,
//     GreaterEqual,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum Expr {
//     Int(u64),
//     Float(f64),
//     Binary {
//         left: Box<Spanned<Expr>>,
//         op: BinaryOperation,
//         right: Box<Spanned<Expr>>,
//     },
//     Unary {
//         op: UnaryOperation,
//         expr: Box<Spanned<Expr>>,
//     },
//     Call {
//         name: Spanned<Identifier>,
//         args: Vec<Spanned<Expr>>,
//     },
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum Stmt {
//     Let {
//         name: Identifier,
//         ty: Option<Spanned<Type>>,
//         val: Spanned<Expr>,
//     },
//     // TODO: Add "else if" support
//     If {
//         cond: Spanned<Expr>,
//         block: Spanned<Block>
//     }
// }

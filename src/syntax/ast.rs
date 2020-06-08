use crate::Spanned;
use lasso::Spur;

pub type Identifier = Spanned<Spur>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    U64,
    F64,
    String,
    Bool,
}

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

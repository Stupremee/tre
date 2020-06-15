use crate::syntax::Parser;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Bool(bool),
}

pub type Environment = HashMap<String, Value>;

#[derive(Debug)]
pub struct Interpreter<'input> {
    env: Environment,
    parser: Parser<'parser>,
}

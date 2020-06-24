#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use std::io::{self, prelude::*};
use tre::diagnostic::*;
use tre::syntax::{visit::ExprVisitor, Parser};
use tre::{
    interpreter::{Interpreter, Value},
    Result,
};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut line).unwrap();

    let mut files = Files::new();
    let id = files.add("<stdin>", &line);
    match do_it(&files, id) {
        Ok(t) => println!("result: {:?}", t),
        Err(d) => emit(&files, &d),
    }
}

fn do_it(files: &Files<'_>, id: FileId) -> Result<Value> {
    let mut parser = Parser::new(&files, id);
    let expr = parser.next_expr()?;
    let mut eval = Interpreter::new(id);
    eval.visit_expr(&expr)
}

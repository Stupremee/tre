#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use std::io::{self, prelude::*};
use tre::diagnostic::*;
use tre::syntax::{ast, Parser};
use tre::Result;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut line).unwrap();

    let mut files = Files::new();
    let id = files.add("<stdin>", &line);
    match do_it(&files, id) {
        Ok(t) => println!("got expr: {}", t),
        Err(d) => emit(&files, &d),
    }
}

fn do_it(files: &Files<'_>, id: FileId) -> Result<ast::Expr> {
    let mut parser = Parser::new(&files, id);
    parser.next_expression()
}

#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use std::io::{self, prelude::*};
use tre::diagnostic::*;
use tre::syntax::{ast, parser::Result, Parser};

fn main() {
    match do_it() {
        Ok(t) => println!("got expr: {}", t),
        // Err(d) => emit(&files, &d),
        Err(err) => println!("{:#?}", err),
    }
}

fn do_it() -> Result<ast::Expr> {
    let mut files = Files::new();

    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut line).unwrap();

    let id = files.add("<stdin>", &line);
    let mut parser = Parser::new(&files, id);
    parser.next_expression()
}

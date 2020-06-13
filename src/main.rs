#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use tre::diagnostic::*;
use tre::syntax::{ast, parser::Result, Parser};

fn main() {
    let mut files = Files::new();
    match do_it(&mut files) {
        Ok(t) => println!("got expr: {:#?}", t),
        // Err(d) => emit(&files, &d),
        Err(err) => println!("{:#?}", err),
    }
}

fn do_it(files: &mut Files<'_>) -> Result<ast::Expr> {
    let id = files.add("input", "2 * 123 + 2");
    let mut parser = Parser::new(&files, id);
    parser.next_expression()
}

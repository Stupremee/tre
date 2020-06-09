#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use tre::diagnostic::*;
use tre::syntax::Parser;
use tre::syntax::Token;
use tre::Result;

fn main() {
    let mut files = Files::new();
    match do_it(&mut files) {
        Ok(t) => println!("got token: {:?}", t),
        Err(d) => emit(&files, &d),
    }
}

fn do_it(files: &mut Files<'_>) -> Result<Token> {
    let id = files.add("input", " 123 df");
    let mut parser = Parser::new(&files, id)?;
    parser.next_int()?;
    parser.next_item()
}

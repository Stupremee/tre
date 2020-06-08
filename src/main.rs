#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

use tre::diagnostic::*;
use tre::syntax::Parser;

fn main() {
    let mut files = Files::new();
    let id = files.add("input", "");
    let parser = Parser::new(&files, id);
    match parser {
        Ok(_) => {}
        Err(d) => emit(&files, &d),
    }
}

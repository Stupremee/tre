#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

mod repl;

fn main() {
    let mut repl = repl::Repl::new();
    match repl.run() {
        Ok(_) => {}
        Err(err) => {
            println!("repl error occurred: {}", err);
            std::process::exit(1)
        }
    }
}

#[macro_use]
extern crate lazy_static;
extern crate regex;

mod parser;

fn main() {
    parser::lexer::tokenize_str("let a = fn(a,v,b) {}");
    println!("HELLO TLJL", );
}

use lexer::Lexer;
use std::io::{self, BufRead};

mod lexer;
mod parser;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let lexer = Lexer::from_str(&line.unwrap());
        let mut parser = parser::Parser::from_tokens(lexer.into_tokens());
        let node = parser.match_expression();
        match node {
            Ok(exp_node) => println!("Expression evaluated to: {0:?}", exp_node.evaluate()),
            Err(inv_node) => println!("{}", inv_node.describe()),
        }
    }
}

use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let string = "03 + 45*5 - (653554445/8)";
    let lexer = Lexer::from_str(string);
    println!("Lexing expression {0:?}", string);
    for token in lexer {
        println!("{0:?} ", token);
    }

}

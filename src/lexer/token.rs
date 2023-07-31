use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Literal(i32),
    Add,
    Substract,
    Multiply,
    Divide,
}

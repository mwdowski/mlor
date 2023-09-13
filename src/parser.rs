use core::iter::Peekable;
use crate::lexer::*;

use self::syntax_tree::*;

mod syntax_tree;

pub struct Parser {
    tokens: Peekable<Box<dyn Iterator<Item = Token>>>,
}

impl Parser {
    fn parse_int_literal(&mut self) -> SyntaxTreeNode {
        match self.tokens.next() {
            Some(Token::IntLiteral(value)) => IntLiteralNode {value: value}.to_syntax_tree_node(),
            _ => BrokenNode{}.to_syntax_tree_node()
        }
    }
}

#[test]
fn parse_one_integer() {
    let tokens = vec![Token::IntLiteral(2137)].into_iter();
    let a = Box::new(tokens) as Box<dyn Iterator<Item = Token>>;
    let mut parser = Parser {tokens: a.peekable()};

    let res = parser.parse_int_literal();
    assert_eq!(res.node.evaluate(), 2137);
}
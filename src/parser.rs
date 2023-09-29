use std::iter::Peekable;

use crate::lexer::{
    source::CharactersSource,
    token::{Token, TokenKind},
    TokenIterator,
};

use self::syntax_tree::{ExpressionNode, FactorNode, InvalidExpressionNode, TermNode};

mod syntax_tree;

pub struct Parser<TSource: CharactersSource> {
    tokens: Peekable<TokenIterator<TSource>>,
}

impl<TSource: CharactersSource> Parser<TSource> {
    pub fn from_tokens(tokens: TokenIterator<TSource>) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<ExpressionNode, InvalidExpressionNode> {
        let result = self.match_expression()?;
        match self.tokens.peek() {
            None => Ok(result),
            Some(token) => Err(InvalidExpressionNode { expected: TokenKind::EOF, got: Some(token.clone()) })
        }
    }

    fn match_expression(&mut self) -> Result<ExpressionNode, InvalidExpressionNode> {
        let left = match self.tokens.peek() {
            Some(Token {
                kind: TokenKind::IntLiteral(_) | TokenKind::ParenthesisOpen,
                ..
            }) => self.match_term(),
            Some(token) => {
                return Err(InvalidExpressionNode {
                    expected: TokenKind::IntLiteral(0),
                    got: Some(token.clone()),
                })
            }
            None => Err(InvalidExpressionNode {
                expected: TokenKind::IntLiteral(0),
                got: None,
            }),
        }?;

        if let Some(&Token {kind: TokenKind::ParenthesisClose, ..}) = self.tokens.peek() {
            return Ok(ExpressionNode::SingleTermNode(left));
        }

        match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::AddOperator => Ok(ExpressionNode::AdditionTermNode {
                    left: left,
                    right: Box::new(self.match_expression()?),
                }),
                TokenKind::SubOperator => Ok(ExpressionNode::SubstractionTermNode {
                    left: left,
                    right: Box::new(self.match_expression()?),
                }),
                _ => {
                    return Err(InvalidExpressionNode {
                        expected: TokenKind::AddOperator,
                        got: Some(token),
                    })
                }
            },
            None => return Ok(ExpressionNode::SingleTermNode(left)),
        }
    }

    fn match_term(&mut self) -> Result<TermNode, InvalidExpressionNode> {
        let left = match self.tokens.peek() {
            Some(Token {
                kind: TokenKind::IntLiteral(_) | TokenKind::ParenthesisOpen,
                ..
            }) => self.match_factor(),
            Some(token) => {
                return Err(InvalidExpressionNode {
                    expected: TokenKind::IntLiteral(0),
                    got: Some(token.clone()),
                })
            }
            None => Err(InvalidExpressionNode {
                expected: TokenKind::IntLiteral(0),
                got: None,
            }),
        }?;

        match self.tokens.peek() {
            Some(token) => match token.kind {
                TokenKind::MulOperator => {
                    self.tokens.next();
                    Ok(TermNode::MultiplicationFactorNode {
                        left: left,
                        right: Box::new(self.match_term()?),
                    })
                }
                TokenKind::DivOperator => {
                    self.tokens.next();
                    Ok(TermNode::DivisionFactorNode {
                        left: left,
                        right: Box::new(self.match_term()?),
                    })
                }
                TokenKind::AddOperator => Ok(TermNode::SingleFactorNode(left)),
                TokenKind::SubOperator => Ok(TermNode::SingleFactorNode(left)),
                TokenKind::ParenthesisClose => Ok(TermNode::SingleFactorNode(left)),
                _ => Err(InvalidExpressionNode {
                    expected: TokenKind::MulOperator,
                    got: Some(token.clone()),
                }),
            },
            None => Ok(TermNode::SingleFactorNode(left)),
        }
    }

    fn match_factor(&mut self) -> Result<FactorNode, InvalidExpressionNode> {
        match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::IntLiteral(value) => Ok(FactorNode::LiteralNode(value)),
                TokenKind::ParenthesisOpen => {
                    let exp = self.match_expression()?;
                    match self.tokens.next() {
                        Some(Token { kind: TokenKind::ParenthesisClose, .. }) => (),
                        token => Err(InvalidExpressionNode {
                            expected: TokenKind::ParenthesisClose,
                            got: token})?,
                        };
                    Ok(FactorNode::ExpressionNode(Box::new(exp)))
                }
                _ => Err(InvalidExpressionNode {
                    expected: TokenKind::IntLiteral(0),
                    got: Some(token.clone()),
                }),
            },
            None => Err(InvalidExpressionNode { expected: TokenKind::IntLiteral(0), got: None }),
        }
    }
}

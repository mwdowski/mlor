use crate::lexer::token::{Token, TokenKind};

pub struct InvalidExpressionNode {
    pub expected: TokenKind,
    pub got: Option<Token>,
}

impl InvalidExpressionNode {
    pub fn describe(&self) -> String {
        if let Some(token) = &self.got {
            return format!("Expected {:?}, got: {:?} ({:?}).", self.expected, token.kind, token.start_position);
        }
        else {
            return format!("Unexpected file end - expected {:?}.", self.expected);
        }
    }
}

pub enum ExpressionNode {
    SingleTermNode(TermNode),
    AdditionTermNode {
        left: TermNode,
        right: Box<ExpressionNode>,
    },
    SubstractionTermNode {
        left: TermNode,
        right: Box<ExpressionNode>,
    },
}

impl ExpressionNode {
    pub fn evaluate(&self) -> i32 {
        match self {
            Self::SingleTermNode(t) => t.evaluate(),
            Self::AdditionTermNode { left, right } => left.evaluate() + right.evaluate(),
            Self::SubstractionTermNode { left, right } => left.evaluate() - right.evaluate(),
        }
    }
}

pub enum TermNode {
    SingleFactorNode(FactorNode),
    MultiplicationFactorNode {
        left: FactorNode,
        right: Box<TermNode>,
    },
    DivisionFactorNode {
        left: FactorNode,
        right: Box<TermNode>,
    },
}

impl TermNode {
    pub fn evaluate(&self) -> i32 {
        match self {
            Self::SingleFactorNode(t) => t.evaluate(),
            Self::MultiplicationFactorNode { left, right } => left.evaluate() * right.evaluate(),
            Self::DivisionFactorNode { left, right } => left.evaluate() / right.evaluate(),
        }
    }
}

pub enum FactorNode {
    LiteralNode(i32),
    ExpressionNode(Box<ExpressionNode>),
}

impl FactorNode {
    pub fn evaluate(&self) -> i32 {
        match self {
            Self::LiteralNode(value) => value.clone(),
            Self::ExpressionNode(exp) => exp.evaluate(),
        }
    }
}
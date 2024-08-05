#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub start_position: Position,
    pub end_position: Position,
    pub lexem: String,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub column: u32,
    pub row: u32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenKind {
    IntLiteral(i32),
    AddOperator,
    SubOperator,
    MulOperator,
    DivOperator,
    ParenthesisOpen,
    ParenthesisClose,
    Unrecognized,
    EOF
}
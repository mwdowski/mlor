use crate::lexer::token::TokenKind;

use self::{source::*, token::*};

pub mod source;
pub mod token;

pub struct Lexer<TSource: CharactersSource> {
    current_position: Position,
    characters: TSource,
}

impl Lexer<StringSource> {
    pub fn from_str(string: &str) -> Self {
        let source = StringSource::from_str(string);
        Self::from_character_source(source)
    }
}

impl<TSource: CharactersSource> Lexer<TSource> {
    fn advance_character(&mut self) -> Option<char> {
        let c = self.characters.next();

        match c {
            Some('\n') => {
                self.current_position.column = 1;
                self.current_position.row += 1;
            }
            Some(_) => {
                self.current_position.column += 1;
            }
            _ => (),
        }

        return c;
    }

    fn from_character_source(source: TSource) -> Self {
        Self {
            characters: source,
            current_position: Position { column: 1, row: 1 },
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.characters.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance_character();
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();
        match self.characters.peek() {
            Some(character) => match character {
                '+' | '-' | '*' | '/' => self.get_operator(),
                '0'..='9' => self.get_int_literal(),
                _ => self.get_unrecognised(),
            },
            None => None,
        }
    }

    fn get_unrecognised(&mut self) -> Option<Token> {
        let start_position = self.current_position.clone();
        let kind = TokenKind::Unrecognized;
        let mut lexem_buf = Vec::<char>::new();

        loop {
            match self.characters.peek() {
                Some(c) => match c {
                    c if c.is_whitespace() => {
                        break;
                    }
                    '+' | '-' | '*' | '/' => {
                        break;
                    }
                    _ => {
                        lexem_buf.push(self.advance_character().unwrap());
                    }
                },
                None => {
                    break;
                }
            };
        }

        match lexem_buf {
            _l if lexem_buf.len() == 0 => None,
            _ => {
                let end_position = self.current_position.clone();
                let lexem = lexem_buf.into_iter().collect::<String>();

                Some(Token {
                    start_position,
                    end_position,
                    lexem,
                    kind,
                })
            }
        }
    }

    fn get_operator(&mut self) -> Option<Token> {
        let start_position = self.current_position.clone();
        let mut lexem_buf = Vec::<char>::new();
        let kind = match self.characters.peek() {
            Some(_) => {
                let c = self.advance_character().unwrap();
                lexem_buf.push(c.clone());
                match c {
                    '+' => TokenKind::AddOperator,
                    '-' => TokenKind::SubOperator,
                    '*' => TokenKind::MulOperator,
                    '/' => TokenKind::DivOperator,
                    _ => TokenKind::Unrecognized,
                }
            }
            None => return None,
        };
        let end_position = self.current_position.clone();
        let lexem = lexem_buf.into_iter().collect::<String>();

        Some(Token {
            start_position,
            end_position,
            lexem,
            kind,
        })
    }

    fn get_int_literal(&mut self) -> Option<Token> {
        let start_position = self.current_position.clone();
        let mut kind = TokenKind::IntLiteral(0);
        let mut lexem_buf = Vec::<char>::new();

        loop {
            match self.characters.peek() {
                Some(c) => match c {
                    '0' => {
                        // first digit can't be zero
                        if lexem_buf.len() == 0 {
                            kind = TokenKind::Unrecognized;
                        }
                        lexem_buf.push(self.advance_character().unwrap());
                    }
                    '1'..='9' => {
                        lexem_buf.push(self.advance_character().unwrap());
                    }
                    c if c.is_whitespace() => {
                        break;
                    }
                    '+' | '-' | '*' | '/' => {
                        break;
                    }
                    _ => {
                        lexem_buf.push(self.advance_character().unwrap());
                        kind = TokenKind::Unrecognized;
                    }
                },
                None => {
                    break;
                }
            };
        }

        match lexem_buf {
            _l if lexem_buf.len() == 0 => None,
            _ => {
                let end_position = self.current_position.clone();
                let lexem = lexem_buf.into_iter().collect::<String>();

                kind = match kind {
                    TokenKind::IntLiteral(_) => match lexem.parse::<i32>() {
                        Ok(value) => TokenKind::IntLiteral(value),
                        _ => TokenKind::Unrecognized,
                    },
                    _ => kind,
                };

                Some(Token {
                    start_position,
                    end_position,
                    lexem,
                    kind,
                })
            }
        }
    }

    pub fn into_tokens(self) -> TokenIterator<TSource> {
        TokenIterator { lexer: self }
    }
}

pub struct TokenIterator<TSource: CharactersSource> {
    lexer: Lexer<TSource>,
}

impl<TSource: CharactersSource> Iterator for TokenIterator<TSource> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.get_next_token()
    }
}

#[test]
fn from_str() {
    let mut lexer = Lexer::from_str("abc");
    assert_eq!(lexer.characters.next().unwrap(), 'a');
    assert_eq!(lexer.characters.next().unwrap(), 'b');
    assert_eq!(lexer.characters.next().unwrap(), 'c');
    assert_eq!(lexer.characters.next(), None);
}

#[test]
fn get_int_literal() {
    let lexer = Lexer::from_str("123 0423 9000000000 65a2\n34");
    let mut tokens = lexer.into_tokens();

    let first = tokens.next().unwrap();
    assert_eq!(first.kind, TokenKind::IntLiteral(123));
    assert_eq!(first.start_position, Position { column: 1, row: 1 });
    assert_eq!(first.end_position, Position { column: 4, row: 1 });
    assert_eq!(first.lexem, format!("123"));

    let second = tokens.next().unwrap();
    assert_eq!(second.kind, TokenKind::Unrecognized);
    assert_eq!(second.start_position, Position { column: 5, row: 1 });
    assert_eq!(second.end_position, Position { column: 9, row: 1 });
    assert_eq!(second.lexem, format!("0423"));

    let third = tokens.next().unwrap();
    assert_eq!(third.kind, TokenKind::Unrecognized);
    assert_eq!(third.start_position, Position { column: 10, row: 1 });
    assert_eq!(third.end_position, Position { column: 20, row: 1 });
    assert_eq!(third.lexem, format!("9000000000"));

    let fourth = tokens.next().unwrap();
    assert_eq!(fourth.kind, TokenKind::Unrecognized);
    assert_eq!(fourth.start_position, Position { column: 21, row: 1 });
    assert_eq!(fourth.end_position, Position { column: 25, row: 1 });
    assert_eq!(fourth.lexem, format!("65a2"));

    let fifth = tokens.next().unwrap();
    assert_eq!(fifth.kind, TokenKind::IntLiteral(34));
    assert_eq!(fifth.start_position, Position { column: 1, row: 2 });
    assert_eq!(fifth.end_position, Position { column: 3, row: 2 });
    assert_eq!(fifth.lexem, format!("34"));

    assert_eq!(tokens.next(), None);
}

#[test]
fn get_operator() {
    let lexer = Lexer::from_str("+- *\n/");
    let mut tokens = lexer.into_tokens();

    let first = tokens.next().unwrap();
    assert_eq!(first.kind, TokenKind::AddOperator);
    assert_eq!(first.start_position, Position { column: 1, row: 1 });
    assert_eq!(first.end_position, Position { column: 2, row: 1 });
    assert_eq!(first.lexem, format!("+"));

    let second = tokens.next().unwrap();
    assert_eq!(second.kind, TokenKind::SubOperator);
    assert_eq!(second.start_position, Position { column: 2, row: 1 });
    assert_eq!(second.end_position, Position { column: 3, row: 1 });
    assert_eq!(second.lexem, format!("-"));

    let third = tokens.next().unwrap();
    assert_eq!(third.kind, TokenKind::MulOperator);
    assert_eq!(third.start_position, Position { column: 4, row: 1 });
    assert_eq!(third.end_position, Position { column: 5, row: 1 });
    assert_eq!(third.lexem, format!("*"));

    let fourth = tokens.next().unwrap();
    assert_eq!(fourth.kind, TokenKind::DivOperator);
    assert_eq!(fourth.start_position, Position { column: 1, row: 2 });
    assert_eq!(fourth.end_position, Position { column: 2, row: 2 });
    assert_eq!(fourth.lexem, format!("/"));

    assert_eq!(tokens.next(), None);
}

#[test]
fn get_unrecognised() {
    let lexer = Lexer::from_str("a44-W");
    let mut tokens = lexer.into_tokens();
    
    let first = tokens.next().unwrap();
    assert_eq!(first.kind, TokenKind::Unrecognized);
    assert_eq!(first.start_position, Position { column: 1, row: 1 });
    assert_eq!(first.end_position, Position { column: 4, row: 1 });
    assert_eq!(first.lexem, format!("a44"));

    let second = tokens.next().unwrap();
    assert_eq!(second.kind, TokenKind::SubOperator);
    assert_eq!(second.start_position, Position { column: 4, row: 1 });
    assert_eq!(second.end_position, Position { column: 5, row: 1 });
    assert_eq!(second.lexem, format!("-"));

    let third = tokens.next().unwrap();
    assert_eq!(third.kind, TokenKind::Unrecognized);
    assert_eq!(third.start_position, Position { column: 5, row: 1 });
    assert_eq!(third.end_position, Position { column: 6, row: 1 });
    assert_eq!(third.lexem, format!("W"));

    assert_eq!(tokens.next(), None);
}

#[test]
fn into_tokens() {
    let lexer = Lexer::from_str("2*3 - 5");
    let mut tokens = lexer.into_tokens();

    let first = tokens.next().unwrap();
    assert_eq!(first.kind, TokenKind::IntLiteral(2));
    assert_eq!(first.start_position, Position { column: 1, row: 1 });
    assert_eq!(first.end_position, Position { column: 2, row: 1 });
    assert_eq!(first.lexem, format!("2"));

    let second = tokens.next().unwrap();
    assert_eq!(second.kind, TokenKind::MulOperator);
    assert_eq!(second.start_position, Position { column: 2, row: 1 });
    assert_eq!(second.end_position, Position { column: 3, row: 1 });
    assert_eq!(second.lexem, format!("*"));

    let third = tokens.next().unwrap();
    assert_eq!(third.kind, TokenKind::IntLiteral(3));
    assert_eq!(third.start_position, Position { column: 3, row: 1 });
    assert_eq!(third.end_position, Position { column: 4, row: 1 });
    assert_eq!(third.lexem, format!("3"));

    let fourth = tokens.next().unwrap();
    assert_eq!(fourth.kind, TokenKind::SubOperator);
    assert_eq!(fourth.start_position, Position { column: 5, row: 1 });
    assert_eq!(fourth.end_position, Position { column: 6, row: 1 });
    assert_eq!(fourth.lexem, format!("-"));

    let fourth = tokens.next().unwrap();
    assert_eq!(fourth.kind, TokenKind::IntLiteral(5));
    assert_eq!(fourth.start_position, Position { column: 7, row: 1 });
    assert_eq!(fourth.end_position, Position { column: 8, row: 1 });
    assert_eq!(fourth.lexem, format!("5"));

    assert_eq!(tokens.next(), None);
}

use crate::lexer::token::TokenKind;

use self::{source::*, token::*};

mod source;
mod token;

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
                //'+' => get_add_operator(),
                //'-' => get_sub_operator(),
                //'*' => get_mul_operator(),
                //'/' => get_div_operator(),
                '0'..='9' => self.get_int_literal(),
                _ => todo!(),
            },
            None => None,
        }
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
fn lexer_from_str() {
    let mut lexer = Lexer::from_str("abc");
    assert_eq!(lexer.characters.next().unwrap(), 'a');
    assert_eq!(lexer.characters.next().unwrap(), 'b');
    assert_eq!(lexer.characters.next().unwrap(), 'c');
    assert_eq!(lexer.characters.next(), None);
}

#[test]
fn lexer_get_int_literal() {
    let lexer = Lexer::from_str("123 0423 9000000000 65a2\n34");
    let mut tokens = lexer.into_tokens();

    let first = tokens.next().unwrap();
    assert_eq!(first.kind, TokenKind::IntLiteral(123));
    assert_eq!(first.start_position, Position{column: 1, row: 1});
    assert_eq!(first.end_position, Position{column: 4, row: 1});
    assert_eq!(first.lexem, format!("123"));
    
    let second = tokens.next().unwrap();
    assert_eq!(second.kind, TokenKind::Unrecognized);
    assert_eq!(second.start_position, Position{column: 5, row: 1});
    assert_eq!(second.end_position, Position{column: 9, row: 1});
    assert_eq!(second.lexem, format!("0423"));

    let third = tokens.next().unwrap();
    assert_eq!(third.kind, TokenKind::Unrecognized);
    assert_eq!(third.start_position, Position{column: 10, row: 1});
    assert_eq!(third.end_position, Position{column: 20, row: 1});
    assert_eq!(third.lexem, format!("9000000000"));

    let fourth = tokens.next().unwrap();
    assert_eq!(fourth.kind, TokenKind::Unrecognized);
    assert_eq!(fourth.start_position, Position{column: 21, row: 1});
    assert_eq!(fourth.end_position, Position{column: 25, row: 1});
    assert_eq!(fourth.lexem, format!("65a2"));

    let fifth = tokens.next().unwrap();
    assert_eq!(fifth.kind, TokenKind::IntLiteral(34));
    assert_eq!(fifth.start_position, Position{column: 1, row: 2});
    assert_eq!(fifth.end_position, Position{column: 3, row: 2});
    assert_eq!(fifth.lexem, format!("34"));

    assert_eq!(tokens.next(), None);
}

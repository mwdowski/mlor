use std::iter::Peekable;

use self::source::StringLexerSource;

mod source;

pub struct Lexer<'a> {
    characters: Peekable<Box<dyn Iterator<Item = char> + 'a>>,
    current_stack: Vec<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    IntLiteral(i32),
    AddOperator,
    SubOperator,
    MulOperator,
    DivOperator,
    ParenOpen,
    ParenClose,
    UnmatchedLexem(String),
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.characters.next() {
            None => None,
            Some(character) => Some(match character {
                '0'..='9' => {
                    self.current_stack.push(character);
                    self.match_int_literal()
                }
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                '+' => Token::AddOperator,
                '-' => Token::SubOperator,
                '*' => Token::MulOperator,
                '/' => Token::DivOperator,
                _ => Token::UnmatchedLexem(format!("Could not parse {0:}", character)),
            }),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn from_str(string: &str) -> Self {
        let str_source = StringLexerSource::new(string);
        let source_box = Box::new(str_source) as Box<dyn Iterator<Item = char> + 'a>;
        Lexer {
            characters: source_box.peekable(),
            current_stack: Vec::new(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.characters.peek().is_some_and(|c| c.is_whitespace()) {
            self.ignore_next();
        }
    }

    fn push_next_to_stack(&mut self) -> () {
        self.current_stack.push(self.characters.next().unwrap())
    }

    fn ignore_next(&mut self) -> () {
        self.characters.next();
    }

    fn collect_int_literal(&mut self) -> Token {
        let lexem = String::from_iter(self.current_stack.iter());
        self.current_stack.clear();

        let value = lexem.parse::<i32>();

        match value {
            Ok(int) => Token::IntLiteral(int),
            Err(_) => Token::UnmatchedLexem(format!("Could not parse {0:}", lexem)),
        }
    }

    fn collect_unmatched_lexem(&mut self) -> Token {
        self.push_next_to_stack();
        let lexem = String::from_iter(self.current_stack.iter());
        self.current_stack.clear();
        Token::UnmatchedLexem(format!("Could not parse {0:}", lexem))
    }

    fn match_int_literal(&mut self) -> Token {
        loop {
            let next_char = self.characters.peek().cloned();
            match next_char {
                Some(character) => match character {
                    '0'..='9' => self.push_next_to_stack(),
                    '+' | '-' | '/' | '*' | '(' | ')' => return self.collect_int_literal(),
                    c if c.is_whitespace() => return self.collect_int_literal(),
                    _ => return self.collect_unmatched_lexem(),
                },
                None => return self.collect_int_literal(),
            };
        }
    }
}

#[test]
fn from_str() {
    let mut lexer = Lexer::from_str("abc");

    assert_eq!(lexer.current_stack.len(), 0);

    assert_eq!(lexer.characters.next(), Some('a'));
    assert_eq!(lexer.characters.next(), Some('b'));
    assert_eq!(lexer.characters.next(), Some('c'));
    assert_eq!(lexer.characters.next(), None);
}

#[test]
fn push_next_to_stack() {
    let mut lexer = Lexer::from_str("69");

    lexer.push_next_to_stack();
    assert_eq!(lexer.current_stack.get(0), Some(&'6'));

    lexer.push_next_to_stack();
    assert_eq!(lexer.current_stack.get(1), Some(&'9'));
}

#[test]
fn collect_int_literal() {
    use std::str::FromStr;

    let mut lexer = Lexer::from_str("");

    lexer.current_stack.push('6');
    lexer.current_stack.push('9');

    assert_eq!(lexer.collect_int_literal(), Token::IntLiteral(69));

    lexer.current_stack.push('6');
    lexer.current_stack.push('9');
    lexer.current_stack.push('f');

    assert_eq!(
        lexer.collect_int_literal(),
        Token::UnmatchedLexem(String::from_str("Could not parse 69f").unwrap())
    );
}

#[test]
fn match_int_literal() {
    use std::str::FromStr;

    let mut lexer = Lexer::from_str("69");
    assert_eq!(lexer.match_int_literal(), Token::IntLiteral(69));

    let mut lexer = Lexer::from_str("9s");
    assert_eq!(
        lexer.match_int_literal(),
        Token::UnmatchedLexem(String::from_str("Could not parse 9s").unwrap())
    );
}

#[test]
fn iterator_next() {
    let mut lexer = Lexer::from_str("6 + 9*(4 - 5) - 8/6");
    assert_eq!(lexer.next(), Some(Token::IntLiteral(6)));
    assert_eq!(lexer.next(), Some(Token::AddOperator));
    assert_eq!(lexer.next(), Some(Token::IntLiteral(9)));
    assert_eq!(lexer.next(), Some(Token::MulOperator));
    assert_eq!(lexer.next(), Some(Token::ParenOpen));
    assert_eq!(lexer.next(), Some(Token::IntLiteral(4)));
    assert_eq!(lexer.next(), Some(Token::SubOperator));
    assert_eq!(lexer.next(), Some(Token::IntLiteral(5)));
    assert_eq!(lexer.next(), Some(Token::ParenClose));
    assert_eq!(lexer.next(), Some(Token::SubOperator));
    assert_eq!(lexer.next(), Some(Token::IntLiteral(8)));
    assert_eq!(lexer.next(), Some(Token::DivOperator));
    assert_eq!(lexer.next(), Some(Token::IntLiteral(6)));
}

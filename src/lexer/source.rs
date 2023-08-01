use std::str::FromStr;

pub struct StringLexerSource {
    string: String,
    i: usize,
}

impl StringLexerSource {
    pub fn new(string: &str) -> Self {
        StringLexerSource {
            string: String::from_str(string).unwrap(),
            i: 0,
        }
    }
}

impl Iterator for StringLexerSource {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.string.chars().nth(self.i);
        self.i += 1;
        res
    }
}

#[test]
fn string_lexer_source() {
    let mut source = StringLexerSource::new("abc");
    assert_eq!(source.next(), Some('a'));
    assert_eq!(source.next(), Some('b'));
    assert_eq!(source.next(), Some('c'));
    assert_eq!(source.next(), None);
}

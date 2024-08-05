use std::iter::Peekable;

pub trait CharactersSource {
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<&char>;
}

pub struct StringSource {
    source: Peekable<<Vec<char> as IntoIterator>::IntoIter>
}

impl StringSource {
    pub fn from_str(string: &str) -> Self {
        let s = string.to_string();
        let v = s.chars().into_iter().collect::<Vec<char>>().into_iter();
        Self { source: v.peekable()}
    }
}

impl CharactersSource for StringSource {
    fn next(&mut self) -> Option<char> {
        self.source.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
}

#[test]
fn string_source_from_str() {
    let mut string_source = StringSource::from_str("abc");
    assert_eq!(string_source.source.next().unwrap(), 'a');
    assert_eq!(string_source.source.next().unwrap(), 'b');
    assert_eq!(string_source.source.next().unwrap(), 'c');
    assert_eq!(string_source.source.next(), None);
}

#[test]
fn string_source_next() {
    let mut string_source = StringSource::from_str("abc");
    assert_eq!(string_source.next().unwrap(), 'a');
    assert_eq!(string_source.next().unwrap(), 'b');
    assert_eq!(string_source.next().unwrap(), 'c');
    assert_eq!(string_source.next(), None);
}

#[test]
fn string_source_peek() {
    let mut string_source = StringSource::from_str("abcd");
    assert_eq!(string_source.peek().unwrap(), &'a');
}

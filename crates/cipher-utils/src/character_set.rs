/// An unordered set of characters. This is similar to an `Alphabet`, but instead of representing an ordering
/// of the english alphabet, this represents an unordered set of any characters.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CharacterSet {
    characters: std::collections::HashSet<char>,
}

impl CharacterSet {
    pub fn of(text: &str) -> Self {
        CharacterSet {
            characters: text
                .chars()
                .filter(|char| "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".contains(*char))
                .collect(),
        }
    }

    pub fn raw(text: &str) -> Self {
        CharacterSet {
            characters: text.chars().collect(),
        }
    }

    pub fn contains(&self, character: char) -> bool {
        self.characters.contains(&character)
    }

    pub fn characters(&self) -> &std::collections::HashSet<char> {
        &self.characters
    }

    pub fn is_alphabetic(&self) -> bool {
        self.characters.iter().all(|character| character.is_alphabetic())
    }

    pub fn is_alphanumeric(&self) -> bool {
        self.characters.iter().all(|character| character.is_alphanumeric())
    }
}

impl std::ops::Add for CharacterSet {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        CharacterSet {
            characters: self.characters.union(&other.characters).map(|character| character.to_owned()).collect(),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref LOWERCASE_ALPHABETIC: CharacterSet = CharacterSet::of("abcdefghijklmnopqrstuvwxyz");
    pub static ref UPPERCASE_ALPHABETIC: CharacterSet = CharacterSet::of("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pub static ref ALPHABETIC: CharacterSet = CharacterSet::of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pub static ref BASE_64: CharacterSet = CharacterSet::raw("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/");
    pub static ref MORSE: CharacterSet = CharacterSet::raw("-./");
    pub static ref NUMERIC: CharacterSet = CharacterSet::of("0123456789");
    pub static ref ALPHANUMERIC: CharacterSet = CharacterSet::of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    pub static ref HEX: CharacterSet = CharacterSet::of("0123456789ABCDEFabcdef");
    pub static ref OCTAL: CharacterSet = CharacterSet::of("01234567");
    pub static ref BINARY: CharacterSet = CharacterSet::of("01");
}

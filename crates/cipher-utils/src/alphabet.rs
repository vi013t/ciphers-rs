use std::ops::RangeBounds;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Alphabet {
    characters: Vec<char>,
    cased: bool,
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::caseless("ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap()
    }
}

impl Alphabet {
    pub fn cased(alphabet: &str) -> anyhow::Result<Self> {
        let mut chars = alphabet.chars().collect::<Vec<_>>();
        chars.dedup();
        if chars.len() != alphabet.len() {
            anyhow::bail!("Duplicate letter in alphabet: {alphabet}");
        }

        if alphabet.len() != 26 {
            anyhow::bail!("Invalid alphabet length: {alphabet}");
        }

        if alphabet.chars().any(|letter| !letter.is_alphabetic()) {
            anyhow::bail!("Invalid character found in alphabet: {alphabet}");
        }

        Ok(Self { characters: chars, cased: true })
    }

    pub fn caseless(alphabet: &str) -> anyhow::Result<Self> {
        let alphabet = alphabet.to_uppercase();
        let mut chars = alphabet.chars().collect::<Vec<_>>();
        chars.dedup();
        if chars.len() != alphabet.len() {
            anyhow::bail!("Duplicate letter in alphabet: {alphabet}");
        }

        if alphabet.len() != 26 {
            anyhow::bail!("Invalid alphabet length: {alphabet}");
        }

        if alphabet.chars().any(|letter| !letter.is_alphabetic()) {
            anyhow::bail!("Invalid character found in alphabet: {alphabet}");
        }

        Ok(Self { characters: chars, cased: false })
    }

    /// Generates an alphabet from a string of text. The created alphabet represents the unique characters
    /// of the given text in the order they appear.
    ///
    /// This is equivalent to calling `text.alphabet()` from the `Analyze` trait.
    ///
    /// # Parameters
    /// - `text` - The text to get the alphabet of.
    ///
    /// # Returns
    /// The generated alphabet
    ///
    /// # Performance
    /// This is `O(n)` for a given text of length `n`.
    pub fn of_cased(text: &str) -> Self {
        let mut characters = Vec::new();
        for character in text.chars() {
            if !characters.contains(&character) {
                characters.push(character);
            }
        }
        Self { characters, cased: true }
    }

    pub fn from_ascii_range<R: RangeBounds<u8> + IntoIterator<Item = u8>>(range: R) -> anyhow::Result<Self> {
        if range.end_bound() != std::ops::Bound::Included(&127) && range.end_bound() != std::ops::Bound::Excluded(&128) {
            anyhow::bail!("Error creating alphabet from ASCII range: Upper bound must be at most 127.");
        }

        let mut characters = Vec::new();
        for code in range {
            characters.push(code as char);
        }

        Ok(Self { characters, cased: true })
    }

    /// Returns the expected index of coincidence for a truly random string of characters of
    /// this alphabet with infinite length.
    ///
    /// # Returns
    /// The index of coincidence
    pub fn random_index_of_coincidence(&self) -> f64 {
        1f64 / self.characters.len() as f64
    }

    pub fn characters(&self) -> &[char] {
        &self.characters
    }

    pub fn index_of(&self, mut character: char) -> Option<AlphabetIndex> {
        if !self.cased {
            character = character.to_ascii_uppercase();
        }
        self.characters.iter().position(|char| char == &character).map(|index| AlphabetIndex(index as u8 + 1))
    }

    pub fn letter_at(&self, index: AlphabetIndex) -> &char {
        self.characters.get(*(index - 1) as usize).unwrap()
    }

    pub fn union(&self, other: &Self) -> Self {
        Alphabet::of_cased(&self.characters.iter().chain(other.characters.iter()).collect::<String>())
    }

    pub fn shift(&self, shift: u8) -> Self {
        let mut characters = String::new();
        for index in 1..=26 {
            let alphabet_index = AlphabetIndex::new(index).unwrap();
            characters.push(*self.letter_at(alphabet_index + shift));
        }
        Alphabet::caseless(&characters).unwrap()
    }
}

lazy_static::lazy_static! {
    pub static ref LOWERCASE_LETTERS: Alphabet = Alphabet::of_cased("abcdefghijklmnopqrstuvwxyz");
    pub static ref CAPITAL_LETTERS: Alphabet = Alphabet::of_cased("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pub static ref LETTERS: Alphabet = CAPITAL_LETTERS.union(&LOWERCASE_LETTERS);
    pub static ref NUMBERS: Alphabet = Alphabet::of_cased("1234567890");
    pub static ref LETTERS_AND_NUMBERS: Alphabet = LETTERS.union(&NUMBERS);
    pub static ref BASE_64: Alphabet = LETTERS_AND_NUMBERS.union(&Alphabet::of_cased("+/"));
    pub static ref ASCII: Alphabet = Alphabet::from_ascii_range(0..128).unwrap();
}

/// A wrapper around a `u8` that denotes a valid "alphabet index"; That is, a number that's always in `[1, 26]`.
/// `AlphabetIndex` provides safety by performing bounds checks upon creation and conciseness by allowing addition
/// and subtraction to be performed mod 26 with operator overloading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlphabetIndex(u8);

impl AlphabetIndex {
    pub fn new(index: u8) -> anyhow::Result<Self> {
        if !(1..=26).contains(&index) {
            anyhow::bail!("Alphabet index out of range: {index}")
        }

        Ok(Self(index))
    }
}

impl std::ops::Deref for AlphabetIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::AddAssign<i32> for AlphabetIndex {
    fn add_assign(&mut self, rhs: i32) {
        *self = AlphabetIndex((self.0 + rhs as u8) % 26)
    }
}

impl std::ops::Add<AlphabetIndex> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn add(self, rhs: AlphabetIndex) -> Self::Output {
        AlphabetIndex((self.0 + rhs.0) % 26)
    }
}

impl std::ops::Add<u32> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn add(self, rhs: u32) -> Self::Output {
        AlphabetIndex((self.0 + rhs as u8) % 26)
    }
}

impl std::ops::Add<u8> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn add(self, rhs: u8) -> Self::Output {
        AlphabetIndex((self.0 + rhs) % 26)
    }
}

impl std::ops::Add<i32> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn add(self, rhs: i32) -> Self::Output {
        AlphabetIndex((self.0 + rhs as u8) % 26)
    }
}

impl std::ops::Sub<AlphabetIndex> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn sub(self, rhs: AlphabetIndex) -> Self::Output {
        AlphabetIndex(((self.0 as i32 - rhs.0 as i32 + 26) % 26) as u8)
    }
}

impl std::ops::Sub<u32> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn sub(self, rhs: u32) -> Self::Output {
        AlphabetIndex(((self.0 as i32 - rhs as i32 + 26) % 26) as u8)
    }
}

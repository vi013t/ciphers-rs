/// The standard ordering of the capital English alphabet, A-Z.
pub const ALPHABET: Alphabet = Alphabet {
    alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes(),
};

/// An immutable ordering of the English alphabet. This is used by `Rotor`s and `Reflector`s, and provides helper functionality
/// such as getting the index of a character and getting the character at an index.
pub struct Alphabet<'letters> {
    /// The letters in the order of this alphabet, as a byte slice. Alphabets are ASCII-only, meaning each character can safely
    /// be stored as a single byte.
    alphabet: &'letters [u8],
}

impl<'a> Alphabet<'a> {
    /// Creates a new alphabet from the given string. The given string must contain exactly 26 unique characters, all of which are
    /// alphabetic; Otherwise, an `Err` is returned.
    ///
    /// # Parameters
    /// - `alphabet` - The string of letters
    ///
    /// # Returns
    /// The created alphabet
    pub fn new(alphabet: &'a str) -> anyhow::Result<Self> {
        // Check for duplicates
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

        Ok(Self { alphabet: alphabet.as_bytes() })
    }

    /// Creates a new `Alphabet` without checking for argument validity. This will not panic if the given argument is invalid;
    /// It just means you may panic at a later operation or get unexpected or incorrect results.
    ///
    /// This is the unsafe API's variant of `Alphabet::new()`.
    ///
    /// # Parameters
    /// - `alphabet` - The alphabet string to construct an `Alphabet` from.
    ///
    /// # Returns
    /// The constructed alphabet
    pub fn new_unchecked(alphabet: &'a str) -> Self {
        Self { alphabet: alphabet.as_bytes() }
    }

    /// Returns the zero-based index of the given letter in this alphabet, or `None` if the given charcter is not alphabetic.
    ///
    /// This is the safe API's variant of `Alphabet::unchecked_index_of()`.
    ///
    /// # Parameters
    /// - `letter` - The letter to get the index of
    ///
    /// # Returns
    /// The zero-based index of the given character, or `None` if it's not alphabetic.
    pub fn index_of(&self, letter: char) -> Option<AlphabetIndex> {
        let letter = letter.to_ascii_uppercase();
        let code = letter as u8;
        letter.is_ascii_uppercase().then(|| {
            for (index, character) in self.alphabet.iter().enumerate() {
                if character == &code {
                    return AlphabetIndex(index as u8);
                }
            }

            unreachable!()
        })
    }

    /// Returns the zero-based index of the given letter in this alphabet, or panics if the given charcter is not alphabetic or not
    /// of the right casing.
    ///
    /// This is the unsafe API's variant of `Alphabet::index_of()`.
    ///
    /// # Parameters
    /// - `letter` - The letter to get the index of
    ///
    /// # Returns
    /// The zero-based index of the given character.
    pub fn unchecked_index_of(&self, letter: char) -> u8 {
        let code = letter as u8;
        for (index, character) in self.alphabet.iter().enumerate() {
            if character == &code {
                return index as u8;
            }
        }

        panic!(
            "Attempted to get the index of '{letter}' in the alphabet \"{}\", but this alphabet doesn't contain that character.",
            String::from_utf8(self.alphabet.to_vec()).unwrap()
        )
    }

    pub fn letter_at<T: std::borrow::Borrow<AlphabetIndex>>(&self, index: T) -> char {
        self.alphabet[**index.borrow() as usize] as char
    }

    /// Returns the letters in this alphabet as a string.
    ///
    /// # Returns
    /// The letters of this alphabet in order in a string.
    pub fn letters(&self) -> String {
        String::from_utf8(self.alphabet.to_vec()).unwrap()
    }

    pub fn unchecked_letter_at(&self, index: u8) -> char {
        self.alphabet[index as usize] as char
    }
}

/// A wrapper around a `u8` that denotes a valid "alphabet index"; That is, a number that's always in `[0, 26)`.
/// `AlphabetIndex` provides safety by performing bounds checks upon creation and conciseness by allowing addition
/// and subtraction to be performed mod 26 with operator overloading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlphabetIndex(u8);

impl std::ops::Deref for AlphabetIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait IntoAlphabetIndex {
    type Output;

    fn try_into_alphabet_index(self) -> anyhow::Result<Self::Output>;
}

impl TryFrom<u8> for AlphabetIndex {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        (0..26)
            .contains(&value)
            .then_some(Self(value))
            .ok_or_else(|| anyhow::anyhow!("Alphabet index out of range: {value}"))
    }
}

impl TryFrom<i32> for AlphabetIndex {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        (0..26)
            .contains(&value)
            .then_some(Self(value as u8))
            .ok_or_else(|| anyhow::anyhow!("Alphabet index out of range: {value}"))
    }
}

impl IntoAlphabetIndex for (u8, u8, u8) {
    type Output = (AlphabetIndex, AlphabetIndex, AlphabetIndex);

    fn try_into_alphabet_index(self) -> anyhow::Result<Self::Output> {
        Ok((self.0.try_into()?, self.1.try_into()?, self.2.try_into()?))
    }
}

impl IntoAlphabetIndex for (i32, i32, i32) {
    type Output = (AlphabetIndex, AlphabetIndex, AlphabetIndex);

    fn try_into_alphabet_index(self) -> anyhow::Result<Self::Output> {
        Ok((self.0.try_into()?, self.1.try_into()?, self.2.try_into()?))
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

impl std::ops::Sub<AlphabetIndex> for AlphabetIndex {
    type Output = AlphabetIndex;

    fn sub(self, rhs: AlphabetIndex) -> Self::Output {
        AlphabetIndex(((self.0 as i32 - rhs.0 as i32 + 26) % 26) as u8)
    }
}

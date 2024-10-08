use itertools::Itertools;

use crate::frequency;

/// Returns the frequencies of each letter of the English alphabet as a map between
/// characters and percentage of words they appear in. The returned map will include both
/// lowercase and uppercase characters, with the lowercase and uppercase variant of each
/// letter having the same frequency value. To get a specific subset, use `Frequency::english_lowercase()`
/// or `Frequency::english_uppercase()`.
///
/// # Performance
/// This is `O(1)`.
///
/// # Returns
/// A map of letters and their frequencies.
pub fn english() -> &'static std::collections::HashMap<char, f64> {
    &ENGLISH_FREQUENCY
}

/// Returns the frequencies of each letter of the English alphabet as a map between
/// characters and percentage of words they appear in. The returned map will include
/// only lowercase characters. To get a different subset, use `Frequency::english_uppercase()`
/// or `Frequency::english()` for both.
///
/// # Performance
/// This is `O(1)`.
///
/// # Returns
/// A map of letters and their frequencies.
pub fn english_lowercase() -> &'static std::collections::HashMap<char, f64> {
    &ENGLISH_LOWERCASE_FREQUENCY
}

/// Returns the frequencies of each letter of the English alphabet as a map between
/// characters and percentage of words they appear in. The returned map will include
/// only uppercase characters. To get a different subset, use `Frequency::english_lowercase()`
/// or `Frequency::english()` for both.
///
/// # Performance
/// This is `O(1)`.
///
/// # Returns
/// A map of letters and their frequencies.
pub fn english_uppercase() -> &'static std::collections::HashMap<char, f64> {
    &ENGLISH_UPPERCASE_FREQUENCY
}

/// Returns a frequency map of the given text. The returned map maps characters to
/// the percent of the entire string that the character makes up. To get the counts of each character,
/// use `Frequency::counts()`.
///
/// # Performance
/// This is `O(n)`.
///
/// # Returns
/// A map of characters and the percentage of the string they make up.
pub fn of(text: &str) -> std::collections::HashMap<char, f64> {
    frequency::counts(text)
        .into_iter()
        .map(|(character, count)| (character, count as f64 / text.len() as f64))
        .collect()
}

/// Returns a frequency map of the given text. The turned map maps characters to the number of
/// times they appear in the given string. To get a frequency map that maps characters to percentages,
/// use `Frequency::of()`.
///
/// This function treats uppercase and lowercase as identical, and the returned map contains both for each character.
/// Use `Frequency::cased_counts()` to retrieve a map that's case-sensitive.
///
/// # Performance
/// This is `O(n)`.
///
/// # Returns
/// A map of characters and the number of times they appear in the given string.
pub fn counts(text: &str) -> std::collections::HashMap<char, usize> {
    text.to_lowercase().chars().counts()
}

pub fn cased_counts(text: &str) -> std::collections::HashMap<char, usize> {
    text.chars().counts()
}

/// Converts each character in the given text to the character that has the closest frequency in the English alphabet.
/// This will not reuse characters, i.e., if the closest frequency to 'B' is 'E' and the closest frequency to 'C' is
/// also 'E', once 'B' is mapped to 'E', 'C' cannot be mapped to 'E' and will be mapped to something else.
///
/// # Parameters
/// - `text` - The text to map to English frequencies
///
/// # Returns
/// The mapped text to English frequencies
pub fn mapped_to_english(text: &str) -> String {
    let mut available_frequencies = ENGLISH_LOWERCASE_FREQUENCY.clone();
    let character_frequencies = frequency::of(text);
    let mut character_map = std::collections::HashMap::new();
    text.chars()
        .map(|character| {
            *character_map.entry(character).or_insert_with(|| {
                let new_character = available_frequencies
                    .iter()
                    .map(|english| (*english.0, (english.1 - character_frequencies.get(&character).unwrap()).abs()))
                    .min_by(|first, other| first.1.total_cmp(&other.1))
                    .unwrap()
                    .0;
                available_frequencies.remove(&new_character);
                new_character
            })
        })
        .collect()
}

/// Returns the English character whose frequency is closest to the given frequency percentage.
///
/// # Parameters
/// - `frequency` - The frequency to get the closest character of. This should be a small number for
/// accurate results, i.e., around the range `0.00074 - 0.127`
pub fn closest_english_letter(frequency: f64) -> char {
    ENGLISH_LOWERCASE_FREQUENCY
        .iter()
        .map(|(letter, english_frequency)| (*letter, (english_frequency - frequency).abs()))
        .min_by(|first, other| first.1.total_cmp(&other.1))
        .unwrap()
        .0
}

/// Returns a "score" in `(0, 1]` that describes how well the given text's letter frequencies fit the same distribution
/// as standard English. A higher score (closer to 1) indicates the text's frequency is closer to English.
///
/// Note that this only scores the distribution itself, not the actual letter frequencies. For example, a simple monoalphabetic
/// substitution cipher would get an almost perfect score, since the frequency distribution is unchanged from the plaintext.
///
/// # Parameters
/// - `text` - The text to get the distribution score of.
///
/// # Returns
/// The frequency distribution fitness score, in `(0, 1]`.
pub fn distribution_score(text: &str) -> f64 {}

pub fn character_score(text: &str) -> f64 {}

lazy_static::lazy_static! {
    static ref ENGLISH_LOWERCASE_FREQUENCY: std::collections::HashMap<char, f64> = std::collections::HashMap::from([
        ('a', 0.082),
        ('b', 0.015),
        ('c', 0.028),
        ('d', 0.043),
        ('e', 0.127),
        ('f', 0.022),
        ('g', 0.020),
        ('h', 0.061),
        ('i', 0.070),
        ('j', 0.0015),
        ('k', 0.0077),
        ('l', 0.040),
        ('m', 0.024),
        ('n', 0.067),
        ('o', 0.075),
        ('p', 0.019),
        ('q', 0.00095),
        ('r', 0.060),
        ('s', 0.063),
        ('t', 0.091),
        ('u', 0.028),
        ('v', 0.0098),
        ('w', 0.024),
        ('x', 0.0015),
        ('y', 0.020),
        ('z', 0.00074),
    ]);
    static ref ENGLISH_UPPERCASE_FREQUENCY: std::collections::HashMap<char, f64> = std::collections::HashMap::from([
        ('A', 0.082),
        ('B', 0.015),
        ('C', 0.028),
        ('D', 0.043),
        ('E', 0.127),
        ('F', 0.022),
        ('G', 0.020),
        ('H', 0.061),
        ('I', 0.070),
        ('J', 0.0015),
        ('K', 0.0077),
        ('L', 0.040),
        ('M', 0.024),
        ('N', 0.067),
        ('O', 0.075),
        ('P', 0.019),
        ('Q', 0.00095),
        ('R', 0.060),
        ('S', 0.063),
        ('T', 0.091),
        ('U', 0.028),
        ('V', 0.0098),
        ('W', 0.024),
        ('X', 0.0015),
        ('Y', 0.020),
        ('Z', 0.00074)
    ]);
    static ref ENGLISH_FREQUENCY: std::collections::HashMap<char, f64> = std::collections::HashMap::from([
        ('a', 0.082),
        ('b', 0.015),
        ('c', 0.028),
        ('d', 0.043),
        ('e', 0.127),
        ('f', 0.022),
        ('g', 0.020),
        ('h', 0.061),
        ('i', 0.070),
        ('j', 0.0015),
        ('k', 0.0077),
        ('l', 0.040),
        ('m', 0.024),
        ('n', 0.067),
        ('o', 0.075),
        ('p', 0.019),
        ('q', 0.00095),
        ('r', 0.060),
        ('s', 0.063),
        ('t', 0.091),
        ('u', 0.028),
        ('v', 0.0098),
        ('w', 0.024),
        ('x', 0.0015),
        ('y', 0.020),
        ('z', 0.00074),
        ('A', 0.082),
        ('B', 0.015),
        ('C', 0.028),
        ('D', 0.043),
        ('E', 0.127),
        ('F', 0.022),
        ('G', 0.020),
        ('H', 0.061),
        ('I', 0.070),
        ('J', 0.0015),
        ('K', 0.0077),
        ('L', 0.040),
        ('M', 0.024),
        ('N', 0.067),
        ('O', 0.075),
        ('P', 0.019),
        ('Q', 0.00095),
        ('R', 0.060),
        ('S', 0.063),
        ('T', 0.091),
        ('U', 0.028),
        ('V', 0.0098),
        ('W', 0.024),
        ('X', 0.0015),
        ('Y', 0.020),
        ('Z', 0.00074)
    ]);
}

lazy_static::lazy_static! {
    static ref WORDS: Vec<&'static str> = include_str!("../data/most_common_words.txt").split_whitespace().collect();
}

/// Returns whether the given word is in the dictionary of the 10,000 most common
/// English words. To get a "score" on how common the word is, see [commonality_score].
///
/// # Parameters
/// - `word` - The word to check whether it's common.
///
/// # Performance
/// This is `O(n)` for a dictionary of `n` words. The one used here is about 10,000
/// words long, so this map have significant impacts on performance
pub fn is_common_word(word: &str) -> bool {
    WORDS.contains(&word)
}

/// Returns a reference to the `n` most common words in English (see [WORDS] for more
/// information on how frequency is determined).
///
/// # Parameters
/// - `n` - The number of words to get
///
/// # Returns
/// The `n` most common words in English.
///
/// # Performance
/// This is `O(1)`.
pub fn n_most_common(n: usize) -> &'static [&'static str] {
    WORDS.get(n..).unwrap()
}

/// Returns a "score" in `[0, 1]` that rates how common the given word is.
/// A score of 1 indicates that it is the #1 most common word ("the"), and a score
/// of 0 indicates that the word isn't on the word list at all. In general,
/// a higher score means a more common word.
///
/// # Parameters
/// - `word` - The word to score
///
/// # Returns
/// The score, in `[0, 1]`, of the given word.
///
/// # Performance
/// This is `O(n)` for a dictionary of `n` words. The one used here is about 10,000
/// words long, so this map have significant impacts on performance
pub fn commonality_score(word: &str) -> f64 {
    WORDS
        .iter()
        .position(|english| english == &word)
        .map(|position| 1. - position as f64 / WORDS.len() as f64)
        .unwrap_or(0.)
}

/// Returns the average [commonality_score] of each word in the given text. "Words"
/// are defined as being separated by whitespace. Punctuation is removed.
///
/// # Parameters
/// - `text` - The text to get the average commonality score of.
///
/// # Returns
/// The average commonality score of the words in the given text.
///
/// # Performance
/// This is `O(n)` for a text with `n` words, and [commonality] score is is `O(k)`
/// for a dictionary of `k` words, so this is effectively ~`O(10,000n)`. This may
/// significantly impact performance.
pub fn average_commonality_score(text: &str) -> f64 {
    let words = text
        .to_lowercase()
        .split_whitespace()
        .map(|word| word.chars().filter(|character| character.is_alphabetic()).collect())
        .collect::<Vec<String>>();

    words.iter().map(|word| commonality_score(word)).fold(0., |accumulator, current| accumulator + current) / words.len() as f64
}

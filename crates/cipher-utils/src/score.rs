use itertools::Itertools as _;

use crate::{dictionary, frequency, Analyze};

/// A possible plaintext. The `PossiblePlaintext` struct provides utilities for analyzing
/// and scoring texts that may be plaintexts. This is useful for brute-forcing ciphers, when
/// you need a system to find the decryption outputs that are most likely to be correct.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PossiblePlaintext(String);

impl PossiblePlaintext {
    /// Creates a new `PossiblePlaintext` with the given text as the possible plaintext.
    ///
    /// # Parameters
    /// - `plaintext` - The possible plaintext.
    ///
    /// # Returns
    /// The created `PossiblePlaintext` object
    pub fn new(plaintext: &str) -> Self {
        Self(plaintext.to_owned())
    }

    /// Returns the "score" of this plaintext. The score is based on cryptographic analysis, and a higher score
    /// indicates a better plaintext. The score is calculated from:
    ///
    /// - Index of coincidence
    /// - Word commonality
    /// - Monogram Frequency
    /// - Bigram Frequency
    /// - Trigram Frequency
    /// - Quadram Frequency
    pub fn score(&self) -> f64 {
        let ioc_score = 1. - (self.0.index_of_coincidence() - 0.0667).abs() / 0.9333;
        let frequency_distribution_score = frequency::distribution_score(&self.0);
        let frequency_character_score = frequency::character_score(&self.0);
        let bigram_distribution_score = frequency::bigram_distribution_score(&self.0);

        let mut scores = vec![ioc_score, frequency_character_score, frequency_distribution_score, bigram_distribution_score];

        // Multiple words - check for commonality
        if self.0.contains(' ') {
            let word_score = dictionary::average_commonality_score(&self.0);
            scores.push(word_score);
        }

        scores.iter().fold(0., |accumulator, current| accumulator + current) / scores.len() as f64
    }

    /// Returns the original text of this plaintext.
    ///
    /// # Returns
    /// A reference to the stored text in this plaintext.
    pub fn text(&self) -> &str {
        &self.0
    }

    /// Returns the best plaintext from the given slice based on cryptographic analysis. To get the best `n` plaintexts,
    /// use `Plaintexts::best_n`.
    ///
    /// # Parameters
    /// - `plaintexts` - The plaintexts to find the best of
    ///
    /// # Returns
    /// The best plaintext of the given slice, or `None` if it's empty.
    pub fn best<T: AsRef<str>>(plaintexts: &[T]) -> Option<String> {
        plaintexts
            .iter()
            .map(|plaintext| Self(plaintext.as_ref().to_owned()))
            .max()
            .map(|plaintext| plaintext.text().to_owned())
    }

    /// Returns the most top `n` plaintexts in order from best to worst based on cryptographic analysis. To get only the
    /// best one, use `Plaintext::best`.
    ///
    /// # Parameters
    /// - `plaintexts` - The plaintexts to find the best of
    /// - `n` - The number of best plaintexts to return
    ///
    /// # Returns
    /// The `n` best plaintexts in order from best to worst.
    ///
    /// # Errors
    /// If the given `n` is greater than the number of plaintexts (or 0), or if the given plaintext slice is empty.
    pub fn best_n<T: AsRef<str>>(plaintexts: &[T], n: usize) -> anyhow::Result<Vec<String>> {
        if n == 0 {
            anyhow::bail!("Attempted to get the best 0 plaintexts; Use a natural number instead.");
        }

        if plaintexts.is_empty() {
            anyhow::bail!("Attempted to get the best {n} plaintexts of an empty plaintext list.");
        }

        let sorted = plaintexts.iter().map(|plaintext| Self(plaintext.as_ref().to_owned())).sorted().rev().collect_vec();
        sorted
            .get(sorted.len() - n..sorted.len())
            .ok_or_else(|| anyhow::anyhow!("Error getting best n plaintexts: Index {n} is out of range of {} plaintexts", plaintexts.len()))
            .map(|ok| ok.iter().map(|plaintext| plaintext.text().to_owned()).collect())
    }
}

impl std::cmp::PartialOrd for PossiblePlaintext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for PossiblePlaintext {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().total_cmp(&other.score())
    }
}

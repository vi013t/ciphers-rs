pub mod alphabet;
pub mod character_set;
pub mod cipher_type;
pub mod dictionary;
pub mod score;
pub mod tabula_recta;

/// The `frequency` module, providing various utilities relating to frequency analysis.
pub mod frequency;

use alphabet::Alphabet;

pub trait Analyze {
    fn index_of_coincidence(&self) -> f64;

    /// Alias for `index_of_coincidence()`.
    fn ioc(&self) -> f64 {
        self.index_of_coincidence()
    }

    /// Returns an `Alphabet` containing the unique characters of this string in-order.
    fn alphabet(&self) -> Alphabet;
}

impl<T: AsRef<str>> Analyze for T {
    fn index_of_coincidence(&self) -> f64 {
        let mut frequency: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        let mut total_chars = 0;

        // Count frequency of each letter (case insensitive)
        for c in self.as_ref().chars() {
            if c.is_alphabetic() {
                let c = c.to_ascii_lowercase();
                *frequency.entry(c).or_insert(0) += 1;
                total_chars += 1;
            }
        }

        // Calculate the IC
        let mut ic = 0.0;
        for &count in frequency.values() {
            ic += (count * (count - 1)) as f64;
        }

        if total_chars > 1 {
            ic /= (total_chars * (total_chars - 1)) as f64;
        }

        ic
    }

    fn alphabet(&self) -> Alphabet {
        Alphabet::of_cased(self.as_ref())
    }
}

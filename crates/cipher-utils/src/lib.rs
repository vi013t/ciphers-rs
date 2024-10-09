pub mod alphabet;
pub mod character_set;
pub mod cipher_type;
pub mod score;

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
        let mut frequency = [0u32; 26];
        let mut total_letters = 0;

        for c in self.as_ref().chars() {
            if c.is_alphabetic() {
                let idx = c.to_ascii_lowercase() as usize - 'a' as usize;
                frequency[idx] += 1;
                total_letters += 1;
            }
        }

        if total_letters < 2 {
            return 0.0;
        }

        let mut numerator = 0u32;

        for &count in &frequency {
            numerator += count * (count - 1);
        }

        let denominator = total_letters * (total_letters - 1);

        numerator as f64 / denominator as f64
    }

    fn alphabet(&self) -> Alphabet {
        Alphabet::of_cased(self.as_ref())
    }
}

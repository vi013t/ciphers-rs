pub mod alphabet;
pub mod base64;
pub mod frequency;
pub mod score;

use alphabet::Alphabet;
use strum::IntoEnumIterator;

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

#[derive(strum_macros::EnumIter, Clone, Copy)]
pub enum Language {
    English,
    French,
    German,
    Italian,
    Russian,
    Spanish,
}

impl Language {
    pub fn best_match(text: &str) -> Language {
        let ioc = text.index_of_coincidence();
        Language::iter()
            .map(|language| (language, (language.index_of_coincidence() - ioc).abs()))
            .min_by(|first, second| first.1.total_cmp(&second.1))
            .unwrap()
            .0
    }

    pub fn index_of_coincidence(&self) -> f64 {
        match self {
            Self::English => 0.0667,
            Self::French => 0.0778,
            Self::German => 0.0762,
            Self::Italian => 0.0738,
            Self::Russian => 0.0529,
            Self::Spanish => 0.0770,
        }
    }
}

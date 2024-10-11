use crate::{
    character_set::{self, CharacterSet},
    Analyze,
};

pub enum CipherType {
    Transposition,
    Substitution,
    Base64,
    Morse,
    Hex,
    Octal,
}

impl CipherType {
    pub fn best_match(ciphertext: &str) -> Option<Self> {
        let raw = CharacterSet::raw(ciphertext);

        if character_set::MORSE.is_superset_of(&raw) {
            return Some(Self::Morse);
        }

        if character_set::OCTAL.is_superset_of(&raw) {
            return Some(Self::Octal);
        }

        if character_set::HEX.is_superset_of(&raw) {
            return Some(Self::Hex);
        }

        let alphanumeric = CharacterSet::of(ciphertext);

        if character_set::ALPHANUMERIC.is_superset_of(&alphanumeric) {
            let capitals = ciphertext.chars().filter(|char| char.is_uppercase()).count();
            let lowercase = ciphertext.chars().filter(|char| char.is_lowercase()).count();

            if (capitals as f64) < 0.1 * lowercase as f64 {
                if (0.6..0.75).contains(&ciphertext.index_of_coincidence()) {
                    return Some(Self::Transposition);
                } else {
                    return Some(Self::Substitution);
                }
            }
        }

        if character_set::BASE_64.is_superset_of(&alphanumeric) {
            return Some(Self::Base64);
        }

        None
    }
}

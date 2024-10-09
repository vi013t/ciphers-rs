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
        let characters = CharacterSet::of(ciphertext);

        if characters == *character_set::MORSE {
            return Some(Self::Morse);
        }

        if characters == *character_set::BASE_64 {
            return Some(Self::Base64);
        }

        if characters == *character_set::OCTAL {
            return Some(Self::Octal);
        }

        if characters == *character_set::HEX {
            return Some(Self::Hex);
        }

        if characters.is_alphabetic() {
            if (0.6..0.75).contains(&ciphertext.index_of_coincidence()) {
                return Some(Self::Transposition);
            } else {
                return Some(Self::Substitution);
            }
        }

        None
    }
}

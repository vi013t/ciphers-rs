use cipher_utils::score::PossiblePlaintext;
use gronsfeld::{Gronsfeld, GronsfeldBuilder};
use itertools::Itertools as _;

#[derive(Default)]
pub struct GronsfeldCracker {
    alphabet: Option<String>,
    key: Option<u128>,
    key_digits: Option<Vec<u128>>,
}

impl GronsfeldCracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decrypt(&self, ciphertext: &str) -> anyhow::Result<String> {
        let mut plaintexts: Vec<String> = Vec::new();
        if let Some(alphabet) = &self.alphabet {
            if let Some(key_digits) = &self.key_digits {
                for permutation in key_digits.iter().permutations(key_digits.len()).unique() {
                    let gronsfeld = Gronsfeld::new()
                        .alphabet(alphabet)
                        .key_str(&permutation.iter().map(|digit| digit.to_string()).collect::<String>())
                        .build()
                        .unwrap();
                    let plaintext = gronsfeld.decrypt(ciphertext)?;
                    plaintexts.push(plaintext);
                }
                return Ok(PossiblePlaintext::best(&plaintexts).unwrap());
            }
        }

        unreachable!()
    }

    pub fn with_known_alphabet(mut self, alphabet: &str) -> Self {
        self.alphabet = Some(alphabet.to_owned());
        self
    }

    pub fn with_known_key_digits(mut self, key_digits: &[u128]) -> Self {
        self.key_digits = Some(key_digits.to_vec());
        self
    }
}

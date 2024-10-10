use std::io::Write;

use cipher_utils::score::PossiblePlaintext;
use colored::Colorize;
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
        let mut plaintexts: Vec<(String, String)> = Vec::new();
        if let Some(alphabet) = &self.alphabet {
            // Alphabet and key are known

            // ALphabet and key digits are known
            if let Some(key_digits) = &self.key_digits {
                println!(
                    "\n{} {} with known alphabet and key digits...\n",
                    "Decrypting".bold().green(),
                    "Gronsfeld cipher".bold().cyan()
                );
                let total = key_digits.iter().permutations(key_digits.len()).unique().count();
                let mut iteration = 0;

                for permutation in key_digits.iter().permutations(key_digits.len()).unique() {
                    iteration += 1;

                    let key = permutation.iter().map(|digit| digit.to_string()).collect::<String>();
                    let gronsfeld = Gronsfeld::new().alphabet(alphabet).key_str(&key).build().unwrap();
                    let plaintext = gronsfeld.decrypt(ciphertext)?;

                    plaintexts.push((key, plaintext));

                    print!("\x1B[A");
                    println!(
                        "{} key permutations... {}",
                        "Brute forcing".bold().green(),
                        format!("{:.2}%", (100. * iteration as f64 / total as f64)).bold().yellow()
                    );
                    std::io::stdout().flush()?;
                }

                println!("{} best plaintext...", "Finding".bold().green());
                let best = plaintexts
                    .into_iter()
                    .sorted_by(|first, other| PossiblePlaintext::new(&first.1).cmp(&PossiblePlaintext::new(&other.1)))
                    .next_back()
                    .unwrap();

                println!("{} permutation: {}\n", "Best key".green().bold(), best.0.cyan().bold());
                return Ok(best.1);
            }
        }

        unimplemented!()
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

#[cfg(test)]
mod tests {
    use crate::GronsfeldCracker;

    #[test]
    fn decrypt_with_known_alphabet_and_key_digits() -> anyhow::Result<()> {
        let ciphertext = include_str!("../tests/encrypted_letter.txt");
        let plaintext = include_str!("../tests/letter.txt");

        let gronsfeld = GronsfeldCracker::new()
            .with_known_alphabet("AYCDWZIHGJKLQNOPMVSTXREUBF")
            .with_known_key_digits(&[1, 2, 3, 3, 4, 4, 8]);

        assert_eq!(plaintext, gronsfeld.decrypt(ciphertext)?);

        Ok(())
    }
}

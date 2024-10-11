use std::io::Write;

use cipher_utils::{score::PossiblePlaintext, Analyze};
use colored::Colorize;
use gronsfeld::{Gronsfeld, GronsfeldBuilder};
use itertools::Itertools;

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
        if let Some(alphabet) = &self.alphabet {
            // Alphabet and key are known

            // ALphabet and key digits are known
            if let Some(key_digits) = &self.key_digits {
                let mut plaintexts: Vec<(String, String)> = Vec::new();
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
                        if iteration == total {
                            format!("{:.2}%", (100. * iteration as f64 / total as f64)).bold().green()
                        } else {
                            format!("{:.2}%", (100. * iteration as f64 / total as f64)).bold().yellow()
                        }
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

            // No key digits known
            println!(
                "\t{} {} with known alphabet and no known key...",
                "Decrypting".bold().green(),
                "Gronsfeld cipher".bold().cyan()
            );
            let mut key_digits = 1;
            loop {
                let mut plaintexts: Vec<(String, String)> = Vec::new();
                println!("\t{} all keys with {} digits...\n", "Checking".bold().green(), key_digits.to_string().bold().cyan());
                let total = (0..10).permutations(key_digits as usize).unique().count();
                let mut iteration = 0;

                for permutation in (0..10).permutations(key_digits as usize).unique() {
                    iteration += 1;

                    let key = permutation.iter().map(|digit| digit.to_string()).collect::<String>();
                    let gronsfeld = Gronsfeld::new().alphabet(alphabet).key_str(&key).build().unwrap();
                    let plaintext = gronsfeld.decrypt(ciphertext)?;

                    plaintexts.push((key, plaintext));

                    print!("\x1B[A");
                    println!(
                        "\t\t{} key permutations... {}",
                        "Brute forcing".bold().green(),
                        if iteration == total {
                            format!("{:.2}%", (100. * iteration as f64 / total as f64)).bold().green()
                        } else {
                            format!("{:.2}%", (100. * iteration as f64 / total as f64)).bold().yellow()
                        }
                    );
                }

                println!("\t\t{} potential plaintexts...", "Scoring and sorting".bold().green(),);
                let (_best_key, best_plaintext) = plaintexts
                    .iter()
                    .max_by(|first, other| PossiblePlaintext::new(&first.1).cmp(&PossiblePlaintext::new(&other.1)))
                    .unwrap();

                println!("\t\t{} best plaintext...", "Quality checking".bold().green(),);
                if PossiblePlaintext::new(&best_plaintext).score() > 0.85 {
                    println!("\t\t{} plaintext found!", "Good quality".bold().green());
                    return Ok(best_plaintext.to_owned());
                }
                println!("\t\t{}. Increasing key digits by {} and repeating...", "Not good enough".bold().red(), "1".bold().cyan());

                key_digits += 1;
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

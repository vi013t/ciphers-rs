use std::borrow::Borrow;

use cipher_utils::alphabet::{Alphabet, AlphabetIndex};

pub fn tabula_recta<T: Borrow<Alphabet>>(alphabet: T) -> std::collections::HashMap<char, std::collections::HashMap<char, char>> {
    let mut rows = std::collections::HashMap::new();
    let alphabet = alphabet.borrow();
    for row in 1..=26 {
        let shifted = alphabet.shift(row - 1);
        rows.insert(
            *alphabet.letter_at(AlphabetIndex::new(row).unwrap()),
            alphabet
                .characters()
                .iter()
                .map(|alphabet_char| (*alphabet_char, *shifted.letter_at(alphabet.index_of(*alphabet_char).unwrap())))
                .collect(),
        );
    }
    rows
}

pub trait TabulaRecta {
    fn at(&self, row: &char, column: &char) -> Option<&char>;
}

impl TabulaRecta for std::collections::HashMap<char, std::collections::HashMap<char, char>> {
    fn at(&self, row_letter: &char, column_letter: &char) -> Option<&char> {
        self.get(row_letter).and_then(|column| column.get(column_letter))
    }
}

pub struct Vigenere {
    alphabet: Alphabet,
    key: String,
}

impl Vigenere {
    pub fn encrypt(&self, plaintext: &str) -> anyhow::Result<String> {
        let repeated_key = self.key.repeat(plaintext.len() / self.key.len());
        let key_bytes = repeated_key.as_bytes();
        let mut index = 0;
        plaintext
            .chars()
            .map(|plain_char| {
                if !plain_char.is_alphabetic() {
                    return Ok(plain_char);
                }
                let key_char = key_bytes[index] as char;
                let plaintext_index = self.alphabet.index_of(plain_char).unwrap();
                let key_index = self.alphabet.index_of(key_char).unwrap();
                let result = self.alphabet.letter_at(plaintext_index + key_index - 1);
                index += 1;
                Ok(if plain_char.is_uppercase() {
                    result.to_ascii_uppercase()
                } else {
                    result.to_ascii_lowercase()
                })
            })
            .collect()
    }

    pub fn decrypt(&self, ciphertext: &str) -> anyhow::Result<String> {
        let repeated_key = self.key.repeat(ciphertext.len() / self.key.len());
        let key_bytes = repeated_key.as_bytes();
        let mut index = 0;
        ciphertext
            .chars()
            .map(|cipher_char| {
                if !cipher_char.is_alphabetic() {
                    return Ok(cipher_char);
                }
                let key_char = key_bytes[index] as char;
                let ciphertext_index = self.alphabet.index_of(cipher_char).unwrap();
                let key_index = self.alphabet.index_of(key_char).unwrap();
                let result = self.alphabet.letter_at(ciphertext_index - key_index + 1);
                index += 1;
                Ok(if cipher_char.is_uppercase() {
                    result.to_ascii_uppercase()
                } else {
                    result.to_ascii_lowercase()
                })
            })
            .collect()
    }
}

pub trait VigenereBuilder {
    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl VigenereBuilder;
    fn key<T: AsRef<str>>(self, key: T) -> impl VigenereBuilder;
    fn build(self) -> anyhow::Result<Vigenere>;
}

#[derive(Debug, Default)]
struct IncompleteVigenere {
    key: Option<String>,
    alphabet: Option<Alphabet>,
}

impl VigenereBuilder for anyhow::Result<IncompleteVigenere> {
    fn key<T: AsRef<str>>(self, key: T) -> impl VigenereBuilder {
        if let Ok(mut vigenere) = self {
            vigenere.key = Some(key.as_ref().to_owned());
            Ok(vigenere)
        } else {
            self
        }
    }

    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl VigenereBuilder {
        if let Ok(mut vigenere) = self {
            vigenere.alphabet = Some(Alphabet::caseless(alphabet.as_ref())?);
            Ok(vigenere)
        } else {
            self
        }
    }

    fn build(self) -> anyhow::Result<Vigenere> {
        if let Ok(vigenere) = self {
            let Some(key) = vigenere.key else {
                anyhow::bail!("Error building Vigenere: No key provided.");
            };

            let Some(alphabet) = vigenere.alphabet else {
                anyhow::bail!("Error building Vigenere: No alphabet provided.");
            };

            Ok(Vigenere { alphabet, key })
        } else {
            Err(self.unwrap_err())
        }
    }
}

impl Vigenere {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl VigenereBuilder {
        Ok(IncompleteVigenere::default())
    }
}

#[cfg(test)]
mod tests {
    use cipher_utils::alphabet::{Alphabet, AlphabetIndex};

    use crate::{tabula_recta, TabulaRecta as _, Vigenere, VigenereBuilder};

    /// Run with `-- --nocapture` to avoid Rust suppressing the output.
    #[test]
    fn print_tabula_recta() {
        let alphabet = Alphabet::default();
        let tabula_recta = tabula_recta(&alphabet);

        for row in 1..=26 {
            for column in 1..=26 {
                let row_character = alphabet.letter_at(AlphabetIndex::new(row).unwrap());
                let column_character = alphabet.letter_at(AlphabetIndex::new(column).unwrap());
                print!("{}", tabula_recta.at(row_character, column_character).unwrap());
            }
            println!()
        }
    }

    #[test]
    fn encrypt_decrypt() -> anyhow::Result<()> {
        let plaintext = include_str!("../tests/letter.txt");
        let ciphertext = include_str!("../tests/encrypted_letter.txt");

        let vigenere = Vigenere::new().alphabet("AYCDWZIHGJKLQNOPMVSTXREUBF").key("MYSUPERTOPSECRETKEY").build()?;

        assert_eq!(ciphertext, vigenere.encrypt(plaintext)?);
        assert_eq!(plaintext, vigenere.decrypt(ciphertext)?);

        Ok(())
    }
}

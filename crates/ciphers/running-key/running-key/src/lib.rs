use cipher_utils::alphabet::Alphabet;
use vigenere_lib::{Vigenere, VigenereBuilder};

pub struct RunningKey {
    alphabet: Alphabet,
    key: String,
}

impl RunningKey {
    pub fn encrypt(&self, plaintext: &str) -> anyhow::Result<String> {
        if self.key.len() < plaintext.len() {
            anyhow::bail!("Error encrypting running-key cipher: Plaintext is shorter than key. If this is intentional, consider using a Vigenere cipher.");
        }
        let key_bytes = self.key.as_bytes();
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
        if self.key.len() < ciphertext.len() {
            anyhow::bail!("Error decrypting running-key cipher: Ciphertext is shorter than key. If this is intentional, consider using a Vigenere cipher.");
        }
        let key_bytes = self.key.as_bytes();
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

pub trait RunningKeyBuilder {
    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl RunningKeyBuilder;
    fn key<T: AsRef<str>>(self, key: T) -> impl RunningKeyBuilder;
    fn build(self) -> anyhow::Result<Vigenere>;
}

#[derive(Debug, Default)]
struct IncompleteRunningKey {
    key: Option<String>,
    alphabet: Option<Alphabet>,
}

impl RunningKeyBuilder for anyhow::Result<IncompleteRunningKey> {
    fn key<T: AsRef<str>>(self, key: T) -> impl RunningKeyBuilder {
        if let Ok(mut running_key) = self {
            running_key.key = Some(key.as_ref().to_owned());
            Ok(running_key)
        } else {
            self
        }
    }

    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl RunningKeyBuilder {
        if let Ok(mut running_key) = self {
            running_key.alphabet = Some(Alphabet::caseless(alphabet.as_ref())?);
            Ok(running_key)
        } else {
            self
        }
    }

    fn build(self) -> anyhow::Result<Vigenere> {
        if let Ok(running_key) = self {
            let Some(key) = running_key.key else {
                anyhow::bail!("Error building RunningKey: No key provided.");
            };

            let Some(alphabet) = running_key.alphabet else {
                anyhow::bail!("Error building RunningKey: No alphabet provided.");
            };

            Ok(Vigenere::new().alphabet(&alphabet.characters().iter().collect::<String>()).key(key).build().unwrap())
        } else {
            Err(self.unwrap_err())
        }
    }
}

impl RunningKey {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl RunningKeyBuilder {
        Ok(IncompleteRunningKey::default())
    }
}

use base64_cipher::Base64;
use cipher_utils::{alphabet::Alphabet, cipher_type::CipherType, score::PossiblePlaintext, Analyze};
use colored::Colorize;
use gronsfeld_cracker::GronsfeldCracker;
use morse_code_cipher::MorseCode;
use octal_cipher::OctalCipher;

#[derive(Default)]
pub struct CipherCracker {
    /// The key of the cipher to crack, if it's known.
    key: Option<String>,

    /// The alphabet of the cipher to crack, if it's known.
    alphabet: Option<Alphabet>,
}

impl CipherCracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_known_key<T: AsRef<str>>(mut self, key: T) -> Self {
        self.key = Some(key.as_ref().to_owned());
        self
    }

    pub fn with_known_alphabet<T: AsRef<str>>(mut self, key: T) -> anyhow::Result<Self> {
        self.alphabet = Some(Alphabet::caseless(key.as_ref())?);
        Ok(self)
    }

    pub fn crack(&self, ciphertext: &str) -> anyhow::Result<String> {
        println!("\n{} cipher...", "Cracking".bold().green());
        let cipher_type = CipherType::best_match(ciphertext).ok_or_else(|| anyhow::anyhow!("Unable to identify cipher type."))?;

        Ok(match cipher_type {
            CipherType::Octal => {
                println!("\t{} cipher type as {}.", "Identified".green().bold(), "octal".cyan().bold());
                println!("\t{} as {} encoding...", "Decrypting".bold().green(), "octal".cyan().bold());
                let plaintext = OctalCipher::decrypt(ciphertext)?;
                if plaintext.chars().all(|character| character.is_ascii()) {
                    println!(
                        "\t{} that {} decryption was successful.\n\t{} for additional encryption layers...",
                        "Detected".green().bold(),
                        "octal".cyan().bold(),
                        "Checking".green().bold()
                    );
                    self.check_for_encryption(&plaintext)?
                } else {
                    todo!()
                }
            }
            CipherType::Base64 => {
                println!("\t{} cipher type as {}.", "Identified".green().bold(), "base 64".cyan().bold());
                println!("\t{} as {} encoding...", "Decrypting".bold().green(), "base 64".cyan().bold());
                let plaintext = Base64::decrypt(ciphertext);

                // Successful Base64 decryption
                if plaintext.chars().all(|character| character.is_ascii()) {
                    println!(
                        "\t{} that {} decryption was successful.\n\t{} for additional encryption layers...",
                        "Detected".green().bold(),
                        "base 64".cyan().bold(),
                        "Checking".green().bold()
                    );
                    self.check_for_encryption(&plaintext)?
                }
                // Not regular Base64
                else {
                    todo!()
                }
            }
            CipherType::Morse => {
                println!("\t{} cipher type as {}.", "Identified".green().bold(), "morse code".cyan().bold());
                println!("\t{} as {} encoding...", "Decrypting".bold().green(), "morse code".cyan().bold());
                let plaintext = MorseCode::decrypt(ciphertext);

                // Successful Base64 decryption
                if plaintext.chars().all(|character| character.is_ascii()) {
                    println!(
                        "\t{} that {} decryption was successful.\n\t{} for additional encryption layers...",
                        "Detected".green().bold(),
                        "morse code".cyan().bold(),
                        "Checking".green().bold()
                    );
                    self.check_for_encryption(&plaintext)?
                }
                // Not regular Base64
                else {
                    todo!()
                }
            }
            CipherType::Substitution => match ciphertext.index_of_coincidence() {
                (0.04..=0.05) => GronsfeldCracker::new().with_known_alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ").decrypt(ciphertext)?,
                _ => todo!(),
            },
            _ => todo!(),
        })
    }

    fn check_for_encryption(&self, plaintext: &str) -> anyhow::Result<String> {
        let mut plaintext = plaintext.to_owned();
        while PossiblePlaintext::new(&plaintext).score() < 0.8 {
            println!(
                "\t{} that cipher has another layer of encryption. Running through another decryption pass...",
                "Detected".green().bold(),
            );
            plaintext = self.crack(&plaintext)?;
        }

        println!("{} additional encryption layers found. {}...\n", "No more".green().bold(), "Exiting".bold().cyan());
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use crate::CipherCracker;
    use base64_cipher::Base64;
    use cipher_utils::score::PossiblePlaintext;
    use gronsfeld::{Gronsfeld, GronsfeldBuilder};
    use morse_code_cipher::MorseCode;
    use octal_cipher::OctalCipher;

    static PLAINTEXT: &str = include_str!("../tests/letter.txt");
    static KEY: &str = "SUPERSECRETKEY";
    static NUMERIC_KEY: &str = "31824";

    #[test]
    fn base_64() -> anyhow::Result<()> {
        let ciphertext = Base64::encrypt(PLAINTEXT);
        println!();
        let plaintext = CipherCracker::new().crack(&ciphertext)?;
        assert_eq!(PLAINTEXT, plaintext);
        Ok(())
    }

    #[test]
    fn octal() -> anyhow::Result<()> {
        let ciphertext = OctalCipher::encrypt(PLAINTEXT);
        println!();
        let plaintext = CipherCracker::new().crack(&ciphertext)?;
        assert_eq!(PLAINTEXT, plaintext);
        Ok(())
    }

    #[test]
    fn morse_code() -> anyhow::Result<()> {
        let ciphertext = MorseCode::encrypt(PLAINTEXT);
        println!();
        let plaintext = CipherCracker::new().crack(&ciphertext)?;
        assert_eq!(PLAINTEXT.to_uppercase().replace(['\n', '\r'], ""), plaintext);
        Ok(())
    }

    #[test]
    fn gronsfeld() -> anyhow::Result<()> {
        let ciphertext = Gronsfeld::new().alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ").key_str(NUMERIC_KEY).build()?.encrypt(PLAINTEXT)?;

        println!();
        let plaintext = CipherCracker::new().with_known_alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ")?.crack(&ciphertext)?;

        assert_eq!(PLAINTEXT, plaintext);
        Ok(())
    }
}

pub mod analysis {
    pub use cipher_utils::*;
}

#[cfg(feature = "enigma")]
pub mod enigma {
    pub use enigma_cracker::*;
}

#[cfg(feature = "gronsfeld")]
pub mod gronsfeld {
    pub use gronsfeld_cracker::*;
}

#[cfg(feature = "morse-code")]
pub mod morse_code {
    pub use morse_code_cipher::*;
}

#[cfg(feature = "octal")]
pub mod octal {
    pub use octal_cipher::*;
}

#[cfg(feature = "base64")]
pub mod base64 {
    pub use base64_cipher::*;
}

#[cfg(feature = "vigenere")]
pub mod vigenere {
    pub use vigenere_cracker::*;
}

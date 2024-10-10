use cipher_utils::{alphabet::Alphabet, cipher_type::CipherType};
use colored::Colorize;
use octal_cipher::OctalCipher;

#[derive(Default)]
pub struct CipherCracker {
    key: Option<String>,
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

    pub fn crack(ciphertext: &str) -> anyhow::Result<String> {
        println!("{} cipher...", "Cracking".bold().green());
        let cipher_type = CipherType::best_match(ciphertext).ok_or_else(|| anyhow::anyhow!("Unable to identify cipher type."))?;

        match cipher_type {
            CipherType::Octal => {
                println!("\t{} cipher type as {}.", "Identified".green().bold(), "octal".green().bold());
                let plaintext = OctalCipher::decrypt(ciphertext)?;
                println!("{} decrypted.", "Cipher".green().bold());
                Ok(plaintext)
            }
            CipherType::Base64 => {
                println!("\t{} cipher type as {}.", "Identified".green().bold(), "base 64".green().bold());
                let plaintext = Base64Cipher::decrypt()?;

                if plaintext.chars().all(|character| character.is_ascii()) {}

                println!(
                    "\t{} that cipher is not just plain {base64}. Checking for {base64} + {}.",
                    "Detected".green().bold(),
                    "transposition".cyan().bold(),
                    base64 = "base 64".cyan().bold(),
                );

                Ok(plaintext)
            }
            _ => todo!(),
        }
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

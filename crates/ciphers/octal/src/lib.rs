use std::fmt::Write as _;

pub struct OctalCipher;

impl OctalCipher {
    pub fn decrypt(ciphertext: &str) -> anyhow::Result<String> {
        ciphertext
            .split_whitespace()
            .map(|octal| Ok(u8::from_str_radix(octal, 8).map(|code| code as char)?))
            .collect()
    }

    pub fn encrypt(plaintext: &str) -> String {
        plaintext
            .chars()
            .fold(String::new(), |mut accumulator, current| {
                write!(accumulator, " {:03o}", current as u8).unwrap();
                accumulator
            })
            .get(1..)
            .unwrap()
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::OctalCipher;

    #[test]
    fn encrypt_decrypt() -> anyhow::Result<()> {
        let ciphertext = include_str!("../tests/letter_octal.txt");
        let plaintext = include_str!("../tests/letter.txt");

        assert_eq!(ciphertext, OctalCipher::encrypt(plaintext));
        assert_eq!(plaintext, OctalCipher::decrypt(ciphertext)?);

        Ok(())
    }
}

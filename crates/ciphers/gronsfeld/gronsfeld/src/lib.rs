use cipher_utils::alphabet::Alphabet;

pub struct Gronsfeld {
    alphabet: Alphabet,
    key: u128,
}

impl Gronsfeld {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl GronsfeldBuilder {
        Ok(IncompleteGronsfeld::default())
    }

    pub fn encrypt(&self, plaintext: &str) -> anyhow::Result<String> {
        let key = self.key.to_string().repeat(plaintext.len() / self.key.to_string().len());

        let mut index = 0;
        plaintext
            .chars()
            .map(|letter| {
                if !letter.is_alphabetic() {
                    return Ok(letter);
                }

                let key_digit = key.chars().nth(index).unwrap();

                let alphabet_index = self.alphabet.index_of(letter).ok_or_else(|| anyhow::anyhow!("Character not in alphabet: {letter}"))?;
                let mut ciphertext_letter = *self.alphabet.letter_at(alphabet_index + key_digit.to_digit(10).unwrap());
                if letter.is_lowercase() {
                    ciphertext_letter = ciphertext_letter.to_ascii_lowercase();
                }
                index += 1;
                Ok(ciphertext_letter)
            })
            .collect::<anyhow::Result<String>>()
    }

    pub fn decrypt(&self, ciphertext: &str) -> anyhow::Result<String> {
        let key = self.key.to_string().repeat(ciphertext.len() / self.key.to_string().len());

        let mut index = 0;
        ciphertext
            .chars()
            .map(|ciphertext_letter| {
                if !ciphertext_letter.is_alphabetic() {
                    return Ok(ciphertext_letter);
                }

                let key_digit = key.chars().nth(index).unwrap();

                let alphabet_index = self
                    .alphabet
                    .index_of(ciphertext_letter)
                    .ok_or_else(|| anyhow::anyhow!("Character not in alphabet: {ciphertext_letter}"))?;
                index += 1;
                let mut plaintext_character = *self.alphabet.letter_at(alphabet_index - key_digit.to_digit(10).unwrap());
                if ciphertext_letter.is_lowercase() {
                    plaintext_character = plaintext_character.to_ascii_lowercase();
                }
                Ok(plaintext_character)
            })
            .collect::<anyhow::Result<String>>()
    }
}

#[derive(Default, Debug)]
struct IncompleteGronsfeld {
    alphabet: Option<Alphabet>,
    key: Option<u128>,
}

pub trait GronsfeldBuilder {
    fn alphabet(self, alphabet: &str) -> Self;
    fn key(self, key: u128) -> Self;
    fn key_str(self, key: &str) -> Self;
    fn build(self) -> anyhow::Result<Gronsfeld>;
}

impl GronsfeldBuilder for anyhow::Result<IncompleteGronsfeld> {
    fn alphabet(self, alphabet: &str) -> Self {
        if let Ok(mut gronsfeld) = self {
            gronsfeld.alphabet = Some(Alphabet::caseless(alphabet)?);
            Ok(gronsfeld)
        } else {
            self
        }
    }

    fn key(self, key: u128) -> Self {
        if let Ok(mut gronsfeld) = self {
            gronsfeld.key = Some(key);
            Ok(gronsfeld)
        } else {
            self
        }
    }

    fn key_str(self, key: &str) -> Self {
        if let Ok(mut gronsfeld) = self {
            gronsfeld.key = Some(key.parse()?);
            Ok(gronsfeld)
        } else {
            self
        }
    }

    fn build(self) -> anyhow::Result<Gronsfeld> {
        if let Ok(gronsfeld) = self {
            let Some(alphabet) = gronsfeld.alphabet else {
                anyhow::bail!("Error constructing Gronsfeld cipher: No alphabet set");
            };
            let Some(key) = gronsfeld.key else {
                anyhow::bail!("Error constructing Gronsfeld cipher: No key set");
            };

            Ok(Gronsfeld { alphabet, key })
        } else {
            Err(self.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Gronsfeld, GronsfeldBuilder as _};

    #[test]
    fn encrypt_decrypt() -> anyhow::Result<()> {
        let ciphertext = "Xtbae hvxaf gvpxe xge jrfu, gppxflbfude czjblriqok bopb, raj wm bjiutsj xbssua jvghzgiise yf qcavwy fu jpnmbz rdqty cnjrcd. Yf upgt jg tkths tfeldx, sbgx muuefdw befwhjuixgmm imowedm ndhtbkb ogxj ib dpqnfgs zf fw gpssvqt zsttboajb. Iyqt cfez lbyyu zmoua jv yuvufmymhcegr je evpdmrcez efpqx bxrz hjpvbs zvxtba cb yflpze vdqnc sjajwfbu. Bulysucbu xjeb vigybwdc hatqwcrdc svv remgizse, edor gm lcoti nfg vgwjiqy zbrzaavf vmnopb dvqv gz fyb owwpuft.";
        let plaintext = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let alphabet = "AYCDWZIHGJKLQNOPMVSTXREUBF";
        let key = 953461223;

        let gronsfeld = Gronsfeld::new().alphabet(alphabet).key(key).build()?;

        assert_eq!(ciphertext, gronsfeld.encrypt(plaintext)?);
        assert_eq!(plaintext, gronsfeld.decrypt(ciphertext)?);

        Ok(())
    }
}

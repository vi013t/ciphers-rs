use itertools::Itertools as _;

pub struct Base64;

const CHARACTERS: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

impl Base64 {
    pub fn encrypt(plaintext: &str) -> String {
        plaintext
            .chars()
            .chunks(3)
            .into_iter()
            .map(|triplet| {
                let mut quadruplet = triplet
                    .map(|character| format!("{:08b}", character as u8))
                    .join("")
                    .chars()
                    .chunks(6)
                    .into_iter()
                    .map(|chunk| {
                        let mut string = chunk.collect::<String>();
                        while string.len() < 6 {
                            string = string + "0";
                        }
                        (*CHARACTERS.get(usize::from_str_radix(&string, 2).unwrap()).unwrap() as char).to_string()
                    })
                    .collect::<String>();
                while quadruplet.len() % 4 != 0 {
                    quadruplet += "=";
                }
                quadruplet
            })
            .collect()
    }

    pub fn decrypt(ciphertext: &str) -> String {
        ciphertext
            .chars()
            .filter(|character| !character.is_whitespace())
            .chunks(4)
            .into_iter()
            .map(|quadruplet| {
                quadruplet
                    .map(|character| {
                        if character == '=' {
                            "2".to_owned()
                        } else {
                            format!("{:06b}", CHARACTERS.iter().position(|other| *other as char == character).unwrap())
                        }
                    })
                    .join("")
                    .chars()
                    .chunks(8)
                    .into_iter()
                    .filter_map(|chunk| {
                        let string = chunk.collect::<String>().trim_end_matches("2").to_owned();
                        (string.len() == 8).then_some(u8::from_str_radix(&string, 2).unwrap() as char)
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Base64;

    #[test]
    fn encrypt_decrypt() {
        let letter = include_str!("../tests/letter.txt").trim().replace("\r", "");
        let encrypted_letter = include_str!("../tests/encrypted_letter.txt").trim().replace("\r", "");

        let ciphertext = Base64::encrypt(&letter);
        let plaintext = Base64::decrypt(&encrypted_letter);

        assert_eq!(letter, plaintext);
        assert_eq!(encrypted_letter, ciphertext);
    }
}

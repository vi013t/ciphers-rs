use itertools::Itertools as _;

pub struct ColumnarTransposition {
    key: Vec<u8>,
}

impl ColumnarTransposition {
    pub fn new<T: AsRef<str>>(key: T) -> Self {
        Self {
            key: key.as_ref().as_bytes().to_vec(),
        }
    }

    pub fn from_key_digits(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut columns = vec![Vec::new(); self.key.len()];
        for (index, character) in plaintext.char_indices() {
            columns[index % self.key.len()].push(character);
        }

        columns
            .iter()
            .enumerate()
            .sorted_by(|left, right| self.key.get(left.0).cmp(&self.key.get(right.0)))
            .map(|column| column.1.iter().map(|letter| letter.to_string()).join(""))
            .join("")
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ColumnarTransposition;

    #[test]
    fn encrypt_decrypt() {
        let ciphertext = include_str!("../tests/encrypted_letter.txt");
        let plaintext = include_str!("../tests/letter.txt");
        let key = &[1, 0, 2, 7, 1, 9, 7, 9];

        let columnar_transposition = ColumnarTransposition::from_key_digits(key);

        assert_eq!(ciphertext, columnar_transposition.encrypt(plaintext));
        //assert_eq!(plaintext, columnar_transposition.decrypt(ciphertext));
    }
}

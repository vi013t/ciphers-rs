pub struct Caeser {
    shift: u8,
}

impl Caeser {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {}
}

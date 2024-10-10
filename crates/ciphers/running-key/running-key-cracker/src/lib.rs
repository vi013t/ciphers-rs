pub struct RunningKeyCracker;

impl RunningKeyCracker {
    pub fn decrypt(ciphertext: &str) -> String {
        if ciphertext.contains(' ') {
            let words = ciphertext.split_whitespace().collect::<Vec<_>>();
        }
        todo!()
    }
}

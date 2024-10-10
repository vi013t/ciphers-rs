use rand::Rng;
use vigenere_lib::{Vigenere, VigenereBuilder};

/// A one-time-pad, which can encrypt messages uncrackably.
pub struct OneTimePad;

impl OneTimePad {
    /// Encrypts the given plaintext with a randomly generated key as long as the plaintext.
    /// By definition of a one-time-pad, which is provably uncrackable, each key can only be
    /// used once, so the key generated is never available to the user. Instead, a decryptor
    /// is returned, which can be used to decrypt the returned ciphertext back into the orignal
    /// plaintext exactly one time.
    ///
    /// # Parameters
    /// - `plaintext` - The plaintext to encrypt
    ///
    /// # Returns
    /// The encrypted ciphertext, and a decryptor that can be used once to decrypt the ciphertext
    /// back into the original plaintext.
    pub fn encrypt(plaintext: &str) -> (String, OneTimePadDecryptor) {
        let key = plaintext.chars().map(|_| rand::thread_rng().gen_range(65u8..=90u8) as char).collect::<String>();
        let vigenere = Vigenere::new().key(&key).alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ").build().unwrap();
        let ciphertext = vigenere.encrypt(plaintext);
        let decryptor = OneTimePadDecryptor { key };
        (ciphertext, decryptor)
    }
}

/// A one-time-pad decryptor, which can be used to decrypt a message with a key passed
/// from an encryptor. By definition of a one-time-pad, which is provably uncrackable,
/// the key can only be used once; Thus, decrypting the given message will consume
/// this decryptor.
pub struct OneTimePadDecryptor {
    key: String,
}

impl OneTimePadDecryptor {
    /// Decrypts the given ciphertext with this decryptor's key. By definition of a
    /// one-time-pad, which is provably uncrackable, each key can only be used once;
    /// Thus, this method consumes `self` to prevent further use.
    ///
    /// # Parameters
    /// - `ciphertext` - The ciphertext to decrypt
    ///
    /// # Returns
    /// The decrypted message.
    pub fn decrypt(self, ciphertext: &str) -> String {
        let vigenere = Vigenere::new().key(self.key).alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ").build().unwrap();
        vigenere.decrypt(ciphertext)
    }
}

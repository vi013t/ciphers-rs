use crate::alphabet::Alphabet;

pub struct Base64;

lazy_static::lazy_static! {
    pub static ref ALPHABET: Alphabet = Alphabet::of_cased("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

//! An absurdly fast and highly flexible Enigma machine simulation, encryption, and decryption library.

mod alphabet;
mod enigma;
mod reflector;
mod rotor;
mod safe_enigma;
mod unsafe_enigma;

pub use crate::safe_enigma::*;
pub use crate::unsafe_enigma::*;

/// The result type returned from enigma functions.
pub type EnigmaResult<T> = anyhow::Result<T>;

#[derive(Default)]
pub struct MachineOptions {
    pub clear_casing: bool,
    pub clear_punctuation: bool,

    /// Whether to print debug information during encryption/decryption. If this is set to `true`, then at each stage of encryption,
    /// the machine will print information about the current character and how it is being transformed. For example:
    ///
    /// ```rust
    /// let ciphertext = "HI";
    /// let machine = EnigmaMachine::new()
    /// 	.rotors(1, 2, 3)
    ///		.reflector("B")
    ///		.ring_settings(10, 12, 14)
    /// 	.ring_positions(5, 22, 3)
    /// 	.plugboard("BY EW FZ GI QM RV UX")
    /// 	 .debug()?;
    ///
    /// machine.decrypt(ciphertext);
    /// ```
    ///
    /// will convert "HI" to "IJ", and will print:
    ///
    /// ```
    /// Decrypting character: 'H'
    /// 	Passing character through plugboard: 'H' -> 'H'
    /// 	Passing character through third rotor: 'H' -> 'C'
    /// 	Passing character through second rotor: 'C' -> 'M'
    /// 	Passing character through first rotor: 'M' -> 'V'
    /// 	Passing character through reflector: 'V' -> 'W'
    /// 	Passing character back through first rotor: 'W' -> 'C'
    /// 	Passing character back through second rotor: 'C' -> 'E'
    /// 	Passing character back through third rotor: 'E' -> 'G'
    /// 	Passing character back through plugboard: 'G' -> 'I'
    /// 	Finalized character: 'I'
    /// Decrypting character: 'I'
    /// 	Passing character through plugboard: 'I' -> 'G'
    /// 	Passing character through third rotor: 'G' -> 'B'
    /// 	Passing character through second rotor: 'B' -> 'X'
    /// 	Passing character through first rotor: 'X' -> 'X'
    /// 	Passing character through reflector: 'X' -> 'J'
    /// 	Passing character back through first rotor: 'J' -> 'F'
    /// 	Passing character back through second rotor: 'F' -> 'K'
    /// 	Passing character back through third rotor: 'K' -> 'J'
    /// 	Passing character back through plugboard: 'J' -> 'J'
    /// 	Finalized character: 'J'
    /// ```
    pub debug: bool,
}

/// Performs a caeser shift on the given text by the given amount. This is used during the
/// decryption process for the Enigma machine. Specifically, this will shift each character
/// by taking its ascii value `code` and getting the character value of `((code - 65 + amount) % 26) + 65`
/// if and only if `code` is in `[65, 90)`. If it's not, the character is left unchanged.
///
/// # Parameters
/// - `text` - The text to shift
/// - `amount` The amount to shift the text by
///
/// # Returns
/// The caeser shifted text.
pub fn caeser_shift(text: &str, amount: u8) -> String {
    text.chars()
        .map(|letter| {
            let code = letter as u8;
            if (65..=90).contains(&code) {
                (((code - 65 + amount) % 26) + 65) as char
            } else {
                letter
            }
        })
        .collect()
}

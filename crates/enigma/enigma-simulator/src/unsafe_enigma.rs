use crate::{
    alphabet::{Alphabet, ALPHABET},
    enigma::caeser_shift,
    reflector::Reflector,
    rotor::{IntoRotors as _, Rotor},
};

pub struct UncheckedEnigmaMachine {
    pub(crate) rotors: (Rotor, Rotor, Rotor),
    pub(crate) ring_positions: (u8, u8, u8),
    pub(crate) ring_settings: (u8, u8, u8),
    pub(crate) reflector: &'static std::collections::HashMap<char, char>,
    pub(crate) plugboard: std::collections::HashMap<char, char>,
}

impl UncheckedEnigmaMachine {
    /// Decrypts the given ciphertext while skipping all safety checks for maxmimum speed.
    ///
    /// This is exactly the same as calling `machine.encrypt_unchecked(text)`, since the enigma cipher is
    /// symmetric; The only difference is semantic meaning and intent, i.e.,
    ///
    /// ```rust
    ///	assert_eq!(text, machine.decrypt_unchecked(machine.decrypt_unchecked(text)));
    ///	assert_eq!(text, machine.encrypt_unchecked(machine.encrypt_unchecked(text)));
    ///	assert_eq!(text, machine.decrypt_unchecked(machine.encrypt_unchecked(text)));
    ///	assert_eq!(text, machine.encrypt_unchecked(machine.decrypt_unchecked(text)));
    /// ```
    ///
    /// # Parameters
    /// - `text` - The ciphertext to decrypt
    ///
    /// # Returns
    /// The decrypted plaintext.
    ///
    /// # Safety
    /// This function may panic if the Enigma machine was constructed with invalid settings. If the machine was constructed
    /// correctly, this is guarnateed to not panic, and will function identically to `EnigmaMachine::decrypt()`, with
    /// marginally better performance. See the `Performance` section of the `README` for more information.
    pub unsafe fn decrypt_unchecked(&self, text: &str) -> String {
        let text = text.to_uppercase();
        let rotor_a = self.rotors.0.alphabet();
        let rotor_b = self.rotors.1.alphabet();
        let rotor_c = self.rotors.2.alphabet();

        let mut rotor_a_letter = self.ring_positions.0;
        let mut rotor_b_letter = self.ring_positions.1;
        let mut rotor_c_letter = self.ring_positions.2;

        let rotor_a_setting = self.ring_settings.0;
        let offset_a_setting = rotor_a_setting;
        let rotor_b_setting = self.ring_settings.1;
        let offset_b_setting = rotor_b_setting;
        let rotor_c_setting = self.ring_settings.2;
        let offset_c_setting = rotor_c_setting;

        let rotor_a = caeser_shift(&rotor_a.letters(), offset_a_setting);
        let rotor_b = caeser_shift(&rotor_b.letters(), offset_b_setting);
        let rotor_c = caeser_shift(&rotor_c.letters(), offset_c_setting);

        let rotor_a_first_half = unsafe { rotor_a.get_unchecked((26 - offset_a_setting as usize)..rotor_a.len()) }.to_owned();
        let rotor_a_second_half = unsafe { rotor_a.get_unchecked(0..(26 - offset_a_setting as usize)) }.to_owned();
        let rotor_a = rotor_a_first_half + &rotor_a_second_half;
        let rotor_a = Alphabet::new_unchecked(&rotor_a);

        let rotor_b_first_half = unsafe { rotor_b.get_unchecked((26 - offset_b_setting as usize)..rotor_b.len()) }.to_owned();
        let rotor_b_second_half = unsafe { rotor_b.get_unchecked(0..(26 - offset_b_setting as usize)) }.to_owned();
        let rotor_b = rotor_b_first_half + &rotor_b_second_half;
        let rotor_b = Alphabet::new_unchecked(&rotor_b);

        let rotor_c_first_half = unsafe { rotor_c.get_unchecked((26 - offset_c_setting as usize)..rotor_c.len()) }.to_owned();
        let rotor_c_second_half = unsafe { rotor_c.get_unchecked(0..(26 - offset_c_setting as usize)) }.to_owned();
        let rotor_c = rotor_c_first_half + &rotor_c_second_half;
        let rotor_c = Alphabet::new_unchecked(&rotor_c);

        text.chars()
            .map(|mut letter| {
                // Non-alphabetic characters stay the same
                if !letter.is_alphabetic() {
                    return letter.to_string();
                }

                // Rotate rotor 3
                let mut rotor_trigger = self
                    .rotors
                    .2
                    .notches()
                    .iter()
                    .map(|notch| ALPHABET.unchecked_index_of(*notch))
                    .collect::<Vec<_>>()
                    .contains(&rotor_c_letter);
                rotor_c_letter = (rotor_c_letter + 1) % 26;

                // Rotate rotor 2
                if rotor_trigger {
                    rotor_trigger = self
                        .rotors
                        .1
                        .notches()
                        .iter()
                        .map(|notch| ALPHABET.unchecked_index_of(*notch))
                        .collect::<Vec<_>>()
                        .contains(&rotor_b_letter);
                    rotor_b_letter = (rotor_b_letter + 1) % 26;

                    // Rotate rotor 1
                    if rotor_trigger {
                        rotor_a_letter = (rotor_a_letter + 1) % 26;
                    }
                }
                // Double step sequence
                else if self
                    .rotors
                    .1
                    .notches()
                    .iter()
                    .map(|notch| ALPHABET.unchecked_index_of(*notch))
                    .collect::<Vec<_>>()
                    .contains(&rotor_b_letter)
                {
                    rotor_b_letter = (rotor_b_letter + 1) % 26;
                    rotor_a_letter = (rotor_a_letter + 1) % 26;
                }

                // Plugboard decryption
                if let Some(plugboarded_letter) = self.plugboard.get(&letter) {
                    letter = *plugboarded_letter;
                }

                let offset_a = rotor_a_letter;
                let offset_b = rotor_b_letter;
                let offset_c = rotor_c_letter;

                // Rotor 3 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = rotor_c.unchecked_letter_at((pos + offset_c) % 26);
                let pos = ALPHABET.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_c) % 26);

                // Rotor 2 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = rotor_b.unchecked_letter_at((pos + offset_b) % 26);
                let pos = ALPHABET.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_b) % 26);

                // Rotor 1 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = rotor_a.unchecked_letter_at((pos + offset_a) % 26);
                let pos = ALPHABET.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_a) % 26);

                // Reflector Encryption
                letter = *self.reflector.get(&letter).unwrap();

                // Rotor 1 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = ALPHABET.unchecked_letter_at((pos + offset_a) % 26);
                let pos = rotor_a.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_a) % 26);

                // Rotor 2 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = ALPHABET.unchecked_letter_at((pos + offset_b) % 26);
                let pos = rotor_b.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_b) % 26);

                // Rotor 3 Encryption
                let pos = ALPHABET.unchecked_index_of(letter);
                let let_ = ALPHABET.unchecked_letter_at((pos + offset_c) % 26);
                let pos = rotor_c.unchecked_index_of(let_);
                letter = ALPHABET.unchecked_letter_at((pos + 26 - offset_c) % 26);

                // Plugboard Second Pass
                if let Some(plugboarded_letter) = self.plugboard.get(&letter) {
                    letter = *plugboarded_letter;
                }

                letter.to_string()
            })
            .collect()
    }

    /// Encrypts the given plaintext while skipping all safety checks for maxmimum speed.
    ///
    /// This is exactly the same as calling `machine.decrypt_unchecked(text)`, since the enigma cipher is
    /// symmetric; The only difference is semantic meaning and intent, i.e.,
    ///
    /// ```rust
    ///	assert_eq!(text, machine.decrypt_unchecked(machine.decrypt_unchecked(text)));
    ///	assert_eq!(text, machine.encrypt_unchecked(machine.encrypt_unchecked(text)));
    ///	assert_eq!(text, machine.decrypt_unchecked(machine.encrypt_unchecked(text)));
    ///	assert_eq!(text, machine.encrypt_unchecked(machine.decrypt_unchecked(text)));
    /// ```
    ///
    /// # Parameters
    /// - `text` - The ciphertext to decrypt
    ///
    /// # Returns
    /// The decrypted plaintext.
    ///
    /// # Safety
    /// This function may panic if the Enigma machine was constructed with invalid settings. If the machine was constructed
    /// correctly, this is guarnateed to not panic, and will function identically to `EnigmaMachine::decrypt()`, with
    /// marginally better performance. See the `Performance` section of the `README` for more information.
    pub unsafe fn encrypt_unchecked(&self, text: &str) -> String {
        self.decrypt_unchecked(text)
    }
}

pub trait UncheckedEnigmaBuilder {
    /// Sets the rotors for the machine.
    ///
    /// # Parameters
    /// - `first` - The first rotor to use
    /// - `second` - The second rotor to use
    /// - `third` - The third rotor to use
    ///
    /// # Returns
    /// The machine builder with the given ring settings applied.
    ///
    /// # Panics
    /// If the given numbers are not all in `[1, 26]`, an error is returned.
    fn rotors(self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder;

    /// Sets the plugboard for the machine. The given plugboard should be a space-separated string of letter pairs. This is automatically
    /// bidirectional, meaning the pair `AY` will map `A` to `Y` and also `Y` to `A`.
    ///
    /// # Parameters
    /// - `plugboard` - A space-separated string of letter pairs, i.e., `AY BF QR UX GZ`.
    ///
    /// # Returns
    /// The machine builder with the given plugboard applied.
    ///
    /// # Panics
    /// If the given plugboard is not formatted correctly, this function may, but is not guarnateed
    /// to, panic. If the given plugboard contains duplicates, this function will not panic, and
    /// instead the later duplicate will overwrite the earlier one.
    fn plugboard(self, plugboard: &str) -> impl UncheckedEnigmaBuilder;

    /// Sets the reflector of the machine.
    ///
    /// # Parameters
    /// - `reflector` - The reflector to give the machine.
    ///
    /// # Returns
    /// The machine builder with the given ring settings applied.
    ///
    /// # Panics
    /// If the given reflector string does not represent an existing reflector. **Note that for
    /// unchecked machines such as this, the reflector given is case-sensitive.** This makes the
    /// function more performant by not converting casing to allow case insensitivity. The given
    /// reflector thus *must* be one of the following *exactly*:
    ///
    /// - `A`
    /// - `B`
    /// - `C`
    /// - `BThin`
    /// - `CThin`
    /// - `UKWK`
    /// - `UKWR`
    fn reflector(self, reflector: &str) -> impl UncheckedEnigmaBuilder;

    /// Sets the ring settings of the machine, unchecked.
    ///
    /// # Parameters
    /// - `first` - The first ring setting, in `[1, 26]`.
    /// - `second` - The second ring setting, in `[1, 26]`.
    /// - `third` - The third ring setting, in `[1, 26]`.
    ///
    /// # Returns
    /// The machine builder with the given ring settings applied.
    ///
    /// # Panics
    /// Even if the given numbers are not all in `[1, 26]`, this function will not panic; The
    /// machine will simply be created with invalid settings. The machine will most likely panic
    /// during encryption/decryption, or possibly will just produce an incorrect output.
    fn ring_settings(self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder;

    /// Sets the "ring positions" or "rotor positions" of the machine, unchecked.
    ///
    /// # Parameters
    /// - `first` - The offset of the first rotor, in `[1, 26]`.
    /// - `second` - The offset of the second rotor, in `[1, 26]`.
    /// - `third` - The offset of the third rotor, in `[1, 26]`.
    ///
    /// # Returns
    /// The machine builder with the given rotor positions applied.
    ///
    /// # Errors
    /// Even if the given numbers are not all in `[1, 26]`, this function will not panic; The
    /// machine will simply be created with invalid settings. The machine will most likely panic
    /// during encryption/decryption, or possibly will just produce an incorrect output.
    fn ring_positions(self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder;

    /// Finalizes building the Enigma machine. This is an absolutely zero-cost method that simply
    /// converts the `impl UncheckedEnigmaBuilder` opaque type into the concrete `EnigmaMachine`
    /// by just returning `self`.
    fn build(self) -> UncheckedEnigmaMachine;
}

impl UncheckedEnigmaBuilder for UncheckedEnigmaMachine {
    fn rotors(mut self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder {
        self.rotors = (first, second, third).unchecked_into_rotors();
        self
    }

    fn ring_positions(mut self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder {
        self.ring_positions = (first - 1, second - 1, third - 1);
        self
    }

    fn ring_settings(mut self, first: u8, second: u8, third: u8) -> impl UncheckedEnigmaBuilder {
        self.ring_settings = (first - 1, second - 1, third - 1);
        self
    }

    fn plugboard(mut self, plugboard: &str) -> impl UncheckedEnigmaBuilder {
        let mappings = plugboard.split_whitespace();
        let mut plugboard = std::collections::HashMap::new();
        for pair in mappings {
            let mut chars = pair.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            plugboard.insert(first, second);
            plugboard.insert(second, first);
        }

        self.plugboard = plugboard;
        self
    }

    fn reflector(mut self, reflector: &str) -> impl UncheckedEnigmaBuilder {
        self.reflector = Reflector::unchecked_from(reflector).alphabet();
        self
    }

    fn build(self) -> UncheckedEnigmaMachine {
        self
    }
}

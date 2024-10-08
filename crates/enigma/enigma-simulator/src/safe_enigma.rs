use colored::Colorize as _;

use crate::{
    alphabet::{Alphabet, AlphabetIndex, IntoAlphabetIndex as _, ALPHABET},
    enigma::{caeser_shift, MachineOptions},
    reflector::Reflector,
    rotor::{IntoRotors as _, Rotor},
    UncheckedEnigmaBuilder, UncheckedEnigmaMachine,
};

/// An enigma machine with applied settings that can encrypt or decrypt text.
pub struct EnigmaMachine {
    rotors: (Rotor, Rotor, Rotor),
    ring_positions: (AlphabetIndex, AlphabetIndex, AlphabetIndex),
    ring_settings: (AlphabetIndex, AlphabetIndex, AlphabetIndex),
    reflector: Reflector,
    plugboard: std::collections::HashMap<char, char>,
    options: MachineOptions,
}

impl EnigmaMachine {
    /// Creates a new Enigma machine with blank settings. The settings for the machine must be added using the methods
    /// of `EnigmaBuilder`; See the README for an example.
    ///
    /// The returned value from this will always be `Ok`, and will be an Enigma machine with rotors 1, 1, 1, ring positions
    /// 1, 1, and 1, ring settings 1, 1, and 1, reflector A, and an empty plugboard.
    ///
    /// This opts you into the safe and Rusty API; To use the unsafe maximum performance API, use
    /// `EnigmaMachine::unchecked()`. See the `README` for more information.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl EnigmaBuilder {
        Ok(Self {
            rotors: (1, 1, 1).try_into_rotors().unwrap(),
            ring_positions: (1, 1, 1).try_into_alphabet_index().unwrap(),
            ring_settings: (1, 1, 1).try_into_alphabet_index().unwrap(),
            reflector: Reflector::A,
            plugboard: std::collections::HashMap::new(),
            options: MachineOptions::default(),
        })
    }

    /// Creates a new Enigma machine with blank settings. The settings for the machine must be added using the methods
    /// of `EnigmaBuilder`; See the README for an example.
    ///
    /// The returned value from this will always be `Ok`, and will be an Enigma machine with rotors 1, 1, 1, ring positions
    /// 1, 1, and 1, ring settings 1, 1, and 1, reflector A, and an empty plugboard.
    ///
    /// This opts you into the unsafe API; To use the safer API at the expense of some performance, use
    /// `EnigmaMachine::new()`. See the `README` for more information.
    pub fn unchecked() -> impl UncheckedEnigmaBuilder {
        UncheckedEnigmaMachine {
            rotors: (Rotor::I, Rotor::I, Rotor::I),
            ring_positions: (1, 1, 1),
            ring_settings: (1, 1, 1),
            reflector: Reflector::A.alphabet(),
            plugboard: std::collections::HashMap::new(),
        }
    }

    /// Decodes the given text using this Enigma machine.
    ///
    /// The decryption process does the following for each letter in the ciphertext:
    ///
    /// - Rotate the rotors
    /// - Pass the letter through the plugboard
    /// - Pass the letter through the rotors from right to left
    /// - Pass the letter through the reflector
    /// - Pass the letter back through the rotors from left to right
    /// - Pass the letter through the plugboard again
    ///
    /// The reflector maps each characters to different ones, meaning no character can be encrypted or decrypted
    /// into itself.
    ///
    /// This is exactly the same as calling `machine.encode(text)`, since the enigma cipher is
    /// symmetric; The only difference is semantic meaning and intent, i.e.,
    ///
    /// ```rust
    ///	assert_eq!(text, machine.decrypt(machine.decrypt(text)));
    ///	assert_eq!(text, machine.encrypt(machine.encrypt(text)));
    ///	assert_eq!(text, machine.decrypt(machine.encrypt(text)));
    ///	assert_eq!(text, machine.encrypt(machine.decrypt(text)));
    /// ```
    ///
    /// # Parameters
    /// - `text` - The text to decode.
    ///
    /// # Returns
    /// The decoded text.
    pub fn decrypt(&self, text: &str) -> String {
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

        let rotor_a = caeser_shift(&rotor_a.letters(), *offset_a_setting);
        let rotor_b = caeser_shift(&rotor_b.letters(), *offset_b_setting);
        let rotor_c = caeser_shift(&rotor_c.letters(), *offset_c_setting);

        let rotor_a_first_half = rotor_a.get((26 - *offset_a_setting as usize)..rotor_a.len()).unwrap().to_owned();
        let rotor_a_second_half = rotor_a.get(0..(26 - *offset_a_setting as usize)).unwrap().to_owned();
        let rotor_a = rotor_a_first_half + &rotor_a_second_half;
        let rotor_a = Alphabet::new(&rotor_a).unwrap();

        let rotor_b_first_half = rotor_b.get((26 - *offset_b_setting as usize)..rotor_b.len()).unwrap().to_owned();
        let rotor_b_second_half = rotor_b.get(0..(26 - *offset_b_setting as usize)).unwrap().to_owned();
        let rotor_b = rotor_b_first_half + &rotor_b_second_half;
        let rotor_b = Alphabet::new(&rotor_b).unwrap();

        let rotor_c_first_half = rotor_c.get((26 - *offset_c_setting as usize)..rotor_c.len()).unwrap().to_owned();
        let rotor_c_second_half = rotor_c.get(0..(26 - *offset_c_setting as usize)).unwrap().to_owned();
        let rotor_c = rotor_c_first_half + &rotor_c_second_half;
        let rotor_c = Alphabet::new(&rotor_c).unwrap();

        text.chars()
            .map(|mut letter| {
                if self.options.debug {
                    println!("Decrypting character: '{}'", letter.to_string().bold().cyan());
                }

                // Non-alphabetic characters stay the same
                if !letter.is_alphabetic() {
                    if self.options.clear_punctuation {
                        return String::new();
                    } else {
                        if self.options.debug {
                            println!("\tCharacter is punctuation; Leaving it as-is.");
                        }
                        return letter.to_string();
                    }
                }

                // Rotate rotor 3
                let mut rotor_trigger = self
                    .rotors
                    .2
                    .notches()
                    .iter()
                    .map(|notch| ALPHABET.index_of(*notch).unwrap())
                    .collect::<Vec<_>>()
                    .contains(&rotor_c_letter);
                rotor_c_letter += 1;

                // Rotate rotor 2
                if rotor_trigger {
                    rotor_trigger = self
                        .rotors
                        .1
                        .notches()
                        .iter()
                        .map(|notch| ALPHABET.index_of(*notch).unwrap())
                        .collect::<Vec<_>>()
                        .contains(&rotor_b_letter);
                    rotor_b_letter += 1;

                    // Rotate rotor 1
                    if rotor_trigger {
                        rotor_a_letter += 1;
                    }
                }
                // Double step sequence
                else if self
                    .rotors
                    .1
                    .notches()
                    .iter()
                    .map(|notch| ALPHABET.index_of(*notch).unwrap())
                    .collect::<Vec<_>>()
                    .contains(&rotor_b_letter)
                {
                    rotor_b_letter += 1;
                    rotor_a_letter += 1;
                }

                // Plugboard decryption
                let old_letter = letter;
                if let Some(plugboarded_letter) = self.plugboard.get(&letter) {
                    letter = *plugboarded_letter;
                }
                if self.options.debug {
                    println!(
                        "\tPassing character through {}: '{}' -> '{}'",
                        "plugboard".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    )
                }

                let offset_a = rotor_a_letter;
                let offset_b = rotor_b_letter;
                let offset_c = rotor_c_letter;

                // Rotor 3 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = rotor_c.letter_at(pos + offset_c);
                let pos = ALPHABET.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_c);
                if self.options.debug {
                    println!(
                        "\tPassing character through {}: '{}' -> '{}'",
                        "third rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Rotor 2 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = rotor_b.letter_at(pos + offset_b);
                let pos = ALPHABET.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_b);
                if self.options.debug {
                    println!(
                        "\tPassing character through {}: '{}' -> '{}'",
                        "second rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Rotor 1 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = rotor_a.letter_at(pos + offset_a);
                let pos = ALPHABET.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_a);
                if self.options.debug {
                    println!(
                        "\tPassing character through {}: '{}' -> '{}'",
                        "first rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Reflector Encryption
                let old_letter = letter;
                letter = *self.reflector.alphabet().get(&letter).unwrap();
                if self.options.debug {
                    println!(
                        "\tPassing character through {}: '{}' -> '{}'",
                        "reflector".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Rotor 1 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = ALPHABET.letter_at(pos + offset_a);
                let pos = rotor_a.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_a);
                if self.options.debug {
                    println!(
                        "\tPassing character back through {}: '{}' -> '{}'",
                        "first rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Rotor 2 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = ALPHABET.letter_at(pos + offset_b);
                let pos = rotor_b.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_b);
                if self.options.debug {
                    println!(
                        "\tPassing character back through {}: '{}' -> '{}'",
                        "second rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Rotor 3 Encryption
                let pos = ALPHABET.index_of(letter).unwrap();
                let let_ = ALPHABET.letter_at(pos + offset_c);
                let pos = rotor_c.index_of(let_).unwrap();
                let old_letter = letter;
                letter = ALPHABET.letter_at(pos - offset_c);
                if self.options.debug {
                    println!(
                        "\tPassing character back through {}: '{}' -> '{}'",
                        "third rotor".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                }

                // Plugboard Second Pass
                let old_letter = letter;
                if let Some(plugboarded_letter) = self.plugboard.get(&letter) {
                    letter = *plugboarded_letter;
                }
                if self.options.debug {
                    println!(
                        "\tPassing character back through {}: '{}' -> '{}'",
                        "plugboard".green().bold(),
                        old_letter.to_string().bold().cyan(),
                        letter.to_string().bold().cyan(),
                    );
                    println!("\tFinalized character: '{}'", letter.to_string().bold().cyan());
                }

                letter.to_string()
            })
            .collect()
    }

    /// Encodes the given text using this Enigma machine.
    ///
    /// The encryption process does the following for each letter in the plaintext:
    ///
    /// - Rotate the rotors
    /// - Pass the letter through the plugboard
    /// - Pass the letter through the rotors from right to left
    /// - Pass the letter through the reflector
    /// - Pass the letter back through the rotors from left to right
    /// - Pass the letter through the plugboard again
    ///
    /// The reflector maps each characters to different ones, meaning no character can be encrypted or decrypted
    /// into itself.
    ///
    /// This is exactly the same as calling `machine.decode(text)`, since the enigma cipher is
    /// symmetric; The only difference is semantic meaning and intent, i.e.,
    ///
    /// ```rust
    ///	assert_eq!(text, machine.decode(machine.decode(text)));
    ///	assert_eq!(text, machine.encode(machine.encode(text)));
    ///	assert_eq!(text, machine.decode(machine.encode(text)));
    ///	assert_eq!(text, machine.encode(machine.decode(text)));
    /// ```
    ///
    /// # Parameters
    /// - `text` - The text to encode.
    ///
    /// # Returns
    /// The encoded text.
    pub fn encrypt(&self, text: &str) -> String {
        self.decrypt(text)
    }
}

/// A trait applied to `anyhow::Result<EnigmaMachine>` that allows building an enigma machine and passing along errors if they occur.
pub trait EnigmaBuilder {
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
    /// # Errors
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    ///
    /// If the given numbers are not all in `[1, 26]`, an error is returned.
    fn rotors(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine>;

    /// Sets the plugboard for the machine. The given plugboard should be a space-separated string of letter pairs. This is automatically
    /// bidirectional, meaning the pair `AY` will map `A` to `Y` and also `Y` to `A`.
    ///
    /// # Parameters
    /// - `plugboard` - A space-separated string of letter pairs, i.e., `AY BF QR UX GZ`.
    ///
    /// # Returns
    /// The machine builder with the given plugboard applied.
    ///
    /// # Errors
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    ///
    /// If the given plugboard contains duplicate letters, an error is returned.
    ///
    /// If the given plugboard is not formatted as a space-separated list of letter pairs, an error is returned.
    fn plugboard(self, plugboard: &str) -> anyhow::Result<EnigmaMachine>;

    // Sets the reflector of the machine.
    ///
    /// # Parameters
    /// - `reflector` - The reflector to give the machine.
    ///
    /// # Returns
    /// The machine builder with the given ring settings applied.
    ///
    /// # Errors
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    ///
    /// If the given reflector string does not represent an existing reflector.
    fn reflector(self, reflector: &str) -> anyhow::Result<EnigmaMachine>;

    // Sets the ring settings of the machine.
    ///
    /// # Parameters
    /// - `first` - The first ring setting, in `[1, 26]`.
    /// - `second` - The second ring setting, in `[1, 26]`.
    /// - `third` - The third ring setting, in `[1, 26]`.
    ///
    /// # Returns
    /// The machine builder with the given ring settings applied.
    ///
    /// # Errors
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    ///
    /// If the given numbers are not all in `[1, 26]`, an error is returned.
    fn ring_settings(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine>;

    /// Sets the "ring positions" or "rotor positions" of the machine.
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
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    ///
    /// If the given numbers are not all in `[1, 26]`, an error is returned.
    fn ring_positions(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine>;

    /// Disables case preservation for this machine. This means that the output will be entirely
    /// uppercase instead of preserving the original message's casing.
    ///
    /// # Returns
    /// The machine builder with preserve casing set to `true`.
    ///
    /// # Errors
    /// If the machine builder passed to this is already an error, an error is returned immediately.
    fn clear_casing(self) -> anyhow::Result<EnigmaMachine>;

    /// Enables debugging for this enigma machine. This means that during each step of encryption,
    /// the machine will print information to stdout about what's happening in the encryption and
    /// what each letter becomes as it goes through each stage of encryption.
    fn debug(self) -> anyhow::Result<EnigmaMachine>;
}

impl EnigmaBuilder for anyhow::Result<EnigmaMachine> {
    fn rotors(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine> {
        let rotors = (first, second, third)
            .try_into_rotors()
            .map_err(|error| anyhow::anyhow!("Error while setting ring positions when creating Enigma machine: {error}"))?;
        self.map(|mut machine| {
            machine.rotors = rotors;
            machine
        })
    }

    fn reflector(self, reflector: &str) -> anyhow::Result<EnigmaMachine> {
        let reflector = Reflector::try_from(reflector).map_err(|error| anyhow::anyhow!("Error while setting ring positions when creating Enigma machine: {error}"))?;
        self.map(|mut machine| {
            machine.reflector = reflector;
            machine
        })
    }

    fn ring_settings(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine> {
        if let Ok(mut machine) = self {
            machine.ring_settings = (first - 1, second - 1, third - 1)
                .try_into_alphabet_index()
                .map_err(|error| anyhow::anyhow!("Error while setting ring positions when creating Enigma machine: {error}"))?;
            Ok(machine)
        } else {
            self
        }
    }

    fn ring_positions(self, first: u8, second: u8, third: u8) -> anyhow::Result<EnigmaMachine> {
        if let Ok(machine) = self {
            Ok(EnigmaMachine {
                ring_positions: (first - 1, second - 1, third - 1)
                    .try_into_alphabet_index()
                    .map_err(|error| anyhow::anyhow!("Error while setting ring positions when creating Enigma machine: {error}"))?,
                ..machine
            })
        } else {
            self
        }
    }

    fn plugboard(self, plugboard: &str) -> anyhow::Result<EnigmaMachine> {
        if let Ok(mut machine) = self {
            let mut chars = plugboard.chars().collect::<Vec<char>>();
            chars.dedup();
            if chars.len() != plugboard.len() {
                anyhow::bail!("Plugboard contains duplicate characters: {plugboard}");
            }

            let mappings = plugboard.split_whitespace();
            let mut plugboard = std::collections::HashMap::new();
            for pair in mappings {
                let mut chars = pair.chars();
                let first = chars.next().unwrap();
                let second = chars.next().unwrap();
                plugboard.insert(first, second);
                plugboard.insert(second, first);
            }

            machine.plugboard = plugboard;
            Ok(machine)
        } else {
            self
        }
    }

    fn clear_casing(self) -> anyhow::Result<EnigmaMachine> {
        if let Ok(mut machine) = self {
            machine.options.clear_casing = true;
            Ok(machine)
        } else {
            self
        }
    }

    fn debug(self) -> anyhow::Result<EnigmaMachine> {
        if let Ok(mut machine) = self {
            machine.options.debug = true;
            Ok(machine)
        } else {
            self
        }
    }
}

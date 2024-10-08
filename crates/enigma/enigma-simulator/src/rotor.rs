use crate::alphabet::Alphabet;

/// A rotor in an Enigma machine.
#[allow(clippy::upper_case_acronyms)]
pub enum Rotor {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

impl Rotor {
    fn unchecked_from(value: u8) -> Self {
        match value {
            1 => Self::I,
            2 => Self::II,
            3 => Self::III,
            4 => Self::IV,
            5 => Self::V,
            6 => Self::VI,
            7 => Self::VII,
            8 => Self::VIII,
            _ => panic!("Rotor number out of range: {value}"),
        }
    }
}

impl TryFrom<u8> for Rotor {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::I,
            2 => Self::II,
            3 => Self::III,
            4 => Self::IV,
            5 => Self::V,
            6 => Self::VI,
            7 => Self::VII,
            8 => Self::VIII,
            _ => anyhow::bail!("Rotor number out of range: {value}"),
        })
    }
}

pub trait IntoRotors {
    fn try_into_rotors(self) -> anyhow::Result<(Rotor, Rotor, Rotor)>;
    fn unchecked_into_rotors(self) -> (Rotor, Rotor, Rotor);
}

impl IntoRotors for (u8, u8, u8) {
    fn try_into_rotors(self) -> anyhow::Result<(Rotor, Rotor, Rotor)> {
        Ok((self.0.try_into()?, self.1.try_into()?, self.2.try_into()?))
    }

    fn unchecked_into_rotors(self) -> (Rotor, Rotor, Rotor) {
        (Rotor::unchecked_from(self.0), Rotor::unchecked_from(self.1), Rotor::unchecked_from(self.2))
    }
}

impl Rotor {
    pub fn alphabet(&self) -> Alphabet {
        Alphabet::new(match self {
            Self::I => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            Self::II => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            Self::III => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            Self::IV => "ESOVPZJAYQUIRHXLNFTGKDCMWB",
            Self::V => "VZBRGITYUPSDNHLXAWMJQOFECK",
            Self::VI => "JPGVOUMFYQBENHZRDKASXLICTW",
            Self::VII => "NZJHGRCXMYSWBOUFAIVLPEKQDT",
            Self::VIII => "FKQHTLXOCBJSPDZRAMEWNIUYGV",
        })
        .unwrap()
    }

    /// Returns the notches on this rotor as a `char` slice. In Enigma machines, each rotors have notches that
    /// determine whether the next rotor should rotate. The five basic rotors each have a single notch, and the
    /// remaining three each have two.
    ///
    /// # Returns
    /// The notches on this rotor as a `char` slice.
    pub const fn notches(&self) -> &'static [char] {
        match self {
            Self::I => &['Q'],
            Self::II => &['E'],
            Self::III => &['V'],
            Self::IV => &['J'],
            Self::V => &['Z'],
            Self::VI => &['M', 'Z'],
            Self::VII => &['M', 'Z'],
            Self::VIII => &['M', 'Z'],
        }
    }
}

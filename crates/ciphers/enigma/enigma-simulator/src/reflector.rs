use crate::alphabet::ALPHABET;
use strum::IntoEnumIterator;

#[derive(strum_macros::EnumIter, Debug, PartialEq, Eq, Hash)]
pub enum Reflector {
    A,
    B,
    C,
    BThin,
    CThin,
    Ukwr,
    Ukwk,
}

/// The memoized reflectors of the Enigma machine. This stores reflector maps so that they don't need to be constructed each time
/// a reflector's alphabet is used.
///
/// This is generated and used by `Reflector::alphabet()`.
static REFLECTORS: std::sync::OnceLock<std::collections::HashMap<Reflector, std::collections::HashMap<char, char>>> = std::sync::OnceLock::new();

impl Reflector {
    pub fn alphabet(&self) -> &'static std::collections::HashMap<char, char> {
        REFLECTORS
            .get_or_init(|| {
                let mut reflectors = std::collections::HashMap::new();
                for reflector in Self::iter() {
                    // Get the standard reflector alphabet used in Enigma machines
                    let alphabet = match reflector {
                        Self::A => "EJMZALYXVBWFCRQUONTSPIKHGD",
                        Self::B => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
                        Self::C => "FVPJIAOYEDRZXWGCTKUQSBNMHL",
                        Self::BThin => "ENKQAUYWJICOPBLMDXZVFTHRGS",
                        Self::CThin => "RDOBJNTKVEHMLFCWZAXGYIPSUQ",
                        Self::Ukwr => "QYHOGNECVPUZTFDJAXWMKISRBL",
                        Self::Ukwk => "IMETCGFRAYSQBZXWLHKDVUPOJN",
                    };

                    // Generate the map from the alphabet
                    let mut map = std::collections::HashMap::new();
                    for (letter, reflected_letter) in ALPHABET.letters().chars().zip(alphabet.chars()) {
                        map.insert(reflected_letter, letter);
                    }

                    // Memoize the alphabet map
                    reflectors.insert(reflector, map);
                }
                reflectors
            })
            .get(self)
            .unwrap()
    }

    pub fn unchecked_from(value: &str) -> Self {
        match value {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "BThin" => Self::BThin,
            "CThin" => Self::CThin,
            "UKWR" => Self::Ukwr,
            "UKWK" => Self::Ukwk,
            _ => panic!("Invalid reflector: {value}"),
        }
    }
}

impl TryFrom<&str> for Reflector {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "bthin" => Self::BThin,
            "cthin" => Self::CThin,
            "ukwr" => Self::Ukwr,
            "ukwk" => Self::Ukwk,
            _ => anyhow::bail!("Invalid reflector: {value}"),
        })
    }
}

use cipher_utils::alphabet::Alphabet;

pub struct Porta {
    key: String,
    alphabet: Alphabet,
}

impl Porta {
    pub fn new() -> impl PortaBuilder {
        Ok(IncompletePorta::default())
    }

    pub fn encrypt(&self, plaintext: &str) -> String {}

    pub fn decrypt(&self, plaintext: &str) -> String {}
}

#[derive(Default, Debug)]
struct IncompletePorta {
    key: Option<String>,
    alphabet: Option<Alphabet>,
}

pub trait PortaBuilder {
    fn key<T: AsRef<str>>(self, key: T) -> impl PortaBuilder;
    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl PortaBuilder;
    fn build(self) -> anyhow::Result<Porta>;
}

impl PortaBuilder for anyhow::Result<IncompletePorta> {
    fn key<T: AsRef<str>>(self, key: T) -> impl PortaBuilder {
        if let Ok(mut porta) = self {
            porta.key = Some(key.as_ref().to_owned());
            Ok(porta)
        } else {
            self
        }
    }

    fn alphabet<T: AsRef<str>>(self, alphabet: T) -> impl PortaBuilder {
        if let Ok(mut porta) = self {
            porta.alphabet = Some(Alphabet::caseless(alphabet.as_ref())?);
            Ok(porta)
        } else {
            self
        }
    }

    fn build(self) -> anyhow::Result<Porta> {
        if let Ok(porta) = self {
            let Some(key) = porta.key else {
                anyhow::bail!("Error building Porta cipher: No key provided.");
            };

            let Some(alphabet) = porta.alphabet else {
                anyhow::bail!("Error building Porta cipher: No alphabet provided.");
            };

            Ok(Porta { key, alphabet })
        } else {
            Err(self.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn encrypt_decrypt() {}
}

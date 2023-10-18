pub struct Reflector {
    wiring: String,
    model: String,
}

impl Reflector {
    pub fn new(wiring: &str, model: &str) -> Self {
        Reflector {
            wiring: wiring.to_string(),
            model: model.to_string(),
        }
    }

    pub fn encrypt(&self, char_in: char) -> Result<char, &'static str> {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        match ALPHABET.chars().position(|c| c == char_in) {
            Some(index) => match self.wiring.chars().nth(index) {
                Some(c) => Ok(c),
                None => Err("Character not found in wiring."),
            },
            None => Err("Character not found in alphabet."),
        }
    }
}

pub mod Reflectors {
    use super::Reflector;

    pub fn ukw_b() -> Reflector {
        Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", "UKW-B")
    }

    pub fn ukw_c() -> Reflector {
        Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", "UKW-C")
    }
}
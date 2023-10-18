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

    pub fn encrypt(&self, char_in: char) -> Option<char> {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        ALPHABET.chars().position(|c| c == char_in)
            .and_then(|index| self.wiring.chars().nth(index))
    }
}

pub mod reflectors {
    use super::Reflector;

    pub fn ukw_b() -> Reflector {
        Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", "UKW-B")
    }

    pub fn ukw_c() -> Reflector {
        Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", "UKW-C")
    }
}
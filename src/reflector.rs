/// Represents the Reflector component of an Enigma machine.
///
/// In the Enigma machine's encryption process, after a character passes through all rotors,
/// it reaches the reflector. The reflector ensures that the encryption process is
/// reversible by redirecting the character to another position before it returns
/// back through the rotors in the reverse order.
///
/// The wiring represents the character mapping of the reflector, determining the position
/// a character is redirected to. There were different types of reflectors (UKW-B, C) ... but
/// the only differed by their wirings and exploited the fact the 26 mod 2 = 0 to ensure each
/// grapheme had one and only one pair.
///
///
/// # Example
///
/// ```rust
/// use enigma_shark::reflectors;
///
/// let reflector = reflectors::ukw_b();
/// let encrypted_char = reflector.encrypt('A').unwrap();
/// // The character will always be reflected to a different position.
/// assert_ne!(encrypted_char, 'A');
/// ```
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
        ALPHABET
            .chars()
            .position(|c: char| c == char_in)
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

    pub fn from_name(name: &str) -> Reflector {
        match name {
            "ukw_b" => ukw_b(),
            "ukw_c" => ukw_c(),
            _ => panic!("Unknown reflector type: {}", name),
        }
    }
}

#[cfg(test)]
mod reflector_tests {
    use super::*;

    #[test]
    fn test_encrypt_ukw_b() {
        let reflector = reflectors::ukw_b();
        assert_eq!(reflector.encrypt('A'), Some('Y'));
        assert_eq!(reflector.encrypt('B'), Some('R'));
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_ne!(reflector.encrypt(c), Some(c));
        }
    }

    #[test]
    fn test_encrypt_ukw_c() {
        let reflector = reflectors::ukw_c();
        assert_eq!(reflector.encrypt('A'), Some('F'));
        assert_eq!(reflector.encrypt('B'), Some('V'));
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_ne!(reflector.encrypt(c), Some(c));
        }
    }

    #[test]
    fn test_reflector_from_name() {
        let reflector_b = reflectors::from_name("ukw_b");
        assert_eq!(reflector_b.model, "UKW-B");
        assert_eq!(reflector_b.encrypt('A'), Some('Y'));

        let reflector_c = reflectors::from_name("ukw_c");
        assert_eq!(reflector_c.model, "UKW-C");
        assert_eq!(reflector_c.encrypt('A'), Some('F'));
    }

    #[test]
    #[should_panic(expected = "Unknown reflector type")]
    fn test_unknown_reflector() {
        reflectors::from_name("unknown");
    }
}

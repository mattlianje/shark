/// Represents the Plugboard component of an Enigma machine.
///
/// The Plugboard is responsible for the final substitution of characters after they pass
/// through the rotors, through the reflector, and back through the rotors. It achieves this by
/// using a set of paired character mappings, essentially "plugging" one character into
/// another. If a character does not have a mapping in the plugboard, it remains unchanged.
///
pub struct Plugboard {
    plugboard_map: Vec<(char, char)>,
}

impl Plugboard {
    pub fn new(plugboard_map: Vec<(char, char)>) -> Result<Self, &'static str> {
        if !Self::is_valid_mapping(&plugboard_map) {
            return Err("Invalid plugboard mapping provided");
        }
        Ok(Plugboard { plugboard_map })
    }

    fn is_valid_mapping(plugboard_map: &[(char, char)]) -> bool {
        let mut chars_seen = std::collections::HashSet::new();
        for &(a, b) in plugboard_map {
            if chars_seen.contains(&a) || chars_seen.contains(&b) {
                return false;
            }
            if !('A'..='Z').contains(&a) || !('A'..='Z').contains(&b) {
                return false;
            }
            chars_seen.insert(a);
            chars_seen.insert(b);
        }
        true
    }

    pub fn pass_through(&self, c: char) -> char {
        self.plugboard_map
            .iter()
            .find(|&&(a, b)| a == c || b == c)
            .map(|&(a, b)| if a == c { b } else { a })
            .unwrap_or(c)
    }
}

pub mod plugboards {
    use super::Plugboard;
    use rand::prelude::ThreadRng;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};

    pub fn generate_random_mappings() -> Result<Plugboard, &'static str> {
        let mut chars: Vec<char> = ('A'..='Z').collect();
        let mut rng: ThreadRng = thread_rng();
        chars.shuffle(&mut rng);

        // Perhaps this should be len
        let num_mappings: usize = rng.gen_range(1..=13);
        let plugboard_map: Vec<(char, char)> = chars
            .chunks(2)
            .take(num_mappings)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        Plugboard::new(plugboard_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through_with_known_mapping() {
        let plugboard = Plugboard::new(vec![('A', 'K'), ('Z', 'C')]).unwrap();
        assert_eq!(plugboard.pass_through('A'), 'K');
        assert_eq!(plugboard.pass_through('K'), 'A');
        assert_eq!(plugboard.pass_through('Z'), 'C');
    }

    #[test]
    fn test_pass_through_with_unknown_character() {
        let plugboard = Plugboard::new(vec![('A', 'K'), ('Z', 'C')]).unwrap();
        assert_eq!(plugboard.pass_through('M'), 'M');
    }

    #[test]
    fn test_random_mapping_generation() {
        let plugboard = plugboards::generate_random_mappings().unwrap();
        // Now that we've unwrapped the Result, we have direct access to the Plugboard instance.
        assert!(plugboard.plugboard_map.len() <= 13);
        assert!(plugboard.plugboard_map.len() >= 1);
    }

    #[test]
    fn test_invalid_mappings() {
        // Duplicate mappings
        let plugboard = Plugboard::new(vec![('A', 'K'), ('A', 'L')]);
        assert!(plugboard.is_err());

        // Out of range characters
        let plugboard = Plugboard::new(vec![('A', '1')]);
        assert!(plugboard.is_err());

        // Characters not in A-Z range
        let plugboard = Plugboard::new(vec![('A', 'a')]);
        assert!(plugboard.is_err());
    }
}

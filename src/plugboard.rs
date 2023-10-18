use rand::seq::SliceRandom;
pub struct Plugboard {
    plugboard_map: Vec<(char, char)>,
}

impl Plugboard {
    pub fn new(plugboard_map: Vec<(char, char)>) -> Self {
        Plugboard { plugboard_map }
    }

    pub fn pass_through(&self, c: char) -> char {
        self.plugboard_map.iter()
            .find(|&&(a, b)| a == c || b == c)
            .map(|&(a, b)| if a == c { b } else { a })
            .unwrap_or(c)
    }
}

pub mod plugboards {
    use super::Plugboard;
    use rand::seq::SliceRandom;
    use rand::{Rng, thread_rng};
    use rand::prelude::ThreadRng;

    pub fn generate_random_mappings() -> Plugboard {
        let mut chars: Vec<char> = ('A'..='Z').collect();
        let mut rng: ThreadRng = thread_rng();
        chars.shuffle(&mut rng);

        // Perhaps this should be len
        let num_mappings: usize = rng.gen_range(1..=13);
        let plugboard_map: Vec<(char, char)> =
            chars.chunks(2).take(num_mappings)
            .map(|chunk| (chunk[0], chunk[1])).collect();

        Plugboard::new(plugboard_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through_with_known_mapping() {
        let plugboard = Plugboard::new(vec![('A', 'K'), ('Z', 'C')]);
        assert_eq!(plugboard.pass_through('A'), 'K');
        assert_eq!(plugboard.pass_through('K'), 'A');
        assert_eq!(plugboard.pass_through('Z'), 'C');
    }

    #[test]
    fn test_pass_through_with_unknown_character() {
        let plugboard = Plugboard::new(vec![('A', 'K'), ('Z', 'C')]);
        assert_eq!(plugboard.pass_through('M'), 'M');
    }

    #[test]
    fn test_random_mapping_generation() {
        let plugboard = plugboards::generate_random_mappings();
        assert!(plugboard.plugboard_map.len() <= 13);
        assert!(plugboard.plugboard_map.len() >= 1);
    }
}
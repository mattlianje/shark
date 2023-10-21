/// Represents a rotor in the Enigma machine.
///
/// Rotors are the primary cryptographic component of the Enigma machine. Each rotor has a set
/// wiring pattern (represented by `letter_roll`) that dictates how an input character is transformed.
/// As characters are encrypted, the rotor rotates, changing the current position (`position`) and thus the
/// transformation it applies.
///
/// The `notch` indicates at which position the rotor will cause the next rotor to its left to turn.
/// The `ring` is a static setting that affects the rotor's behavior but doesn't move during encryption.
/// Different rotor models (`model`) have different wiring patterns and notch positions.
///
/// # Example
///
/// ```rust
/// use enigma_shark::rotors;
///
/// let rotor = rotors::type_i('A', 'A');
/// let encrypted_char = rotor.pass_through_forward('A').unwrap();
/// ```
pub struct Rotor {
    letter_roll: String,
    position: char,
    notch: char,
    ring: char,
    model: String,
}

impl Rotor {
    const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub fn new(letter_roll: &str, position: char, notch: char, model: &str, ring: char) -> Self {
        Rotor {
            letter_roll: letter_roll.to_string(),
            position,
            notch,
            ring,
            model: model.to_string(),
        }
    }

    fn position_of(c: char) -> Option<usize> {
        Self::ALPHABET.chars().position(|x| x == c)
    }

    fn offset_position(&self, pos: usize) -> usize {
        (pos + Self::position_of(self.position).unwrap() + Self::position_of(self.ring).unwrap())
            % Self::ALPHABET.len()
    }

    pub fn turn(&mut self) -> bool {
        let current_pos = self
            .letter_roll
            .chars()
            .position(|x| x == self.position)
            .unwrap();
        let next_pos = (current_pos + 1) % self.letter_roll.len();
        self.position = self.letter_roll.chars().nth(next_pos).unwrap();
        self.position == self.notch
    }

    pub fn pass_through_forward(&self, c: char) -> Option<char> {
        let input_pos = Self::position_of(c).unwrap();
        let offset_input_pos = self.offset_position(input_pos);

        self.letter_roll.chars().nth(offset_input_pos)
    }

    pub fn pass_through_reverse(&self, c: char) -> Option<char> {
        let letter_pos = self.letter_roll.chars().position(|x| x == c).unwrap();
        let offset_letter_pos = (letter_pos + Self::ALPHABET.len()
            - Self::position_of(self.position).unwrap()
            - Self::position_of(self.ring).unwrap())
            % Self::ALPHABET.len();

        Self::ALPHABET.chars().nth(offset_letter_pos)
    }
}

pub mod rotors {
    use super::Rotor;

    /**
    For example, type_i maps E->A, K->B etc ...
    **/

    pub fn type_i(p: char, r: char) -> Rotor {
        Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", p, 'R', "type I", r)
    }

    pub fn type_ii(p: char, r: char) -> Rotor {
        Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", p, 'F', "type II", r)
    }

    pub fn type_iii(p: char, r: char) -> Rotor {
        Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", p, 'W', "type III", r)
    }

    pub fn type_iv(p: char, r: char) -> Rotor {
        Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", p, 'K', "type IV", r)
    }

    pub fn type_v(p: char, r: char) -> Rotor {
        Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", p, 'A', "type V", r)
    }
}

#[cfg(test)]
mod rotor_tests {
    use super::*;

    #[test]
    fn test_position_of() {
        assert_eq!(Rotor::position_of('A'), Some(0));
        assert_eq!(Rotor::position_of('Z'), Some(25));
        assert_eq!(Rotor::position_of('M'), Some(12));
        assert_eq!(Rotor::position_of('!'), None);
    }

    #[test]
    fn test_offset_position() {
        let rotor = rotors::type_i('A', 'A');
        assert_eq!(rotor.offset_position(0), 0);
        assert_eq!(rotor.offset_position(25), 25);
    }

    #[test]
    fn test_turn() {
        let mut rotor = rotors::type_i('A', 'A');
        assert_eq!(rotor.turn(), false);
        assert_eq!(rotor.position, 'I');
    }

    #[test]
    fn test_turn_with_notch() {
        let mut rotor = rotors::type_i('B', 'A');
        assert_eq!(rotor.turn(), true);
        assert_eq!(rotor.position, 'R');
    }

    #[test]
    fn test_pass_through_forward() {
        let rotor = rotors::type_i('A', 'A');
        assert_eq!(rotor.pass_through_forward('A'), Some('E'));
        assert_eq!(rotor.pass_through_forward('B'), Some('K'));
    }

    #[test]
    fn test_pass_through_reverse() {
        let rotor = rotors::type_i('A', 'A');
        assert_eq!(rotor.pass_through_reverse('E'), Some('A'));
        assert_eq!(rotor.pass_through_reverse('K'), Some('B'));
    }

    #[test]
    fn test_rotor_types() {
        let rotor = rotors::type_ii('A', 'A');
        assert_eq!(rotor.pass_through_forward('A'), Some('A'));
        assert_eq!(rotor.pass_through_forward('B'), Some('J'));
    }
}

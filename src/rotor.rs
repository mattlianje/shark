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

    fn next_letter(&self, c: char) -> char {
        match c {
            'Z' => 'A',
            _ => (c as u8 + 1) as char,
        }
    }

    fn offset_position(&self, pos: usize) -> usize {
        (pos + Self::position_of(self.position).unwrap() + Self::position_of(self.ring).unwrap()) % Self::ALPHABET.len()
    }

    pub fn turn(&mut self) -> bool {
        let current_pos = self.letter_roll.chars().position(|x| x == self.position).unwrap();
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
        let offset_letter_pos = (letter_pos + Self::ALPHABET.len() - Self::position_of(self.position).unwrap() - Self::position_of(self.ring).unwrap()) % Self::ALPHABET.len();

        Self::ALPHABET.chars().nth(offset_letter_pos)
    }
}

pub mod rotors {
    use super::Rotor;

    pub fn type_i(p: char) -> Rotor {
        Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", p, 'R', "type I", 'A')
    }

    pub fn type_ii(p: char) -> Rotor {
        Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", p, 'F', "type II", 'A')
    }

    pub fn type_iii(p: char) -> Rotor {
        Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", p, 'W', "type III", 'A')
    }

    pub fn type_iv(p: char) -> Rotor {
        Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", p, 'K', "type IV", 'A')
    }

    pub fn type_v(p: char) -> Rotor {
        Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", p, 'A', "type V", 'A')
    }
}
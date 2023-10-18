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

    pub fn turn(&self) -> Rotor {
        Rotor::new(&self.letter_roll, self.next_letter(self.position), self.notch, &self.model, self.ring)
    }

    fn next_letter(&self, c: char) -> char {
        match c {
            'Z' => 'A',
            _ => ((c as u8 + 1) as char),
        }
    }

    pub fn pass_through_forward(&self, c: char) -> Option<char> {
        let shifted_pos = Self::position_of(c).and_then(|pos_c|
            Self::position_of(self.position).map(|pos_p| pos_c + pos_p)
        )?;
        self.letter_roll.chars().nth(shifted_pos % Self::ALPHABET.len())
    }

    pub fn pass_through_reverse(&self, c: char) -> Option<char> {
        let letter_pos = Self::position_of(c).and_then(|pos_c|
            Self::position_of(self.position).map(|pos_p| pos_c - pos_p + Self::ALPHABET.len())
        )?;

        let original_letter = self.letter_roll.chars().nth(letter_pos % Self::ALPHABET.len())?;
        self.letter_roll.chars().position(|x| x == original_letter).and_then(|pos|
            Self::ALPHABET.chars().nth(pos)
        )
    }
}

pub mod Rotors {
    use super::Rotor;

    pub fn rotor_i(p: char) -> Rotor {
        Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", p, 'R', "type I", 'A')
    }

    pub fn rotor_ii(p: char) -> Rotor {
        Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", p, 'F', "type II", 'A')
    }

    pub fn rotor_iii(p: char) -> Rotor {
        Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", p, 'W', "type III", 'A')
    }

    pub fn type_iv(p: char) -> Rotor {
        Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", p, 'K', "type IV", 'A')
    }

    pub fn type_v(p: char) -> Rotor {
        Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", p, 'A', "type V", 'A')
    }
}
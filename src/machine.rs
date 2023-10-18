use crate::plugboard::Plugboard;
use crate::reflector::{Reflector, reflectors};
use crate::rotor::{Rotor, rotors};

pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    pub fn new(rotors: Vec<Rotor>, reflector: Reflector, plugboard: Plugboard) -> Self {
        EnigmaMachine {
            rotors,
            reflector,
            plugboard,
        }
    }

    fn encrypt(&mut self, input: char) -> Option<char> {
        (input.is_ascii_alphabetic() && input.is_uppercase())
            .then(|| input)
            .and_then(|char_to_encrypt| {
                self.rotors.iter_mut().fold(Some(char_to_encrypt), |current_char, rotor| {
                    current_char.and_then(|ch| rotor.pass_through_forward(ch))
                })
            })
            .and_then(|char_after_rotors| self.reflector.encrypt(char_after_rotors))
            .and_then(|mid_char| {
                self.rotors.iter_mut().rev().fold(Some(mid_char), |current_char, rotor| {
                    current_char.and_then(|ch| rotor.pass_through_reverse(ch))
                })
            })
            .map(|after_rotors_reverse| self.plugboard.pass_through(after_rotors_reverse))
    }

    fn advance_rotors(&mut self) {
        // Starting from the rightmost rotor
        let mut should_advance_next = true;
        for rotor in self.rotors.iter_mut().rev() {
            if should_advance_next {
                should_advance_next = rotor.turn();
            } else {
                break;
            }
        }
    }

    pub fn encrypt_message(&mut self, message: &str) -> Result<String, &'static str> {
        message.chars()
            .map(|ch| {
                self.advance_rotors();
                self.encrypt(ch).ok_or("Failed to encrypt character")
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_enigma_machine() -> EnigmaMachine {
        let rotor1 = rotors::type_i('A');
        let rotor2 = rotors::type_ii('B');
        let rotor3 = rotors::type_iii('C');

        let reflector = reflectors::ukw_b();
        let plugboard = Plugboard::new(vec![]);

        EnigmaMachine::new(vec![rotor1, rotor2, rotor3], reflector, plugboard)
    }

    #[test]
    fn test_enigma_reversibility() {
        let message = "HELLO";

        let mut enigma = setup_enigma_machine();
        let encrypted_msg = enigma.encrypt_message(message).unwrap();

        // Resets machine to the original starting state used to encrypt
        let mut enigma_reset = setup_enigma_machine();
        let decrypted_msg = enigma_reset.encrypt_message(&encrypted_msg).unwrap();

        assert_eq!(message, decrypted_msg);
    }
}
use crate::plugboard::Plugboard;
use crate::reflector::{reflectors, Reflector};
use crate::rotor::{rotors, Rotor};

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
                self.rotors
                    .iter_mut()
                    .fold(Some(char_to_encrypt), |current_char, rotor| {
                        current_char.and_then(|ch| rotor.pass_through_forward(ch))
                    })
            })
            .and_then(|char_after_rotors| self.reflector.encrypt(char_after_rotors))
            .and_then(|mid_char| {
                self.rotors
                    .iter_mut()
                    .rev()
                    .fold(Some(mid_char), |current_char, rotor| {
                        current_char.and_then(|ch| rotor.pass_through_reverse(ch))
                    })
            })
            .map(|after_rotors_reverse| self.plugboard.pass_through(after_rotors_reverse))
    }

    /// Advances the rotors in the Enigma machine.
    ///
    /// The rightmost rotor advances with every key press. When a rotor completes
    /// a full revolution, the rotor to its left advances one position. This
    /// function implements this cascading behavior.
    ///
    /// # Behavior
    ///
    /// - Starts from the rightmost rotor.
    /// - Advances the current rotor.
    /// - If the rotor hits its notch (i.e., completes a revolution, then turn() returns true),
    ///   ... the next rotor to its left will be signaled to turn on the subsequent call.
    /// - Stops advancing rotors when one does not hit its notch.
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

    /// Encrypts a given message using the current Enigma machine configuration.
    ///
    /// This method processes each character of the message sequentially. Before encrypting
    /// each character, it advances the rotors to simulate the behavior of a real Enigma machine.
    ///
    /// # Arguments
    ///
    /// * `message` - A reference to a string slice containing the message to be encrypted.
    ///   It is expected that the message only contains uppercase alphabetic characters.
    ///
    /// # Returns
    ///
    /// * A `Result` where:
    ///     - The `Ok` variant contains the encrypted message as a `String`.
    ///     - The `Err` variant indicates an error occurred during encryption, typically because
    ///       a non-encryptable character was encountered in the input message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enigma_shark::{EnigmaMachine, Plugboard, reflectors, rotors};
    /// fn setup_enigma_machine() -> EnigmaMachine {
    ///         let rotor1 = rotors::type_i('A', 'A');
    ///         let rotor2 = rotors::type_ii('B', 'A');
    ///         let rotor3 = rotors::type_iii('C', 'A');
    ///
    ///         let reflector = reflectors::ukw_b();
    ///         let plugboard = match Plugboard::new(vec![]) {
    ///             Ok(plugboard) => plugboard,
    ///             Err(e) => {
    ///                 eprintln!("Error setting up the plugboard: {}", e);
    ///                 Plugboard::new(vec![]).unwrap()
    ///             }
    ///         };
    ///
    ///         EnigmaMachine::new(vec![rotor1, rotor2, rotor3], reflector, plugboard)
    ///     }
    /// let mut enigma = setup_enigma_machine();
    /// let result = enigma.encrypt_message("HELLO");
    /// assert!(result.is_ok());
    /// ```
    pub fn encrypt_message(&mut self, message: &str) -> Result<String, String> {
        message
            .chars()
            .map(|ch| {
                self.advance_rotors();
                self.encrypt(ch)
                    .ok_or_else(|| format!("Failed to encrypt character: '{}'", ch))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_enigma_machine() -> EnigmaMachine {
        let rotor1 = rotors::type_i('A', 'A');
        let rotor2 = rotors::type_ii('B', 'A');
        let rotor3 = rotors::type_iii('C', 'A');
        let rotor4 = rotors::type_iv('D', 'A');

        let reflector = reflectors::ukw_b();

        let plugboard = match Plugboard::new(vec![]) {
            Ok(plugboard) => plugboard,
            Err(e) => {
                eprintln!("Error setting up the plugboard: {}", e);
                Plugboard::new(vec![]).unwrap()
            }
        };

        EnigmaMachine::new(vec![rotor1, rotor2, rotor3, rotor4], reflector, plugboard)
    }

    #[test]
    fn test_enigma_reversibility() {
        let message = "BANBURISMUS";

        let mut enigma = setup_enigma_machine();
        let encrypted_msg = enigma
            .encrypt_message(message)
            .expect("Failed to encrypt message");

        // Resets machine to the original starting state used to encrypt
        let mut enigma_reset = setup_enigma_machine();
        let decrypted_msg = enigma_reset
            .encrypt_message(&encrypted_msg)
            .expect("Failed to decrypt message");

        assert_eq!(message, decrypted_msg);
    }

    #[test]
    fn test_enigma_encryption_failure() {
        let message = "BANBURISMUS9";

        let mut enigma = setup_enigma_machine();
        match enigma.encrypt_message(message) {
            Ok(_) => panic!("Expected an error due to invalid character in the message"),
            Err(e) => assert_eq!(e, "Failed to encrypt character: '9'"),
        }
    }
}

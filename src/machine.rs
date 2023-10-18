use crate::plugboard::Plugboard;
use crate::reflector::{Reflector, Reflectors};
use crate::rotor::{Rotor, Rotors};

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

    pub fn encrypt(&self, input: char) -> Result<char, &'static str> {
        let plugboard_pass = self.plugboard.pass_through(input)?;

        let rotors_after_forward_pass = self.rotors.iter()
            .try_fold(plugboard_pass, |state, rotor| rotor.pass_through_forward(state))
            .map_err(|_| "Error in forward rotor pass")?;

        let mid_char = self.reflector.encrypt(rotors_after_forward_pass)?;

        let final_char_result = self.rotors.iter().rev()
            .try_fold(mid_char, |state, rotor| rotor.pass_through_reverse(state));

        let final_char = match final_char_result {
            Some(Ok(c)) => c,
            Some(Err(e)) => return Err(e),
            None => return Err("Failed processing with reverse rotor pass."),
            _ => {}
        };

        self.plugboard.pass_through(final_char)
    }
}

fn main() {
    let plugboard = Plugboard::new(vec![('A', 'K'), ('Z', 'C')]);
    let rotor1 = Rotors::rotor_i('A');
    let rotor2 = Rotors::rotor_ii('A');
    let rotor3 = Rotors::rotor_iii('A');
    let reflector = Reflectors::ukw_b();

    let enigma = EnigmaMachine::new(vec![rotor1, rotor2, rotor3], reflector, plugboard);

    let encrypted = enigma.encrypt('A');
    match encrypted {
        Some(e) => println!("Encrypted Character: {}", e),
        None => println!("Error encrypting character."),
    }
}
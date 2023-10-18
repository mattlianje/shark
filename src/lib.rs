mod rotor;
mod reflector;
mod plugboard;
mod machine;

pub use machine::EnigmaMachine;
pub use rotor::{Rotor, Rotors};
pub use reflector::{Reflector, Reflectors};
pub use plugboard::Plugboard;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

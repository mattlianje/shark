use clap::Parser;
use std::fs;
use std::io::Read;
use serde::Deserialize;
use crate::{EnigmaMachine, Plugboard, reflectors, rotors};

#[derive(Deserialize, Debug)]
struct RotorConfig {
    type_: String,
    position: char,
    ring_setting: char,
}

#[derive(Deserialize, Debug)]
struct PlugboardMapping {
    from: char,
    to: char,
}

#[derive(Deserialize, Debug)]
struct MachineConfig {
    rotors: Vec<RotorConfig>,
    reflector: String,
    plugboard: Vec<PlugboardMapping>,
}

#[derive(Parser, Debug)]
struct Args {
    /// Input message or file for encryption
    #[arg(short, long)]
    input: String,

    /// Optional: Configuration file for machine settings
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut enigma_machine = match args.config {
        Some(config_file) => {
            let config = fs::read_to_string(config_file)
                .expect("Failed to read the machine configuration file");
            match setup_enigma_from_config(Some(config)) {
                Ok(machine) => machine,
                Err(err) => panic!("Failed to set up the enigma machine: {}", err),
            }
        },
        None => match setup_enigma_from_config(None) {
            Ok(machine) => machine,
            Err(err) => panic!("Failed to set up the enigma machine: {}", err),
        },
    };

    let mut reader: Box<dyn Read> = match (atty::is(atty::Stream::Stdin), &args.input) {
        (true, input) if input.is_empty() => {
            eprintln!("Error: Please provide input through stdin or use the '-i' option.");
            std::process::exit(1);
        },
        (false, _) => Box::new(std::io::stdin()),
        (_, input) => Box::new(fs::File::open(input).expect("Failed to open input file")),
    };

    let mut buffer = [0; 4096];
    while let Ok(len) = reader.read(&mut buffer) {
        if len == 0 { break; }
        let input_chunk = String::from_utf8_lossy(&buffer[..len]);
        print!("{}", encrypt_with_enigma(input_chunk.to_string(), &mut enigma_machine));
    }
}

fn setup_enigma_from_config(machine_config: Option<String>) -> Result<EnigmaMachine, String> {
    if let Some(config) = machine_config {
        let machine_settings: MachineConfig = serde_json::from_str(&config)
            .expect("Failed to parse machine configuration");

        let mut rotor_list = Vec::new();

        for rotor_config in machine_settings.rotors {
            match rotor_config.type_.as_str() {
                "type_i" => rotor_list.push(rotors::type_i(rotor_config.position, rotor_config.ring_setting)),
                "type_ii" => rotor_list.push(rotors::type_ii(rotor_config.position, rotor_config.ring_setting)),
                "type_iii" => rotor_list.push(rotors::type_iii(rotor_config.position, rotor_config.ring_setting)),
                "type_iv" => rotor_list.push(rotors::type_iv(rotor_config.position, rotor_config.ring_setting)),
                "type_v" => rotor_list.push(rotors::type_v(rotor_config.position, rotor_config.ring_setting)),
                _ => panic!("Unsupported rotor type!"),
            }
        }

        let reflector = reflectors::from_name(&machine_settings.reflector);

        let plugboard_mappings: Vec<(char, char)> = machine_settings.plugboard
            .iter()
            .map(|mapping| (mapping.from, mapping.to))
            .collect();

        let plugboard = match Plugboard::new(plugboard_mappings) {
            Ok(p) => p,
            Err(err) => return Err(err.to_string()),
        };

        Ok(EnigmaMachine::new(rotor_list, reflector, plugboard))
    } else {
        let rotor1 = rotors::type_i('A', 'A');
        let rotor2 = rotors::type_ii('B', 'A');
        let rotor3 = rotors::type_iii('C', 'A');
        let rotor4 = rotors::type_iv('D', 'A');

        let reflector = reflectors::ukw_b();
        let plugboard = match Plugboard::new(vec![]) {
            Ok(p) => p,
            Err(err) => return Err(err.to_string()),
        };

        Ok(EnigmaMachine::new(vec![rotor1, rotor2, rotor3, rotor4], reflector, plugboard))
    }
}

fn encrypt_with_enigma(input: String, enigma: &mut EnigmaMachine) -> String {
    match enigma.encrypt_message(&input) {
        Ok(encrypted_msg) => encrypted_msg,
        Err(err) => panic!("Encryption failed with error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_with_default_config() {
        let input = "HELLO".to_string();
        let mut machine = match setup_enigma_from_config(None) {
            Ok(machine) => machine,
            Err(err) => panic!("Failed to set up the enigma machine for test: {}", err),
        };
        let encrypted = encrypt_with_enigma(input.clone(), &mut machine);
        assert_ne!(encrypted, input);
    }

    #[test]
    fn test_encrypt_with_custom_config() {
        let config = r#"
        {
            "rotors": [
                {"type_": "type_i", "position": "A", "ring_setting": "A"},
                {"type_": "type_ii", "position": "B", "ring_setting": "A"},
                {"type_": "type_iii", "position": "C", "ring_setting": "A"}
            ],
            "reflector": "ukw_b",
            "plugboard": [{"from": "A", "to": "B"}]
        }
        "#.to_string();

        let input = "HELLO".to_string();
        let mut machine = match setup_enigma_from_config(Some(config)) {
            Ok(machine) => machine,
            Err(err) => panic!("Failed to set up the enigma machine for test: {}", err),
        };
        let encrypted = encrypt_with_enigma(input.clone(), &mut machine);
        assert_ne!(encrypted, input);
    }

    #[test]
    #[should_panic(expected = "Unsupported rotor type!")]
    fn test_invalid_rotor_type() {
        let config = r#"
        {
            "rotors": [{"type_": "invalid_type", "position": "A", "ring_setting": "A"}],
            "reflector": "ukw_b",
            "plugboard": []
        }
        "#.to_string();
        setup_enigma_from_config(Some(config)).unwrap();
    }
}

use clap::Parser;
use enigma_shark::{reflectors, rotors, EnigmaMachine, Plugboard};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

#[derive(Deserialize, Debug)]
struct RotorConfig {
    type_: String,
    position: char,
    //ring: char,
    ring: char,
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
    //plugboard: Vec<PlugboardMapping>,
    plugboard: HashMap<char, char>,
}

#[derive(Parser, Debug)]
struct Args {
    /// Optional: Input message or file for encryption
    #[arg(short, long)]
    input: Option<String>,

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
        }
        None => match setup_enigma_from_config(None) {
            Ok(machine) => machine,
            Err(err) => panic!("Failed to set up the enigma machine: {}", err),
        },
    };

    let mut reader: Box<dyn Read> = match args.input {
        Some(input_str) if atty::is(atty::Stream::Stdin) => {
            Box::new(Cursor::new(input_str.into_bytes()))
        }
        None if !atty::is(atty::Stream::Stdin) => Box::new(std::io::stdin()),
        _ => {
            eprintln!("Error: Please provide input through stdin or use the '--input' option.");
            std::process::exit(1);
        }
    };

    let mut buffer = [0; 4096];
    while let Ok(len) = reader.read(&mut buffer) {
        if len == 0 {
            break;
        }
        let input_chunk = String::from_utf8_lossy(&buffer[..len]);
        println!(
            "{}",
            encrypt_with_enigma(input_chunk.to_string(), &mut enigma_machine)
        );
    }
}

fn setup_enigma_from_config(machine_config: Option<String>) -> Result<EnigmaMachine, String> {
    if let Some(config) = machine_config {
        let machine_settings: MachineConfig =
            serde_json::from_str(&config).expect("Failed to parse machine configuration");

        let mut rotor_list = Vec::new();

        for rotor_config in machine_settings.rotors {
            match rotor_config.type_.as_str() {
                "type_i" | "i" => {
                    rotor_list.push(rotors::type_i(rotor_config.position, rotor_config.ring))
                }
                "type_ii" | "ii" => {
                    rotor_list.push(rotors::type_ii(rotor_config.position, rotor_config.ring))
                }
                "type_iii" | "iii" => {
                    rotor_list.push(rotors::type_iii(rotor_config.position, rotor_config.ring))
                }
                "type_iv" | "iv" => {
                    rotor_list.push(rotors::type_iv(rotor_config.position, rotor_config.ring))
                }
                "type_v" | "v" => {
                    rotor_list.push(rotors::type_v(rotor_config.position, rotor_config.ring))
                }
                _ => panic!("Unsupported rotor type!"),
            }
        }

        let reflector = reflectors::from_name(&machine_settings.reflector);

        let plugboard_mappings: Vec<(char, char)> = machine_settings
            .plugboard
            .iter()
            .map(|(&from, &to)| (from, to))
            .collect();

        let plugboard = Plugboard::new(plugboard_mappings)
            .expect("Failed to initialize the plugboard with provided mappings");

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

        Ok(EnigmaMachine::new(
            vec![rotor1, rotor2, rotor3, rotor4],
            reflector,
            plugboard,
        ))
    }
}

pub fn encrypt_with_enigma(input: String, enigma: &mut EnigmaMachine) -> String {
    let processed_input = input.trim().to_uppercase();
    match enigma.encrypt_message(&processed_input) {
        Ok(encrypted_msg) => encrypted_msg,
        Err(err) => {
            eprintln!("Encryption failed with error: {}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_encrypt_with_default_config() {
        let input = "BLETCHLEY".to_string();
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
                {"type_": "type_i", "position": "A", "ring": "A"},
                {"type_": "type_ii", "position": "B", "ring": "A"},
                {"type_": "type_iii", "position": "C", "ring": "A"}
            ],
            "reflector": "ukw_b",
            "plugboard": {"A": "B"}
        }
        "#
        .to_string();

        let input = "BLETCHLEY".to_string();
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
            "rotors": [{"type_": "invalid_type", "position": "A", "ring": "A"}],
            "reflector": "ukw_b",
            "plugboard": {}
        }
        "#
        .to_string();
        setup_enigma_from_config(Some(config)).unwrap();
    }

    #[test]
    fn benchmark_encrypt_5mb() {
        // Generate a 5MB string input
        let input: String = std::iter::repeat("A").take(5 * 1024).collect();

        let mut machine = match setup_enigma_from_config(None) {
            Ok(machine) => machine,
            Err(err) => panic!("Failed to set up the enigma machine for test: {}", err),
        };

        let mut cursor = Cursor::new(input.into_bytes());
        let mut buffer = [0; 4096];

        let start_time = Instant::now();

        while let Ok(len) = cursor.read(&mut buffer) {
            if len == 0 {
                break;
            }
            let input_chunk = String::from_utf8_lossy(&buffer[..len]);
            let _ = encrypt_with_enigma(input_chunk.to_string(), &mut machine);
        }

        let elapsed_time = start_time.elapsed();

        println!("Time taken to encrypt 5MB: {:?}", elapsed_time);
    }
}

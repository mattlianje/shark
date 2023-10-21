# shark

A CLI Enigma tool named after the "Shark" Blackout of 1942, when the Kriegsmarine added a fourth rotor to their Enigma machines 
stymieing Bletchley Park's efforts to decipher Atlantic U-boat communications for nearly 10 months.

## Installation
```bash
$ curl -sSL https://github.com/mattlianje/shark/blob/main/install.sh | bash

# Run the below to make sure the installation was successful
# `enigma` should be available system wide
$ enigma --help
```

## Use
```bash
# Chain enigma with other commands
$ echo "HELLO WORLD" | enigma | grep "SPECIFIC_PATTERN"

# Use your custom settings
$ cat config.json
{
    "rotors": [
        {"type_": "type_i", "position": "A", "ring_setting": "A"},
        {"type_": "type_ii", "position": "B", "ring_setting": "A"},
        {"type_": "type_iii", "position": "C", "ring_setting": "A"}
    ],
    "reflector": "ukw_b",
    "plugboard": [{"from": "A", "to": "B"}]
}

# Use these settings easily
$ enigma --input plaintext.txt --config config.json > encrypted.txt
```
## Features/Goals
- **UNIX Philosophy Adherence:** Shark focuses on doing one thing well: encryption.

![UNIX philosophy](img/enigma-pipes-diagram.png)
- **Blazing Speed:** Primarily built to learn about Rust, I will continue to work on lowering the memory footprint of `shark` and making use of Rust's concurrency. Shark processes data in chunks, making it suitable for large datasets and > 1000x faster on 500MB inputs than other cli enigmas.
- **Infinite Stream Capable:** Designed with streaming data in mind, Shark can handle infinite data streams, allowing for real-time encryption tasks.
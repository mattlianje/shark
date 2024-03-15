# shark
<p align="center">
  <img src="img/4-rotor-enigma.jpeg" width="300" alt="four rotor Enigma">
</p>

A UNIX friendly CLI [Enigma](https://en.wikipedia.org/wiki/Enigma_machine) emulator.

![shark on unix](img/bp-readme.gif)

`Shark` is named after the "Shark" Blackout of 1942, when the Kriegsmarine added a fourth rotor to their Enigma machines 
stymieing Bletchley Park's efforts to decipher Atlantic U-boat communications for nearly 10 months.

## Installation
Add this to your `Cargo.toml` for [crates.io/enigma_shark](https://crates.io/crates/enigma_shark):
```toml
[dependencies]
enigma_shark = "*"
```

System wide:
```bash
curl -sSL https://raw.githubusercontent.com/mattlianje/shark/main/install.sh | bash
```

Check the installation was successful
```
enigma --help
```

## Use
```bash
# Chain enigma with other commands
$ echo "HELLO" | enigma | grep "FOO"

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

$ enigma --input plaintext.txt --config config.json > encrypted.txt
```
## Features/Goals
- **UNIX Philosophy Adherence:** Shark focuses on doing one thing well: symmetric en/decryption.

![UNIX philosophy](img/enigma-pipes-diagram.png)
- **Blazing Speed:** Primarily built to learn about Rust, I will continue to work on lowering the memory footprint of `shark` and making use of Rust's concurrency. Shark processes data in chunks, making it suitable for large datasets and > 50x faster on 5MB inputs than performant [C++ cli enigmas](benches/bench.sh)[^1].
- **Infinite Stream Capable:** Designed with streaming data in mind, Shark can handle infinite data streams, allowing for real-time encryption tasks.

[^1]: Encryption times for 5MB of data ... Shark: 53ms, C++ cli: 2693ms

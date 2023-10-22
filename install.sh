#!/bin/bash

if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

echo "Fetching the enigma-shark crate..."
cargo install enigma_shark

if [[ "$SHELL" == *"zsh"* ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
else
    SHELL_CONFIG="$HOME/.bashrc"
fi

if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "Adding Cargo's bin directory to PATH in $SHELL_CONFIG..."
    echo "export PATH=\$PATH:\$HOME/.cargo/bin" >> $SHELL_CONFIG
else
    echo "Cargo's bin directory is already in PATH."
fi

echo "Installation complete! You should now be able to use the enigma_cli command system-wide."
echo "Example usage:"
echo "    enigma --input HELLO "
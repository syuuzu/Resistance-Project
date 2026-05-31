# Resistance Project
Cryptology Class Project

Stream cipher program designed for communication in a highly constrained, zero-trust environment.

The threat model assumes major industrial hardware and open-source software distributions are subverted and unsafe.
## Core Features

* **Stream Cipher:** Uses the Salsa20 cipher so one key can encrypt and decrypt multiple messages.
* **QR Code Transmission:** An offline machine can display a QR code with encrypted data to transfer to another device without using any wires or radios.
* **Vendored Dependencies:** Designed to be compiled entirely offline using vendored dependencies. Avoids the risk of using potentially compromised package managers during the build phase.

## Setup

This software is intended to be run on a air-gapped machine and secure machine. For the sake of the threat model **do not** compile this program on an internet-connected machine.

1. On a internet connected device, clone this repository.
2. Burn the entire project directory to a read-only CD or DVD.
3. Insert the disk into your air-gapped machine.
4. Compile the binary locally on the trusted hardware:
```bash
cargo build --release
```

## Usage
```bash
./resistance --help
```
```bash
Resistance Project

Usage:
  ./resistance -generate|-g <output_key_file>
  ./resistance -encrypt|-e <input_file> <key_file>
  ./resistance -decrypt|-d <input_file> <key_file>
  ./resistance --help

Options:
  -g <size_in_bytes> <output_key_file> generates a key pad of size 32 to <output_key_file>.
  -e <input_file> <key_file> displays a QR code of <input_file> encrypted with <key_file>. Max size of file size 2953 bytes.
  -d <input_file> <key_file> prints decrypted <input_file> using <key_file>.
  --help displays this message.
```

## WARNINGS

1. **DATA LIMITS:**  Because the encryption output uses a QR code, a single ciphertext can't exceed **2,953 bytes**.

## Resources used
[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

[The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-vendor.html)

[QR Code](https://docs.rs/qrcode/latest/qrcode/)

[Wikipedia Salsa20](https://en.wikipedia.org/wiki/Salsa20)
## License

This project is licensed under the [Apache License 2.0](LICENSE).

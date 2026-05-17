# Resistance Project

This is a stream cipher program designed for communication in a highly constrained, zero-trust environment.

This project assumes major industrial hardware and open-source software distributions are subverted.
## Core Features

* **QR Code Transmission:** A clean offline machine displays a QR code with encrypted data which is scanned by a secondary "dirty" transmission device. 
* **Vendored Dependencies:** Designed to be compiled entirely offline using vendored dependencies. Avoids the risk of using potentially compromised package managers during the build phase.
* **Hardware Entropy:** Key generation relies directly on the Linux kernel's CSPRNG avoiding potentially backdoored CPU-level instructions (like Intel RDRAND).

## Threat Model & Setup

This software is intended to be run on a strictly air-gapped machine (e.g. a Librebooted ThinkPad with all physical radios disabled).

**Do not** compile this binary on an internet-connected machine.

1. On a "dirty" internet connected machine, clone this repository.
2. Burn the entire project directory to a read-only CD-R or DVD-R.
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

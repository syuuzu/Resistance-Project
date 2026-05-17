# Resistance OTP

A One-Time Pad (OTP) designed for communication in a highly constrained, zero-trust environment.

This tool assumes major industrial hardware and open-source software distributions are subverted. This program is designed to facilitate communication across an air-gap, so no sensitive data ever touches an internet connected machine.

## Core Features

* **QR Code Transmission:** A sterile machine displays a QR code with encrypted data which is scanned by a secondary "dirty" transmission device. 
* **Vendored Dependencies:** Designed to be compiled entirely offline using vendored dependencies. This bypasses the need for the internet, avoiding potentially compromised package managers during the build phase.
* **Hardware Entropy:** Key generation relies directly on the Linux kernel's CSPRNG avoiding potentially backdoored CPU-level instructions (like Intel RDRAND).

## Threat Model & Setup

This software is intended to be run on a strictly air-gapped machine (e.g. a Librebooted ThinkPad with all physical radios disabled).

**Do not** compile this binary on an internet-connected machine.

1. On a "dirty" internet connected machine, clone this repository.
2. Burn the entire project directory to a read-only CD-R or DVD-R.
3. Insert the optical media into your air-gapped machine.
4. Compile the binary locally on the trusted hardware:
```bash
cargo build --release
```

## Usage
```bash
./resistance_otp --help

Resistance OTP

Usage:
  target/debug/resistance_otp -generate|-g <size_in_bytes> <output_key_file>
  target/debug/resistance_otp -encrypt|-e <input_file> <key_file>
  target/debug/resistance_otp -decrypt|-d <input_file> <key_file>
  target/debug/resistance_otp --help

Options:
  -g <size_in_bytes> <output_key_file> generates a key pad of size <size_in_bytes> to <output_key_file>. Never reuse a key pad.
  -e <input_file> <key_file> displays a QR code of <input_file> encrypted with <key_file>.
  -d <input_file> <key_file> prints decrypted <input_file> using <key_file>.
  --help displays this message.
```

## WARNINGS

1. **NEVER REUSE A PAD:**  This is a ONE-TME pad. If a key file is used to encrypt more than one message your encrypted messages are at risk to be recovered.
2. **DATA LIMITS:**  Because the encryption output uses a QR code, a single ciphertext can't exceed **2,953 bytes**.

## License

This project is licensed under the [Apache License 2.0](LICENSE).

use qrcode::QrCode;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

//uses linux's built in CSPRNG to generate the key
//(possible flaw if linux is subverted but better than using a non cryptographically safe PRNG)
fn generate_key(size: usize, path: &str) {
    //open linux's kernel entropy pool
    let mut urandom = File::open("/dev/urandom").unwrap_or_else(|err| {
        eprintln!("Failed to open /dev/urandom: {}", err);
        process::exit(1);
    });

    //create a buffer to hold random bytes of defined size
    let mut key_data = vec![0u8; size];
    urandom.read_exact(&mut key_data).unwrap_or_else(|err| {
        eprintln!("Failed to read from /dev/urandom: {}", err);
        process::exit(1);
    });

    //write key to file
    let mut key_file = File::create(path).unwrap_or_else(|err| {
        eprintln!("Failed to create key file at '{}': {}", path, err);
        process::exit(1);
    });

    key_file.write_all(&key_data).unwrap_or_else(|err| {
        eprintln!("Failed to write key file: {}", err);
        process::exit(1);
    });

    println!("Generated key to '{}'", path)
}

//preforms the xor operation and returns the result as a vector
fn xor_key(input_path: &str, key_path: &str) -> Vec<u8> {
    //open and read input file
    let mut input_file = File::open(input_path).unwrap_or_else(|err| {
        eprintln!("Failed to open input file '{}': {}", input_path, err);
        process::exit(1);
    });
    let mut input_data = Vec::new();
    input_file
        .read_to_end(&mut input_data)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read input file: {}", err);
            process::exit(1);
        });

    //open and read key file
    let mut key_file = File::open(key_path).unwrap_or_else(|err| {
        eprintln!("Failed to open key file '{}': {}", key_path, err);
        process::exit(1);
    });
    let mut key_data = Vec::new();
    key_file.read_to_end(&mut key_data).unwrap_or_else(|err| {
        eprintln!("Failed to read key file: {}", err);
        process::exit(1);
    });

    //make sure key is longer than what is being encrypted
    if key_data.len() < input_data.len() {
        eprintln!(
            "Key length ({}) is shorter than input length ({})!",
            key_data.len(),
            input_data.len()
        );
        process::exit(1);
    }

    //bitwise XOR to encrypt/decrypt
    let mut output_data = Vec::with_capacity(input_data.len());
    //loop for each bit
    for i in 0..input_data.len() {
        output_data.push(input_data[i] ^ key_data[i]);
    }

    return output_data;
}

fn encrypt(input_path: &str, key_path: &str) {
    let encrypted = xor_key(input_path, key_path);

    //make sure message is small enough to fit on a qr code (from what I could find 2953 is the max bytes)
    if encrypted.len() > 2953 {
        eprintln!(
            "Encrypted file is too big to fit onto a qr code. Max size is 2953 bytes your file is {} bytes.",
            encrypted.len()
        );
        process::exit(1);
    }

    //print qr code
    let code = QrCode::new(&encrypted).unwrap_or_else(|_| {
        eprintln!("Failed to encode to QR code");
        process::exit(1);
    });

    //using ansi color codes to make a qr code in the terminal
    let image = code
        .render::<&str>()
        .quiet_zone(true)
        .dark_color("\x1b[40m  \x1b[0m")
        .light_color("\x1b[47m  \x1b[0m")
        .build();

    println!("{}", image);
}

fn decrypt(input_path: &str, key_path: &str) {
    let output_data = xor_key(input_path, key_path);
    let message =
        String::from_utf8(output_data).expect("Decryption failed to decode the file to utf8.");
    println!("---------DECODED MESSAGE---------");
    println!("{}", message);
}

fn display_usage(name: &str) {
    println!("Resistance OTP\n");
    println!("Usage:");
    println!("  {} -generate|-g <size_in_bytes> <output_key_file>", name);
    println!("  {} -encrypt|-e <input_file> <key_file>", name);
    println!("  {} -decrypt|-d <input_file> <key_file>", name);
    println!("  {} --help", name);
    println!("\nOptions:");
    println!(
        "  -g <size_in_bytes> <output_key_file> generates a key pad of size <size_in_bytes> to <output_key_file>. Never reuse a key pad."
    );
    println!(
        "  -e <input_file> <key_file> displays a QR code of <input_file> encrypted with <key_file>."
    );
    println!("  -d <input_file> <key_file> prints decrypted <input_file> using <key_file>.");
    println!("  --help displays this message.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        //exit if less than 2 args
        display_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];

    //match statement to eval commands
    match command.as_str() {
        //help flags to explain commands
        "--help" | "-h" | "help" => {
            display_usage(&args[0]);
            process::exit(0);
        }
        //generate key
        "generate" | "-generate" | "-g" => {
            if args.len() != 4 {
                eprintln!(
                    "Usage: {} generate <size_in_bytes> <output_key_file>",
                    args[0]
                );
                process::exit(1);
            }
            let size: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid size parameter.");
                process::exit(1);
            });

            let path = &args[3];
            generate_key(size, path);
        }
        //encrypt command takes in a message and a key file to generate a qr code
        "encrypt" | "-encrypt" | "-e" => {
            if args.len() != 4 {
                eprintln!("Usage: {} crypt <input_file> <key_file>", args[0]);
                process::exit(1);
            }
            let input_path = &args[2];
            let key_path = &args[3];
            encrypt(input_path, key_path);
        }
        "decrypt" | "-decrypt" | "-d" => {
            if args.len() != 4 {
                eprintln!("Usage: {} crypt <input_file> <key_file>", args[0]);
                process::exit(1);
            }
            let input_path = &args[2];
            let key_path = &args[3];
            decrypt(input_path, key_path);
        }
        //else
        _ => {
            eprintln!("Unknown command: {}", command);
            display_usage(&args[0]);
            process::exit(1);
        }
    }
}

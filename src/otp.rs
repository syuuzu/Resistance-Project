use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

//plan to use linux's built in CSPRNG to generate the key
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

fn crypt(input_path: &str, key_path: &str, output_path: &str) {
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

    //write to file (for now)
    let mut output_file = File::create(output_path).unwrap_or_else(|err| {
        eprintln!("Failed to create output file at '{}': {}", output_path, err);
        process::exit(1);
    });
    output_file.write_all(&output_data).unwrap_or_else(|err| {
        eprintln!("Failed to write output file: {}", err);
        process::exit(1);
    });

    println!("Message saved to: {}", output_path);
}

fn display_usage(name: &str) {
    eprintln!("Rust OTP Util)");
    eprintln!("Usage:");
    eprintln!(" {} generate <size_in_bytes> <output_key_file>", name);
    eprintln!(" {} crypt <input_file> <key_file> <output_file>", name);
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
        //generate key
        "generate" => {
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
        //encrypt/decrypt command
        "crypt" => {
            if args.len() != 5 {
                eprintln!(
                    "Usage: {} crypt <input_file> <key_file> <output_file>",
                    args[0]
                );
                process::exit(1);
            }
            let input_path = &args[2];
            let key_path = &args[3];
            let output_path = &args[4];
            crypt(input_path, key_path, output_path);
        }
        //else
        _ => {
            eprintln!("Unknown command: {}", command);
            display_usage(&args[0]);
            process::exit(1);
        }
    }
}

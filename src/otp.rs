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

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() < 2) {
        //exit if less than 2 args
        process::exit(1);
    }

    let command = &args[1];

    //match statement to eval commands
    match command.as_str() {
        //generate key
        "generate" => {}
        //encrypt command
        "encrypt" => {}
        //else
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

use qrcode::QrCode;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

//salsa20 quarter round with mod 2^32 add, xor update, and bit shifts
fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(7);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(9);
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(13);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(18);
}

//64 bit block of salsa20
fn salsa20_block(key: &[u8; 32], nonce: &[u8; 8], count: u64) -> [u8; 64] {
    let mut state = [0u32; 16];

    //nothing-up-my-sleeve numbers that the wikipedia example uses
    state[0] = u32::from_le_bytes(*b"expa");
    state[5] = u32::from_le_bytes(*b"nd 3");
    state[10] = u32::from_le_bytes(*b"2-by");
    state[15] = u32::from_le_bytes(*b"te k");

    //load key into upper and lower slots
    for i in 0..4 {
        state[1 + i] = u32::from_le_bytes(key[i * 4..(i + 1) * 4].try_into().unwrap());
    }
    for i in 0..4 {
        state[11 + i] =
            u32::from_le_bytes(key[16 + (i * 4)..16 + ((i + 1) * 4)].try_into().unwrap());
    }

    //load in nonce in index 6 and 7
    state[6] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
    state[7] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());

    //load counter
    let counter_bytes = count.to_le_bytes();
    state[8] = u32::from_le_bytes(counter_bytes[0..4].try_into().unwrap());
    state[9] = u32::from_le_bytes(counter_bytes[4..8].try_into().unwrap());

    //copy and scramble the state
    let mut temp = state.clone();
    for _ in 0..10 {
        //odd rounds
        quarter_round(&mut temp, 0, 4, 8, 12);
        quarter_round(&mut temp, 5, 9, 13, 1);
        quarter_round(&mut temp, 10, 14, 2, 6);
        quarter_round(&mut temp, 15, 3, 7, 11);

        //even round
        quarter_round(&mut temp, 0, 1, 2, 3);
        quarter_round(&mut temp, 5, 6, 7, 4);
        quarter_round(&mut temp, 10, 11, 8, 9);
        quarter_round(&mut temp, 15, 12, 13, 14);
    }

    //add copy to original state
    for i in 0..16 {
        state[i] = state[i].wrapping_add(temp[i]);
    }
    //convert the 16 u32s to a 64 byte block
    let mut block = [0u8; 64];
    for i in 0..16 {
        //convert u32 to le bytes
        let bytes = state[i].to_le_bytes();
        //memcpy to block in 4 bytes
        for j in 0..4 {
            //(current word * 4) + current byte is the next index
            block[(i * 4) + j] = bytes[j];
        }
    }
    return block;
}

//preforms the salsa20 operation for both encrypt and decrypt
fn salsa20(input: &[u8], key_path: &str, is_encrypt: bool) -> Vec<u8> {
    let mut key_file = File::open(key_path).unwrap_or_else(|err| {
        eprintln!("Failed to open key file '{}': {}", key_path, err);
        process::exit(1);
    });

    let mut key = [0u8; 32];
    //key must be 32 bytes
    key_file.read_exact(&mut key).unwrap_or_else(|err| {
        eprintln!("Key file must be exactly 32 bytes: {}", err);
        process::exit(1);
    });

    let mut nonce = [0u8; 8];
    let mut to_ret = Vec::new();
    let to_cipher;

    //if encrypting take 8 bytes from /dev/urandom for the nonce
    if is_encrypt {
        let mut urandom = File::open("/dev/urandom").unwrap();
        urandom.read_exact(&mut nonce).unwrap();

        //add nonce to to return
        to_ret.extend_from_slice(&nonce);
        to_cipher = input;
    }
    //if decrypting the first 8 bytes should be the nonce
    else {
        if input.len() < 8 {
            eprintln!("ciphertext is too short to have a nonce");
            process::exit(1);
        }
        nonce.copy_from_slice(&input[0..8]);
        to_cipher = &input[8..];
    }

    //xor loop
    let mut count: u64 = 0;
    //seperate data into chunks and xor with salsa20 block
    for chunk in to_cipher.chunks(64) {
        let stream_block = salsa20_block(&key, &nonce, count);
        for (i, byte) in chunk.iter().enumerate() {
            to_ret.push(byte ^ stream_block[i]);
        }
        count += 1;
    }

    return to_ret;
}

//uses linux's built in CSPRNG to generate the key
//(possible flaw if linux is subverted but better than using a non cryptographically safe PRNG)
fn generate_key(path: &str) {
    //open linux's kernel entropy pool
    let mut urandom = File::open("/dev/urandom").unwrap_or_else(|err| {
        eprintln!("Failed to open /dev/urandom: {}", err);
        process::exit(1);
    });

    //create a buffer to hold random 32 bytes for salsa20
    let mut key_data = vec![0u8; 32];
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

    println!("Generated 32 bit key to '{}'", path)
}

//encrypts the input message using the key file and displays a qr code for a device to scan.
fn encrypt(input_path: &str, key_path: &str) {
    //open file and move to a vector
    let mut input_file = File::open(input_path).unwrap();
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).unwrap();

    let encrypted = salsa20(&input_data, key_path, true);

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
    println!("Safe to scan with a dirty device");
}

//decrypts input file using the key file and prints to terminal
fn decrypt(input_path: &str, key_path: &str) {
    let mut input_file = File::open(input_path).unwrap();
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).unwrap();

    let output_data = salsa20(&input_data, key_path, false);

    let message =
        String::from_utf8(output_data).expect("Decryption failed to decode the file to utf8.");
    println!("---------DECODED MESSAGE---------");
    println!("{}", message);
}

fn display_usage(name: &str) {
    println!("Resistance Project\n");
    println!("Usage:");
    println!("  {} -generate|-g <output_key_file>", name);
    println!("  {} -encrypt|-e <input_file> <key_file>", name);
    println!("  {} -decrypt|-d <input_file> <key_file>", name);
    println!("  {} --help", name);
    println!("\nOptions:");
    println!(
        "  -g <size_in_bytes> <output_key_file> generates a key of 32 bits  to <output_key_file>."
    );
    println!(
        "  -e <input_file> <key_file> displays a QR code of <input_file> encrypted with <key_file>. Max size of file size 2953 bytes."
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
            if args.len() != 3 {
                eprintln!(
                    "Usage: {} generate <size_in_bytes> <output_key_file>",
                    args[0]
                );
                process::exit(1);
            }

            let path = &args[2];
            generate_key(path);
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

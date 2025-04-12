use serde_json::Value;
use std::{env::args, process::exit};
use track_payments_rust::{crypto::decrypt_str, fs::read_file};

fn main() {
    // get paths
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        return;
    }
    let key_file = &args[1];
    let encrypted_file = &args[2];

    // read files
    let key_data = match read_file(key_file) {
        Ok(res) => res,
        Err(_) => exit(0),
    };
    let encrypted_data = match read_file(encrypted_file) {
        Ok(res) => res,
        Err(_) => exit(0),
    };

    // decrypt file
    let decrypted_data = match decrypt_str(&key_data, &encrypted_data) {
        Ok(res) => res,
        Err(_) => {
            println!("{}", String::from_utf8_lossy(&encrypted_data));
            exit(0);
        }
    };

    // if file is json, format it, otherwise just print it
    let json_value: Value = match serde_json::from_str(&decrypted_data) {
        Ok(res) => res,
        Err(_) => {
            println!("{decrypted_data}");
            exit(0);
        }
    };
    let json_str = match serde_json::to_string_pretty(&json_value) {
        Ok(res) => res,
        Err(_) => {
            println!("{decrypted_data}");
            exit(0);
        }
    };
    println!("{json_str}");
}

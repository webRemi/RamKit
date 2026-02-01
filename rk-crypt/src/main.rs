use std::env;
use terminal_banner::Banner;
use colored::Colorize;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305};
use sha2::{Sha256, Digest};
use chacha20poly1305::aead::generic_array::GenericArray;

fn main() {
    let banner = Banner::new()
        .text("rk-crypt from RamKit".bold().to_string().into())
        .text(format!("Powered by @{}", "ASX".bold()).into())
        .render();
    println!("{}", banner);

    let args: Vec<String> = env::args().collect();
    let mut secret = String::new();

    match args.len() {
        6 => {
            secret = args[5].parse().expect("Invalid secret");
        }
        _ => menu(),
    }

    let file_path = &args[3];

    let key = generate_sha2(secret);

    match args[1].as_str() {
        "encrypt" => encrypt(file_path.to_string(), key),
        "decrypt" => decrypt(file_path.to_string(), key),
        _ => {
            menu();
            return();
        }
    }.expect("Failed");
}

fn menu() {
    println!("{} -f <file> [options]", "rk-crypt".bold().cyan());
    println!("Options:");
    println!("-s\t\tSecret");
    std::process::exit(0);
}

fn encrypt(file_path: String, key: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read(&file_path).expect("Failed");    
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, contents.as_ref()).map_err(|e| e.to_string())?;

    let mut path = PathBuf::from(file_path.clone());
    path.set_extension("rk");
    let mut file = File::create(&path)?;
    file.write_all(&nonce)?;
    file.write_all(&ciphertext)?;
    fs::remove_file(file_path)?;
    Ok(())
}

fn decrypt(file_path: String, key: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(&key.into());
    let data = fs::read(&file_path).expect("Failed");
    let ciphertext = &data[12..];
    let nonce = GenericArray::from_slice(&data[..12]);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).map_err(|e| e.to_string())?;
    
    let mut path = PathBuf::from(file_path.clone());
    path.set_extension("txt");
    let mut file = File::create(&path)?;
    file.write_all(&plaintext)?;
    fs::remove_file(file_path)?;
    Ok(())
}

fn generate_sha2(secret: String) -> [u8; 32] {
    let result = Sha256::digest(secret.as_bytes());
    result.into()
}

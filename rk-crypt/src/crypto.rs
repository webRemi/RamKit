use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305};
use sha2::{Sha256, Digest};
use chacha20poly1305::aead::generic_array::GenericArray;

pub fn encrypt(file_path: String, key: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read(&file_path)?;
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

pub fn decrypt(file_path: String, key: [u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(&key.into());
    let data = fs::read(&file_path)?;
    if data.len() < 12 {
        return Err("Not valid crypted file".into());
    }
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

pub fn generate_sha2(secret: String) -> [u8; 32] {
    let result = Sha256::digest(secret.as_bytes());
    result.into()
}

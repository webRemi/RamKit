mod banner;
mod args;
mod crypto;

use colored::Colorize;

fn main() {
    banner::print_banner();

    let some_args = args::extract_args();

    let secret = some_args.secret;

    let file_path = some_args.file;

    let key = crypto::generate_sha2(secret);

    match some_args.mode.as_str() {
        "encrypt" => {
            match crypto::encrypt(file_path.to_string(), key) {
                Ok(_) => println!("[{}] File encrypted", "+".green()),
                Err(e) => eprintln!("[{}] Failed to encrypt: {}", "-".red(), e),
            }
        }
        "decrypt" => {
            match crypto::decrypt(file_path.to_string(), key) {
                Ok(_) => println!("[{}] File decrypted", "+".green()),
                Err(e) => eprintln!("[{}] Failed to decrypt: {}", "-".red(), e),
            }
        }
        _ => {
            eprintln!("[{}] Unknow mode", "!".yellow());
        }
    }
}


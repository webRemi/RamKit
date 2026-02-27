use hex_literal::hex;
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use sha2::Sha256;

let password = b"password";
let salt = b"salt";
// number of iterations
let n = 600_000;
// Expected value of generated key
let expected = hex!("669cfe52482116fda1aa2cbe409b2f56c8e45637");

println!(expected);

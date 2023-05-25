use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::engine::general_purpose;
use base64::Engine;

pub fn create_password_hash(password: &[u8]) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt).unwrap();
    general_purpose::STANDARD_NO_PAD.encode(password_hash.to_string())
}

pub fn verify_password(password: &[u8], password_hash: String) -> bool {
    let argon2 = Argon2::default();
    let password_hash_decoded = general_purpose::STANDARD_NO_PAD.decode(password_hash);
    match password_hash_decoded {
        Ok(password_hash_vector) => {
            let password_string = String::from_utf8(password_hash_vector).unwrap();
            let parsed_hash = PasswordHash::new(&password_string).unwrap();
            argon2.verify_password(password, &parsed_hash).is_ok()
        }
        Err(_) => false,
    }
}

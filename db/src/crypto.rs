use pbkdf2::{
    Pbkdf2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn to_hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn check_hash(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(parsed_hash) => parsed_hash,
        Err(_) => return false,
    };

    Pbkdf2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

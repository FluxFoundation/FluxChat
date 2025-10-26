use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(pass: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19456, 2, 1, None).expect("argon2 params"),
    );

    let password_hash = argon2
        .hash_password(pass.as_bytes(), &salt)
        .expect("argon2 hash")
        .to_string();

    password_hash
}

pub fn verify_password(pass: &str, hash: &str) -> bool {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19456, 2, 1, None).expect("argon2 params"),
    );

    let parsed_hash = PasswordHash::new(&hash).expect("argon2 parse hash");

    argon2
        .verify_password(pass.as_bytes(), &parsed_hash)
        .is_ok()
}

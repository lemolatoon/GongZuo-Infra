use ring::pbkdf2::{self, verify, PBKDF2_HMAC_SHA256};
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;

/// Derive a password using PBKDF2
/// Returns a tuple of (salt, hashed_password)
pub fn derive(password: String) -> anyhow::Result<(String, String)> {
    let mut salt = [0u8; 16];
    SystemRandom::new()
        .fill(&mut salt)
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut hashed_password = [0u8; 32];
    pbkdf2::derive(
        PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut hashed_password,
    );
    let salt = hex::encode(salt);
    let hashed_password = hex::encode(hashed_password);

    Ok((salt, hashed_password))
}

/// Verify a password using PBKDF2
pub fn verify_with_salt(
    salt: String,
    hashed_password: String,
    password: String,
) -> anyhow::Result<bool> {
    let salt = hex::decode(salt)?;
    let hashed_password = hex::decode(hashed_password)?;

    Ok(verify(
        PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &hashed_password,
    )
    .is_ok())
}

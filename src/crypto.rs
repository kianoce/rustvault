use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use dialoguer::Password;
use sha2::{Digest, Sha256};
use std::fmt;
use std::string::FromUtf8Error;

/// Prompts the user to enter a master password and generates an encryption key from it.
///
/// # Returns
///
/// * A `Key<Aes256Gcm>` generated from the user's master password.
pub fn get_key() -> Key<Aes256Gcm> {
    let password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    generate_key_from_password(&password)
}

/// Generates an AES-256-GCM encryption key from the provided password.
///
/// # Arguments
///
/// * `password` - The password to derive the encryption key from.
///
/// # Returns
///
/// * A `Key<Aes256Gcm>` derived from the provided password.
pub fn generate_key_from_password(password: &str) -> Key<Aes256Gcm> {
    let mut hasher = <Sha256 as Digest>::new();
    hasher.update(password);
    let hash = hasher.finalize();

    *Key::<Aes256Gcm>::from_slice(&hash)
}

/// Encrypts the provided data using AES-256-GCM.
///
/// # Arguments
///
/// * `data` - The data to encrypt.
/// * `key` - The encryption key.
///
/// # Returns
///
/// * The encrypted data, with the nonce appended to the end.
pub fn encrypt_data(data: String, key: Key<Aes256Gcm>) -> Vec<u8> {
    // encrypt the data
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut encrypted_data = cipher.encrypt(&nonce, data.as_ref()).unwrap();
    // add nonce to end of data
    encrypted_data.append(&mut nonce.to_vec());
    encrypted_data
}

/// Decrypts the provided data using AES-256-GCM.
///
/// # Arguments
///
/// * `data` - The encrypted data with nonce appended.
/// * `key` - The decryption key.
///
/// # Returns
///
/// * The decrypted data as a String.
///
/// # Errors
///
/// Returns an error if decryption fails.
pub fn decrypt_data(mut data: Vec<u8>, key: Key<Aes256Gcm>) -> Result<String, DecryptError> {
    // Ensure data length is sufficient to contain the nonce
    let length = data.len();
    if length < 12 {
        return Err(DecryptError::InsufficientData);
    }

    // get nonce from encrypted data
    let nonce_vec = data.split_off(length - 12);
    let nonce = Nonce::from_slice(&nonce_vec);

    let cipher = Aes256Gcm::new(&key);
    // decrypting the data
    let result = cipher.decrypt(nonce, data.as_ref())?;
    let decrypted_data = String::from_utf8(result)?;
    Ok(decrypted_data)
}

#[derive(Debug)]
pub enum DecryptError {
    AesGcm(aes_gcm::Error),
    Utf8Error(FromUtf8Error),
    InsufficientData,
}

impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecryptError::AesGcm(_) => write!(f, "Master password is incorrect"),
            DecryptError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            DecryptError::InsufficientData => write!(f, "Insufficient data length"),
        }
    }
}

impl std::error::Error for DecryptError {}

impl From<aes_gcm::Error> for DecryptError {
    fn from(err: aes_gcm::Error) -> DecryptError {
        DecryptError::AesGcm(err)
    }
}

impl From<FromUtf8Error> for DecryptError {
    fn from(err: FromUtf8Error) -> DecryptError {
        DecryptError::Utf8Error(err)
    }
}

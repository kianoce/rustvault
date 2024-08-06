use std::collections::{btree_map::Entry, BTreeMap};
use std::error::Error;

pub mod args;
pub mod crypto;
pub mod file;

use args::{Commands, RustVaultArgs};
use file as rustvault_file;

// Encryption related
use aes_gcm::{Aes256Gcm, Key};

// Clipboard
use arboard::Clipboard;

// Dialog for input / confirmation
use dialoguer::{Confirm, Input, Password, Select};

/// Runs the main logic for RustVault.
///
/// # Arguments
///
/// * `args` - Command-line arguments specifying the operation to perform.
/// * `key` - Encryption key for encrypting/decrypting data.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
pub fn run(args: RustVaultArgs, mut key: Key<Aes256Gcm>) -> Result<(), Box<dyn Error>> {
    let encrypted_data = rustvault_file::get_encrypted_data();
    let mut credentials_map = generate_map_from_encrypted_data(encrypted_data, key)?;

    match &args.command {
        Some(Commands::Get(args)) => get_credentials(&credentials_map, &args.id)?,
        Some(Commands::Add(args)) => add_credentials(&mut credentials_map, &args.id)?,
        Some(Commands::Delete(args)) => delete_credentials(&mut credentials_map, &args.id)?,
        Some(Commands::Modify(args)) => modify_credentials(&mut credentials_map, &args.id)?,
        Some(Commands::List) => list_credential_ids(&credentials_map)?,
        Some(Commands::ChangePassword) => {
            key = change_master_password()?;
            println!("Master password updated.");
        }
        None => {}
    }

    let data = convert_map_to_string(credentials_map);
    let encrypted_data = crypto::encrypt_data(data, key);
    rustvault_file::save_to_file(encrypted_data);

    Ok(())
}

/// Generates a BTreeMap from encrypted data.
///
/// # Arguments
///
/// * `data` - Encrypted data as a byte vector.
/// * `key` - Encryption key for decrypting the data.
///
/// # Returns
///
/// * A BTreeMap containing decrypted password credentials.
fn generate_map_from_encrypted_data(
    data: Vec<u8>,
    key: Key<Aes256Gcm>,
) -> Result<BTreeMap<String, CredentialsEntry>, Box<dyn Error>> {
    if !data.is_empty() {
        let decrypted_data = crypto::decrypt_data(data, key)?;
        return Ok(create_credential_map_from_string(decrypted_data));
    }
    Ok(BTreeMap::new())
}

/// Lists all credential IDs in the BTreeMap.
///
/// # Arguments
///
/// * `data` - Reference to a BTreeMap containing password credentials.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
fn list_credential_ids(data: &BTreeMap<String, CredentialsEntry>) -> Result<(), Box<dyn Error>> {
    println!("--- RustVault ID's ---");
    for key in data.keys() {
        println!("{key}");
    }
    println!("----------------------");
    Ok(())
}

/// Retrieves credentials for a given ID and copies the password to the clipboard.
///
/// # Arguments
///
/// * `credentials_map` - Reference to a BTreeMap containing credentials.
/// * `id` - The ID of the credentials to retrieve.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
fn get_credentials(
    credentials_map: &BTreeMap<String, CredentialsEntry>,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    if let Some(credentials) = credentials_map.get(id) {
        // Unescape semicolons
        let username = unescape_semicolons(&credentials.username);
        let password = unescape_semicolons(&credentials.password);
        println!("--- Credentials for {id} ---");
        println!("username: {}", username);
        println!("password: [hidden] (copied to clipboard)");
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(password)?;
    } else {
        println!("ID '{}' does not exist", id);
    }
    Ok(())
}

/// Adds new credentials to the BTreeMap.
///
/// # Arguments
///
/// * `credentials_map` - Mutable reference to a BTreeMap containing password credentials.
/// * `id` - The ID for the new credentials.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
fn add_credentials(
    credentials_map: &mut BTreeMap<String, CredentialsEntry>,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        println!("Invalid ID: only a-z, A-Z, '-', and '_' are allowed");
        return Ok(());
    }
    if credentials_map.contains_key(id) {
        println!("Credentials with given ID already exist");
        return Ok(());
    }
    let mut username = Input::new()
        .with_prompt("Enter username/email")
        .interact_text()?;
    let mut password = Password::new()
        .with_prompt("Enter password")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()?;

    // Escape semicolons
    username = escape_semicolons(&username);
    password = escape_semicolons(&password);

    credentials_map.insert(id.to_string(), CredentialsEntry { username, password });
    println!("Added credentials with ID '{}'", id);
    Ok(())
}

/// Deletes credentials for a given ID from the BTreeMap.
///
/// # Arguments
///
/// * `credentials_map` - Mutable reference to a BTreeMap containing password credentials.
/// * `id` - The ID of the credentials to delete.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
fn delete_credentials(
    credentials_map: &mut BTreeMap<String, CredentialsEntry>,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    if !credentials_map.contains_key(id) {
        println!("Credentials with id '{}' does not exist.", id);
        return Ok(());
    }

    let prompt = format!("Do you want to delete credentials with id '{}'?", id);
    if Confirm::new().with_prompt(prompt).interact()? {
        credentials_map.remove(id);
        println!("Deleted credentials with id '{}'", id);
    }
    Ok(())
}

/// Modifies credentials for a given ID in the BTreeMap.
///
/// # Arguments
///
/// * `credentials_map` - Mutable reference to a BTreeMap containing password credentials.
/// * `id` - The ID of the credentials to modify.
///
/// # Returns
///
/// * `Ok(())` if the operation is successful.
/// * `Err(Box<dyn Error>)` if an error occurs.
fn modify_credentials(
    credentials_map: &mut BTreeMap<String, CredentialsEntry>,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    match credentials_map.entry(id.to_string()) {
        Entry::Vacant(_) => {
            println!("Credentials with id '{}' does not exist.", id);
            return Ok(());
        }
        Entry::Occupied(mut credential_entry) => {
            println!("Modifying credentials with ID - {}", credential_entry.key());
            let selection = Select::new()
                .with_prompt("What would you like to modify?")
                .item("password")
                .item("username")
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    let password = Password::new()
                        .with_prompt("Enter new password")
                        .with_confirmation("Confirm password", "Passwords don't match")
                        .interact()?;
                    credential_entry.get_mut().password = password;
                    println!("Password updated.");
                }
                1 => {
                    let username = Input::new()
                        .with_prompt("Enter new username")
                        .interact_text()?;
                    credential_entry.get_mut().username = username;
                    println!("Username updated.");
                }
                _ => {
                    println!("Nothing selected");
                }
            }
        }
    }

    Ok(())
}

/// Prompts the user to change the master password and generates a new encryption key.
///
/// # Returns
///
/// * A new encryption key generated from the new master password.
fn change_master_password() -> Result<Key<Aes256Gcm>, Box<dyn Error>> {
    let password = Password::new()
        .with_prompt("Enter new master password")
        .with_confirmation("Confirm master password", "Passwords don't match")
        .interact()?;

    Ok(crypto::generate_key_from_password(&password))
}

/// Represents the credentials for a password entry.
///
/// # Fields
///
/// * `username` - The username or email associated with the credentials.
/// * `password` - The password associated with the credentials.
#[derive(Debug)]
pub struct CredentialsEntry {
    pub username: String,
    pub password: String,
}

/// Creates a BTreeMap of password credentials from a string.
///
/// # Arguments
///
/// * `text` - The input string containing serialized password credentials.
///
/// # Returns
///
/// * A BTreeMap containing deserialized password credentials.
fn create_credential_map_from_string(text: String) -> BTreeMap<String, CredentialsEntry> {
    let mut credentials_map: BTreeMap<String, CredentialsEntry> = BTreeMap::new();
    for line in text.lines() {
        let line_vec: Vec<&str> = line.split(';').collect();
        let password_details = CredentialsEntry {
            username: String::from(line_vec[1]),
            password: String::from(line_vec[2]),
        };
        credentials_map.insert(String::from(line_vec[0]), password_details);
    }
    credentials_map
}

/// Converts a BTreeMap of password credentials to a string.
///
/// # Arguments
///
/// * `map` - The BTreeMap containing password credentials.
///
/// # Returns
///
/// * A string containing serialized password credentials.
fn convert_map_to_string(map: BTreeMap<String, CredentialsEntry>) -> String {
    let mut s = String::new();

    for (id, credentials) in map.into_iter() {
        let row = format!("{};{};{}\n", id, credentials.username, credentials.password);
        s.push_str(&row);
    }

    s = s.trim_end().to_string();
    s
}

// Helper Functions
fn escape_semicolons(input: &String) -> String {
    input.replace(";", "###semicolon###")
}

fn unescape_semicolons(input: &String) -> String {
    input.replace("###semicolon###", ";")
}

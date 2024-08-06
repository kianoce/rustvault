use dirs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn get_encrypted_data() -> Vec<u8> {
    let path = get_path();
    let file = get_file(path.as_path());
    let mut reader = BufReader::new(file);
    let mut encrypted_data: Vec<u8> = vec![];
    reader
        .read_to_end(&mut encrypted_data)
        .expect("Unable to read data");
    encrypted_data
}

// Retrieve file
pub fn get_file(path: &Path) -> File {
    // read file
    let pwfile = File::open(path);

    let file = match pwfile {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                create_directory(path.parent().unwrap()).expect("Unable to create directory.");
                create_file(path)
            }
            _ => panic!("There was an error retrieving the credentials file"),
        },
    };
    file
}

fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

fn create_file(path: &Path) -> File {
    File::create_new(path).unwrap()
}

pub fn save_to_file(data: Vec<u8>) {
    let path = get_path();
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);
    writer
        .write_all(&data)
        .expect("Failed to write encrypted data to file");
}

fn get_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".rustvault");
    path.push("data");
    path
}

# RustVault - A Secure Password Manager

## Quick Links

- [Project Overview](#project-overview)
- [Setup Guide](#setup-guide)
- [Available Commands](#available-commands)
- [Backup and Data Management](#backup-and-data-management)

## Project Overview
RustVault was inspired by my desire to deepen my understanding of Rust after completing "The Rust Programming Language" book. Having used password managers like LastPass and Bitwarden, I wanted a solution that didn't require email registration, internet access, and provided a greater sense of security through transparent handling of credentials. Rust proved to be an ideal choice for addressing these needs, allowing me to learn about cryptography and design a tool I use daily.

### Requirements
These were my personal requirements for the project:

- Store username/passwords
- Ask for master password per session
- Create/read/modify/delete passwords
- Encrypt with similar techniques to LastPass
- Store all passwords in an encrypted file. File is never decrypted; passwords are only accessible via the application.


## Design and Architecture
### Architecture Overview
- **Data Storage**: Passwords and other sensitive data are encrypted and stored locally.
- **Encryption**: Uses AES-256-GCM for encrypting and decrypting the stored data.
- **User Input**: Validates and sanitizes user input to prevent security vulnerabilities.

### Technology Stack
- **Language**: Rust
- **Encryption**: AES-256-GCM
- **Storage**: Local file system

## Security
### Data Encryption
RustVault uses AES-256-GCM for encrypting passwords and other sensitive data before storing them in a local file. This ensures that even if the storage file is accessed by unauthorized users, the data remains protected.

### User Input Validation
Ensures that user input only contains valid characters to prevent injection attacks and other security vulnerabilities.

## Features
### Password Management
- **Encryption**: Securely encrypts passwords with a master password before storing them.
- **Decryption**: Decrypts passwords when needed, ensuring they are only accessible to authorized users.
- **Data Validation**: Validates user input to ensure it conforms to expected formats.
- **Functions**: List, Add, Get, Delete and Modify credentials. Ability to change the master password.

## Learning Outcomes
### Challenges
- **Encryption and Decryption**: Implementing secure encryption and decryption mechanisms.
- **User Input Validation**: Ensuring that user input is sanitized and validated to prevent security vulnerabilities.
- **Rust**: Being new to rust learning about different data types available, their functions and the borrow checker were all fun challenges to work with.

### Skills Acquired
- **Rust Programming**: Gained experience in writing secure and efficient Rust code.
- **Cryptography**: Understanding and implementing encryption and decryption using AES-256-GCM.
- **Security Best Practices**: Ensuring that sensitive data is securely managed and protected.

## Conclusion
RustVault is a secure and reliable command line password manager built with Rust. It ensures that sensitive data is encrypted and stored securely, providing users with a safe way to manage their passwords.

### Future Improvements
- **Cloud Storage**: Add support for cloud-based storage options to sync passwords across devices.
- **Saved Username/Emails**: Ability to select common emails or usernames to avoid typing them when adding new credentials
- **Multiple Operations**: Enable users to perform multiple operations in a single command line execution, instead of just one.
- **Password Generation**: Add features like password generation, password strength analysis.
- **Tags/Categories**: Allow users to organize and search credentials using tags or categories.

## Setup Guide
This guide will walk you through setting up the RustVault project.

> **Note:** This project has been developed and tested on macOS. There may be issues when running on Windows or Linux, and additional adjustments might be needed.


### Prerequisites
- Rust and Cargo installed on your machine.

### Steps
#### 1. Clone the Repository

Clone the RustVault repository to your local machine:

```bash
git clone https://github.com/kianoce/rustvault.git
cd rustvault
```

#### 2. Build the Project

Build the project using cargo

```bash
cargo build --release
```

#### 3. Copy Executable to /usr/local/bin:

Copy the built executable to `/usr/local/bin` to make it available system-wide.

```bash
sudo cp target/release/rustvault /usr/local/bin/
```

#### 4. Run the program
You can now run the program from anywhere in the terminal using:

```bash
rustvault
```

## Features

### Available Commands
- **list**: Lists all available IDs.
  ```bash
  rustvault list
  ```
- **add**: Add credentials with the given ID.
  ```bash
  rustvault add <id>
  ```
- **get**: Get credentials by ID.
  ```bash
  rustvault get <id>
  ```
- **delete**: Deletes existing credentials.
  ```bash
  rustvault delete <id>
  ```
- **modify**: Edits the username/email or password of existing credentials.
  ```bash
  rustvault modify <id>
  ```
- **change-password**: Changes the master password
  ```bash
  rustvault change-password
  ```

## Backup and Data Management

### Location of Encrypted Credentials File
The encrypted credentials are stored in a file located at `~/.rustvault/data`. This file contains all your saved credentials in an encrypted format.

### Backup Instructions
To backup your encrypted credentials file, simply copy the file to a secure location. You can use the following command to create a backup:

```bash
cp ~/.rustvault/data /path/to/your/backup/location/rustvault_backup
```

Make sure to store the backup in a secure place, such as an external hard drive or a secure cloud storage service.

### Restoring from Backup
To restore your credentials from a backup, copy the backup file back to the original location:
```bash
cp /path/to/your/backup/location/rustvault_backup ~/.rustvault/data
```
After restoring the file, you can use RustVault as usual with your saved credentials.






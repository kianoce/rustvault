use clap::Parser;
use rustvault::args::RustVaultArgs;
use rustvault::crypto;
use std::process;

fn main() {
    let args = RustVaultArgs::parse();
    let key = crypto::get_key();

    if let Err(e) = rustvault::run(args, key) {
        eprintln!("Error: {e}");
        process::exit(1)
    }
}

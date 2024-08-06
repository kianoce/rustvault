use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustVaultArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add login credentials
    Add(AddArgs),

    /// Get login credentials by an ID
    Get(GetArgs),

    /// Modify login credentials
    Modify(ModifyArgs),

    /// Delete login credentials
    Delete(DeleteArgs),

    /// List all IDs
    List,

    /// Change Master Password
    ChangePassword,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// ID
    pub id: String,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// ID
    pub id: String,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// ID
    pub id: String,
}

#[derive(Debug, Args)]
pub struct ModifyArgs {
    /// ID
    pub id: String,
}

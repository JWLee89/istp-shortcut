use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::common::constants::app::DEFAULT_DB_PATH;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A simple application to create shortcuts on the terminal"
)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Default location where database will be stored.
    #[arg(default_value=get_default_db_path().into_os_string())]
    pub db_path: String,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all the subcommands stored
    #[clap(visible_alias("ls"))]
    List {
        /// Search by name
        name: Option<String>,
    },
    #[clap(visible_alias("sv"))]
    Save {
        /// The name of the command to save
        name: String,
        /// The command to save
        command: String,
        /// Will disable sanity checks. Use at your own risk.
        /// This will bypass sanity checks that prevent injection.
        disable_checks: Option<bool>,
    },
}

fn get_default_db_path() -> PathBuf {
    let mut default_path = PathBuf::new();
    default_path.push(DEFAULT_DB_PATH);
    default_path
}

#[cfg(test)]
mod test {
    use super::*;

    // Default path should be a pathbuf object
    // with same value as default DB path string.
    #[test]
    fn test_get_default_path() {
        let path = get_default_db_path();
        assert_eq!(path.as_os_str(), DEFAULT_DB_PATH);
    }
}

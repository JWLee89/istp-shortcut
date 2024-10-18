use clap::Parser;
use color_eyre::eyre::Result;
use lazy_shortcut::{
    arg::cli::{Cli, Commands},
    common::debug::init_tracing,
    database::sqlite::db::{connect, create_db_if_not_exists},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    // Parse CLI outputs
    let cli = Cli::parse();

    // Initialize database
    create_db_if_not_exists(&cli.db_path).await?;
    let url = format!("sqlite:///{}", &cli.db_path);
    // Connect to DB
    let _ = connect(&url).await;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::List { name }) => {
            println!("'myapp add' was used, name is: {:?}", name)
        },
        // This will insert / update an entry in the database
        Some(Commands::Save {
            name,
            command,
            disable_checks,
        }) => {
            // 1. Check if command exists by name
            // 2. If exists, update existing
            // 3. If new, insert
            println!(
                "Save was used: name is: {:?}. Command is: {:?}, disable checks: {:?}",
                name, command, disable_checks
            )
        },
        None => {
            println!("Default subcommand");
        },
    }
    Ok(())
}

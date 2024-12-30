use clap::Parser;
use color_eyre::eyre::Result;
use lazy_shortcut::{
    arg::cli::{Cli, Commands},
    command::base::CommandBuilder,
    common::debug::init_tracing,
    database::{
        common::CommandStore,
        sqlite::{command::SqliteCommandStore, db::create_db_if_not_exists},
    },
};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    // Parse CLI outputs
    let cli = Cli::parse();
    // Initialize database
    if let Err(e) = create_db_if_not_exists(&cli.db_path).await {
        debug!(
            "Did not create new database. Root cause: {}",
            e.root_cause()
        );
    }
    let url = format!("sqlite://{}", &cli.db_path);
    debug!("CLI DB path: {}", cli.db_path);
    debug!("URL: {}", url);
    // Connect to DB
    let command_store: SqliteCommandStore = SqliteCommandStore::from_str(&url).await?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Exec { name }) => {
            debug!("`exec` was used. Attempting to execute: {:?}", name);
            let command = command_store.get(name).await?;
            println!("Command found: {:?}", command);
        },
        Some(Commands::List { name }) => {
            debug!("'myapp add' was used, name is: {:?}", name);
            match name {
                Some(command_name) => {
                    // Query database
                    let command = command_store.get(command_name).await;
                    if let Ok(cmd) = command {
                        println!("command: {command_name} found. {:?}", cmd);
                    } else {
                        println!("Command: {command_name} not found");
                    }
                },
                None => {
                    let commands = command_store.get_all().await?;
                    for command in commands {
                        println!(
                            "Command: {}, Statement: {}",
                            command.name, command.statement
                        );
                    }
                },
            }
        },
        // This will insert entry in the database
        Some(Commands::Add {
            name,
            statement,
            description,
        }) => {
            debug!(
                "Save was used: name is: {:?}. Command to execute is: {:?}",
                name, statement
            );
            let mut builder = CommandBuilder::new()
                .name(name.to_string())
                .statement(statement.to_string());

            // Clone here is only for one command and should not be too costly
            // so we will keep it for now.
            if let Some(desc) = description {
                builder.clone().description(desc.to_owned());
            }
            let command = builder.build()?;
            // This should create the command. Since we did not pass ownership,
            // we can continue using the command object until it goes out of scope.
            command_store.create(&command).await?;
        },
        Some(Commands::Update {}) => {
            debug!("Update was called");
        },
        None => {
            debug!("Default subcommand");
        },
    }
    Ok(())
}

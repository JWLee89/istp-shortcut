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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    // Parse CLI outputs
    let cli = Cli::parse();
    // Initialize database
    // TODO: Create specific errors to handle case where DB cannot be overwritten
    if let Err(e) = create_db_if_not_exists(&cli.db_path).await {
        println!(
            "Did not create new database. Root cause: {}",
            e.root_cause()
        );
    }
    let url = format!("sqlite://{}", &cli.db_path);
    println!("CLI DB path: {}", cli.db_path);
    println!("URL: {}", url);
    // Connect to DB
    let command_store = SqliteCommandStore::from_str(&url).await?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::List { name }) => {
            println!("'myapp add' was used, name is: {:?}", name);
            match name {
                Some(command_name) => {
                    println!("command: {command_name}")
                },
                None => {
                    let commands = command_store.get_all().await?;
                    for command in commands {
                        println!("Command: {}", command.id);
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
            println!(
                "Save was used: name is: {:?}. Command to execute is: {:?}",
                name, statement
            );
            let mut builder = CommandBuilder::new()
                .name(name.to_string())
                .statement(statement.to_string());

            // TODO: get rid of clone later
            if let Some(desc) = description {
                builder.clone().description(desc.to_owned());
            }
            let command = builder.build()?;
            // This should create the command. Since we did not pass ownership,
            // we can continue using the command object until it goes out of scope.
            command_store.create(&command).await?;
        },
        Some(Commands::Update {}) => {
            println!("Update was called");
        },
        None => {
            println!("Default subcommand");
        },
    }
    Ok(())
}

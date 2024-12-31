use color_eyre::eyre::Result;
use tracing::{debug, info};

use crate::{
    arg::cli::{Cli, Commands},
    command::base::CommandBuilder,
    database::common::CommandStore,
};

/// Application runtime that keeps track fo states.
#[derive(Debug)]
pub struct App<T>
where
    T: CommandStore,
{
    // Used to interat with the command store (a database).
    // For local use, we will interact with local SQLite command store
    command_store: T,
    cli: Cli,
}

impl<T> App<T>
where
    T: CommandStore,
{
    pub fn new(command_store: T, cli: Cli) -> Self {
        Self { command_store, cli }
    }

    /// Process parsed command and do compute.
    pub async fn run(&self) -> Result<()> {
        // You can check for the existence of subcommands, and if found use their
        // matches just as you would the top level cmd
        match &self.cli.command {
            Some(Commands::Exec { name }) => {
                debug!("`exec` was used. Attempting to execute: {:?}", name);
                let command = self.command_store.get(name).await?;
                println!("Command found: {:?}", command);
            },
            Some(Commands::List { name }) => {
                debug!("'myapp add' was used, name is: {:?}", name);
                match name {
                    Some(command_name) => {
                        // Query database
                        let command = self.command_store.get(command_name).await;
                        if let Ok(cmd) = command {
                            println!("command: {command_name} found. {:?}", cmd);
                        } else {
                            println!("Command: {command_name} not found");
                        }
                    },
                    None => {
                        let commands = self.command_store.get_all().await?;
                        info!("running command: list");
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
                self.command_store.create(&command).await?;
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
}

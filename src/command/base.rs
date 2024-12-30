use color_eyre::{eyre::OptionExt, Result};
use sqlx::types::chrono::{self, Utc};

pub trait Executable {
    /// Execute the command
    fn execute(&self) -> CommandResult;
    // we will
    fn parse(&self) {}
}

type DateTimeType = chrono::NaiveDateTime;
type IdType = i64;

// TODO: Maybe add serde in the future.
#[derive(Debug, PartialEq, Default, sqlx::FromRow)]
#[sqlx(default)]
pub struct Command {
    /// Optional because when inserting,
    /// we don't need to define this, since the ID is auto-increment
    pub id: IdType,
    /// The actual name of the command.
    /// E.g. `ls` is a command name used to list files in a directory.
    pub name: String,
    /// The actual statement that will be executed. E.g.
    /// `ls -l | wc -l`
    pub statement: String,
    /// The user-defined description of what this statement should do
    /// E.g. This statement is used to count the number of lines in a file.
    pub description: Option<String>,
    /// When this command was created
    pub created_at: DateTimeType,
    /// When this command was last modified
    pub updated_at: DateTimeType,
}

// Same as what is in the datase
#[derive(Debug, Clone)]
pub struct CommandBuilder {
    id: IdType,
    name: Option<String>,
    statement: Option<String>,
    description: Option<String>,
    created_at: DateTimeType,
    updated_at: DateTimeType,
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        self.statement.as_str()
    }
}

/// Command is simply the statement we want to execute.
impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)
    }
}

impl CommandBuilder {
    pub fn new() -> Self {
        let utc_now: chrono::NaiveDateTime = Utc::now().naive_utc();
        Self {
            id: 0,
            name: None,
            statement: None,
            description: None,
            created_at: utc_now,
            updated_at: utc_now,
        }
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the statement to run when this command is called
    pub fn statement(mut self, statement: String) -> Self {
        self.statement = Some(statement);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(&mut self) -> Result<Command> {
        let name = self
            .name
            .as_ref()
            .ok_or_eyre("name is not defined")?
            // Need ownership of string, since we will be moving it into the
            // newly build command object
            .to_owned();
        let statement = self
            .statement
            .as_ref()
            .ok_or_eyre(
                "statement is not defined.
            Must be defined to execute command",
            )?
            .to_owned();

        Ok(Command {
            id: self.id,
            name,
            statement,
            description: self.description.take(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl Default for CommandBuilder {
    fn default() -> Self {
        CommandBuilder::new()
    }
}

pub enum CommandError {
    OTHERS,
}

/// The command results will be stored here.
pub struct CommandResult {
    // Store the result from executing the command.
    // result: Result<String, CommandError>,
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    /// Test the command builder
    #[test_case("list", "ls -l", Some("List items in directory"))]
    #[test_case("save", "save something", None)]
    fn test_command_builder(name: &str, statement: &str, description: Option<&str>) {
        let mut builder = CommandBuilder::new()
            .name(name.to_string())
            .statement(statement.to_string());

        // Handle optional description
        if let Some(desc) = description {
            builder = builder.description(desc.to_string());
        }
        let command = builder.build().unwrap();

        // TODO: write a simple macro to handle something like this
        assert_eq!(command.name.as_str(), name);
        assert_eq!(command.statement.as_str(), statement);

        // Handle testing for optional
        let actual: Option<&str> = command.description.as_deref();
        assert_eq!(actual, description);
    }

    #[test_case(None, None)]
    #[test_case(Some("teemo"), None)]
    #[test_case(None, Some(""))]
    fn test_command_builder_failure(name: Option<&str>, statement: Option<&str>) {
        let mut builder = CommandBuilder::new();
        if let Some(n) = name {
            builder = builder.name(n.to_string());
        }
        if let Some(s) = statement {
            builder = builder.statement(s.to_string());
        }
        let result = builder.build();
        if let Ok(_) = result {
            panic!(
                "CommandBuilder.build() failed. Check fn build(). Result object: {:?}",
                result
            )
        }
    }
}

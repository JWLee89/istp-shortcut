pub trait Executable {
    /// Execute the command
    fn execute(&self) -> CommandResult;
    // we will
    fn parse(&self) {}
}

pub struct Command {}

pub enum CommandError {
    OTHERS,
}

/// The command results will be stored here.
pub struct CommandResult {
    // Store the result from executing the command.
    // result: Result<String, CommandError>,
}

use crate::command::base::Command;
use async_trait::async_trait;
use color_eyre::Result;

/// for performing CRUD operations
#[async_trait]
pub trait CommandStore: Sync + Send {
    async fn create(&self, command: &Command) -> Result<()>;
    async fn get(&self, name: &str) -> Result<Command>;
    async fn get_all(&self) -> Result<Vec<Command>>;
    async fn delete(&self, command: Command) -> Result<()>;
    async fn update(&self, name: &str, &command: Command) -> Result<()>;
}

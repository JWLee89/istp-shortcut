use crate::{command::base::Command, database::common::SqliteDataObject};

#[async_trait::async_trait]
impl SqliteDataObject for Command {
    async fn create(&self) {}
    async fn read(&self) {}
    async fn update(&self) {}
    async fn delete(&self) {
        todo!()
    }
}

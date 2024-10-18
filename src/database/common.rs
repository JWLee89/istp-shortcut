use async_trait::async_trait;

/// for performing CRUD operations
#[async_trait]
pub trait SqliteDataObject {
    async fn create(&self);
    async fn read(&self);
    async fn update(&self);
    async fn delete(&self);
}

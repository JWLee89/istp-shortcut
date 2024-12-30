use async_trait::async_trait;
use color_eyre::{eyre::OptionExt, Result};
use sqlx::{Pool, Sqlite};

use crate::{command::base::Command, database::common::CommandStore};

use super::db::connect;

pub struct SqliteCommandStore {
    pool: Pool<Sqlite>,
}

impl SqliteCommandStore {
    pub async fn from_str(db_url: &str) -> Result<Self> {
        let pool = connect(db_url).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl CommandStore for SqliteCommandStore {
    async fn create(&self, command: &Command) -> Result<()> {
        println!("Inserting command: {:?}", command);
        sqlx::query(
            r#"
            INSERT INTO COMMAND
            (name, statement, description)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&command.name)
        .bind(&command.statement)
        .bind(&command.description)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn get(&self, name: &str) -> Result<Command> {
        let option: Option<Command> = sqlx::query_as(
            r#"
            SELECT
            id, name, statement, description,
            created_at, updated_at
            FROM COMMAND
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        option.ok_or_eyre(format!("Command: `{:}` is not defined", name))
    }
    async fn get_all(&self) -> Result<Vec<Command>> {
        let result = sqlx::query_as(
            r#"
            SELECT
            id, name, statement, description,
            created_at, updated_at
            FROM COMMAND
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
    async fn delete(&self, _command: Command) -> Result<()> {
        todo!()
    }
    async fn update(&self, _name: &str, _command: Command) -> Result<()> {
        todo!()
    }
}

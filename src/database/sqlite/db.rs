use std::{fs, path::Path};

use color_eyre::eyre::{eyre, Result};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Error, Pool, Sqlite};

pub fn is_existing_file(path: &str) -> bool {
    let p = Path::new(path);
    p.exists() && p.is_file()
}

// TODO: Allow us to handle different database errors differently
pub enum DatabaseError {
    AlreadyExists,
    InvalidPath, // Path does not point to database. E.g. empty string, wrong path.
}

/// Create the database if it does not exist.
/// The string must not be empty. Otherwise, will raise an error.
pub async fn create_db_if_not_exists(db_path: &str) -> Result<()> {
    if db_path.is_empty() {
        return Err(eyre!("db_path cannot be an empty string"));
    }
    if is_existing_file(db_path) {
        return Err(eyre!("db_path is an existing file. Cannot overwrite."));
    }
    // Create folder if it does not exist
    let parent_path = Path::new(db_path).parent();
    if let Some(parent) = parent_path {
        fs::create_dir_all(parent)?;
        // Create DB after creating folder
        Sqlite::create_database(db_path).await?;
    } else {
        return Err(eyre!("Parent path: {:?} does not exist", parent_path));
    }

    Ok(())
}

/// Connect to the database using the given db_path
/// For more information on Sqlite connection string format,
/// see: https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html
pub async fn connect(db_url: &str) -> Result<Pool<Sqlite>, Error> {
    // Create DB Pool object
    let pool = SqlitePoolOptions::new().connect(db_url).await?;
    // Migrate database if it does not exist.
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;
    use test_case::test_case;

    #[test_case("databasefile.db")]
    #[test_case("some_random_file.txt")]
    #[tokio::test]
    async fn test_edge_case_other_file_exists(db_path: &str) -> Result<()> {
        // Create a file. This should cause failures
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join(db_path);
        let path_str = path.to_str().unwrap();
        File::create(path_str)?;

        let db_exists = create_db_if_not_exists(path_str).await;
        // is an error
        assert!(db_exists.is_err());
        Ok(())
    }

    #[test_case("databasefile.db")]
    #[test_case("db.db")]
    #[test_case("mooomin@hotmail.com")]
    #[tokio::test]
    async fn test_create_db_if_not_exists(db_path: &str) -> Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join(db_path);
        let path_str = path.to_str().unwrap();

        // File should not exist prior to testing
        // TODO: Create more specific test later
        let _ = Sqlite::database_exists(path_str).await;
        // Create the DB
        create_db_if_not_exists(path_str).await.unwrap();

        // It should exist now
        let db_exists = Sqlite::database_exists(path_str).await.unwrap();
        assert!(db_exists);
        Ok(())
    }

    #[test_case("README.md")]
    #[test_case("Cargo.toml")]
    fn test_is_existing_file(file_path: &str) {
        let file_exists = is_existing_file(file_path);
        assert!(file_exists);
    }

    #[test_case("src")]
    #[test_case("teemo.txt")]
    fn test_is_existing_file_false(file_path: &str) {
        let is_not_existing_file = !is_existing_file(file_path);
        assert!(is_not_existing_file);
    }

    #[tokio::test]
    async fn test_create_db_if_not_exists_errors() {
        for invalid_db_path in [".", ""] {
            let result = create_db_if_not_exists(invalid_db_path).await;
            assert!(result.is_err())
        }
    }
}

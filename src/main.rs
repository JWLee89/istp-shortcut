use clap::Parser;
use color_eyre::eyre::Result;
use lazy_shortcut::{
    app::state::App,
    arg::cli::Cli,
    common::debug::init_tracing,
    database::sqlite::{command::SqliteCommandStore, db::create_db_if_not_exists},
};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
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

    // Print info
    let url = format!("sqlite://{}", &cli.db_path);
    debug!("CLI DB path: {}", cli.db_path);
    debug!("URL: {}", url);

    // Create Application Runtime
    let command_store: SqliteCommandStore = SqliteCommandStore::from_str(&url).await?;
    let app: App<SqliteCommandStore> = App::new(command_store, cli);
    app.run().await
}

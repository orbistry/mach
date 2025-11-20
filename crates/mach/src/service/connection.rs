use std::path::Path;

use miette::{Context, IntoDiagnostic};
use sea_orm::{Database, DatabaseConnection};
use tokio::fs;
use turso::Builder;

/// Initialize the local Turso database file and return a SeaORM connection.
pub async fn init_database(path: impl AsRef<Path>) -> miette::Result<DatabaseConnection> {
    let path = path.as_ref();

    ensure_parent_dir(path).await?;

    let path_string = path_to_string(path);

    // Create (or open) the local Turso database file.
    Builder::new_local(&path_string)
        .build()
        .await
        .into_diagnostic()
        .wrap_err("failed to initialize Turso database file")?;

    let url = sqlite_url(&path_string);

    let conn = Database::connect(&url)
        .await
        .into_diagnostic()
        .wrap_err("failed to open SeaORM SQLite connection")?;

    conn.get_schema_registry("mach::entity::*")
        .sync(&conn)
        .await
        .into_diagnostic()
        .wrap_err("failed to synchronize schema via SeaORM entity registry")?;

    Ok(conn)
}

fn sqlite_url(path: &str) -> String {
    format!("sqlite://{path}?mode=rwc")
}

fn path_to_string(path: &Path) -> String {
    path.to_path_buf()
        .into_os_string()
        .to_string_lossy()
        .into_owned()
}

async fn ensure_parent_dir(path: &Path) -> miette::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to create parent directory {}", parent.display()))?;
    }

    Ok(())
}

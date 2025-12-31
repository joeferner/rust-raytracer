use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool, migrate::Migrator, sqlite::SqliteConnectOptions};
use std::{path::Path, str::FromStr};

pub mod project_repository;
pub mod user_repository;

pub type DbPool = Pool<Sqlite>;

pub async fn create_db_pool(sqlite_connection_string: &str) -> Result<DbPool> {
    let options = SqliteConnectOptions::from_str(sqlite_connection_string)?
        .foreign_keys(true)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Run migrations
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    Ok(pool)
}

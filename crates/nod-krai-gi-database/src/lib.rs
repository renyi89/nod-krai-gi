mod config;
pub mod data;
mod error;
pub mod sql_op;
mod util;
pub use error::DbError;

pub use config::DatabaseSettings;

pub use sqlx::Error as SqlError;
use sqlx::{
    migrate::{MigrateDatabase, MigrateError},
    Sqlite,
};

pub struct DbConnection(pub(crate) sqlx::SqlitePool);

pub async fn connect_to(settings: &DatabaseSettings) -> Result<DbConnection, SqlError> {
    let db_url = format!("sqlite://{}", settings.db_file);

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    Ok(DbConnection(pool))
}

pub async fn run_migrations(pool: &DbConnection) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(&pool.0).await
}

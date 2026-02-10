pub mod data;
mod error;
pub mod rocksdb_op;
pub use error::DbError;

use common::database_config::DatabaseSettings;
use rocksdb::{DBCompressionType, Options, WriteBatch, DB};

pub struct DbConnection(pub(crate) DB);

pub fn connect_to(settings: &DatabaseSettings) -> Result<DbConnection, DbError> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_compression_type(DBCompressionType::Zstd);
    opts.set_max_background_jobs(4);
    opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB

    let db = DB::open(&opts, settings.db_file.clone())?;
    Ok(DbConnection(db))
}

pub fn batch_write<F>(conn: &DbConnection, f: F) -> Result<(), DbError>
where
    F: FnOnce(&mut WriteBatch),
{
    let mut batch = WriteBatch::default();
    f(&mut batch);
    conn.0.write(batch)?;
    Ok(())
}

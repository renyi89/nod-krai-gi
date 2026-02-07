#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("RocksDB error: {0}")]
    RocksDbError(#[from] rocksdb::Error),
    #[error("entry not found")]
    NotFound,
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub mod event;
pub mod output;

pub static USER_VERSION: std::sync::OnceLock<
    std::sync::Arc<dashmap::DashMap<u32, String>>,
> = std::sync::OnceLock::new();
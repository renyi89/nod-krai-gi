pub mod event;
pub mod output;

pub static PLAYER_VERSION: std::sync::OnceLock<
    std::sync::Arc<dashmap::DashMap<u32, String>>,
> = std::sync::OnceLock::new();

#[macro_export]
macro_rules! get_player_version {
    ($uid:expr) => {{
        $crate::PLAYER_VERSION
            .get()
            .and_then(|map| map.get($uid))
            .map(|v| v.clone())
            .unwrap_or_else(|| "unknown version".to_string())
    }};
}

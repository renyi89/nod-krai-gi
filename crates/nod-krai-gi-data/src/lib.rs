pub mod ability;
pub mod config;
pub mod dynamic_float;
pub mod excel;
pub mod prop_type;
pub mod quest;
pub mod scene;

pub use dynamic_float::DynamicFloat;

use common::game_server_config::GameServerConfig;
use common::language::Language;
use common::TomlConfig;
use std::sync::LazyLock;

pub static GAME_SERVER_CONFIG: LazyLock<GameServerConfig> = LazyLock::new(|| {
    let mut config = GameServerConfig::load_or_create("game-server.toml");
    config.language = Language::from_locale() as u32;
    config
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let _ = excel::load_all("../../assets/ExcelBinOutput");
    }

    #[test]
    fn test_load_ability() {
        let _ = ability::load_ability_configs_from_bin("../../assets/BinOutput");
    }
}

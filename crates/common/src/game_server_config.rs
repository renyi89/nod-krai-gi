use crate::database_config::DatabaseSettings;
use crate::TomlConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameServerConfig {
    #[serde(skip)]
    pub language: u32,
    pub network: NetworkSettings,
    pub plugin: PluginSettings,
    pub database: DatabaseSettings,
    pub cur_region_name: String,
    pub region_list_path: String,
    pub encryption_config_path: String,
}

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub udp_host: String,
}

#[derive(Deserialize)]
pub struct PluginSettings {
    pub packet_log: bool,
    pub ability: bool,
    pub ability_log: bool,
    pub social: bool,
    pub quest: bool,
}

impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../game-server.default.toml");
}

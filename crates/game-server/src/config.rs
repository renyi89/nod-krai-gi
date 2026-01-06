use common::TomlConfig;
use nod_krai_gi_database::DatabaseSettings;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub network: NetworkSettings,
    pub database: DatabaseSettings,
    pub cur_region_name: String,
    pub region_list_path: String,
    pub encryption_config_path: String,
}

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub udp_host: String,
}
impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../game-server.default.toml");
}

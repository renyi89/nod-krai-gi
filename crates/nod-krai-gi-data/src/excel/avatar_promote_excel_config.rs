use super::common::{IdCountConfig, PropValConfig};
use std::collections::HashMap;
use common::string_util::InternString;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarPromoteExcelConfig {
    pub avatar_promote_id: u32,
    pub promote_level: u32,
    pub promote_audio: InternString,
    pub scoin_cost: u32,
    pub cost_items: Vec<IdCountConfig>,
    pub unlock_max_level: u32,
    pub add_props: Vec<PropValConfig>,
    pub required_player_level: u32,
}

pub trait AvatarPromoteExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarPromoteExcelConfig>;
}

impl AvatarPromoteExcelConfigKeyed<u32> for AvatarPromoteExcelConfig {
    fn key(&self) -> u32 {
        self.avatar_promote_id<<8+self.promote_level
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarPromoteExcelConfig> {
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/AvatarPromoteExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list:Vec<AvatarPromoteExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

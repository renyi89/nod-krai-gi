use super::common::{AddProp, IdCountConfig};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarPromoteExcelConfig {
    pub avatar_promote_id: u32,
    #[serde(default)]
    pub promote_level: u32,
    pub cost_items: Vec<IdCountConfig>,
    pub add_props: Vec<AddProp>,
}

pub trait AvatarPromoteExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarPromoteExcelConfig>;
}

impl AvatarPromoteExcelConfigKeyed<u32> for AvatarPromoteExcelConfig {
    fn key(&self) -> u32 {
        (self.avatar_promote_id << 8) + self.promote_level
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarPromoteExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarPromoteExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarPromoteExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .filter(|item| item.promote_level != 0)
            .map(|item| (item.key(), item.clone()))
            .collect();
        data
    }
}

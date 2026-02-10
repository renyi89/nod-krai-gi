use super::common::{EquipType, ItemType};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliquaryExcelConfig {
    pub id: u32,
    pub item_type: ItemType,
    pub equip_type: EquipType,

    pub main_prop_depot_id: u32,
    pub append_prop_depot_id: u32,
    pub append_prop_num: u32,
    pub set_id: u32,
    pub add_prop_levels: Vec<u32>,

    pub rank: u32,
    pub rank_level: u32,
    pub gadget_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait ReliquaryExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, ReliquaryExcelConfig>;
}

impl ReliquaryExcelConfigKeyed<u32> for ReliquaryExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, ReliquaryExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/ReliquaryExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<ReliquaryExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

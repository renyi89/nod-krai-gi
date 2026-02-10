use std::collections::HashMap;
use common::string_util::InternString;
use super::common::AddProp;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarTalentExcelConfig {
    pub add_props: Vec<AddProp>,
    pub desc_text_map_hash: u64,
    pub main_cost_item_count: u32,
    pub main_cost_item_id: u32,
    pub name_text_map_hash: u64,
    pub open_config: InternString,
    pub param_list: Vec<f32>,
    pub prev_talent: u32,
    pub talent_id: u32,
}

pub trait AvatarTalentExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarTalentExcelConfig>;
}

impl AvatarTalentExcelConfigKeyed<u32> for AvatarTalentExcelConfig {
    fn key(&self) -> u32 {
        self.talent_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarTalentExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarTalentExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarTalentExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

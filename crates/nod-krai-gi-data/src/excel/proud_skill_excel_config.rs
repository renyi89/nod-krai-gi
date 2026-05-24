use super::common::{AddProp, IdCountConfig};
use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProudSkillExcelConfig {
    pub add_props: Vec<AddProp>,
    #[serde(default)]
    pub break_level: u32,
    #[serde(default)]
    pub coin_cost: u32,
    #[serde(default)]
    pub cost_items: Vec<IdCountConfig>,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub open_config: InternString,
    #[serde(default)]
    pub param_list: Vec<f32>,
    #[serde(default)]
    pub proud_skill_group_id: u32,
    #[serde(default)]
    pub proud_skill_id: u32,
    #[serde(default)]
    pub proud_skill_type: u32,
    pub unlock_desc_text_map_hash: u64,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait ProudSkillExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, ProudSkillExcelConfig>;
}

impl ProudSkillExcelConfigKeyed<u32> for ProudSkillExcelConfig {
    fn key(&self) -> u32 {
        self.proud_skill_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, ProudSkillExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/ProudSkillExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<ProudSkillExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

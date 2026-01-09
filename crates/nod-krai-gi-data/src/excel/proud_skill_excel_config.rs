use std::collections::HashMap;
use super::common::AddProp;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostItem {
    pub count: u32,
    pub id: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProudSkillExcelConfig {
    pub add_props: Vec<AddProp>,
    pub break_level: u32,
    pub coin_cost: u32,
    pub cost_items: Vec<CostItem>,
    pub desc_text_map_hash: u64,
    pub filter_conds: Vec<String>,
    pub icon: String,
    pub is_hide_life_proud_skill: bool,
    pub level: u32,
    pub life_effect_params: Vec<String>,
    pub life_effect_type: String,
    pub name_text_map_hash: u64,
    pub open_config: String,
    pub param_desc_list: Vec<u32>,
    pub param_list: Vec<f32>,
    pub proud_skill_group_id: u32,
    pub proud_skill_id: u32,
    pub proud_skill_type: u32,
    pub unlock_desc_text_map_hash: u64,
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
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/ProudSkillExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<ProudSkillExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

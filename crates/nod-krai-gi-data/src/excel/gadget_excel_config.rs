use std::collections::HashMap;
use common::string_util::InternString;
use crate::excel::common::EntityType;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetExcelConfig {
    pub id: u32,
    pub r#type: EntityType,
    pub point_type: Option<u32>,
    pub json_name: InternString,
    pub is_interactive: bool,
    pub tags: Vec<InternString>,
    pub item_json_name: InternString,
    #[serde(alias = "campID")]
    #[serde(default)]
    pub camp_id: u32,
    pub vision_level: InternString,
    pub name_text_map_hash: u64,
}

pub trait GadgetExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, GadgetExcelConfig>;
}

impl GadgetExcelConfigKeyed<u32> for GadgetExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, GadgetExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/GadgetExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<GadgetExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

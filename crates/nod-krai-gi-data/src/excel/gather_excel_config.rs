use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatherExcelConfig {
    pub area_id: u32,
    pub extra_item_id_vec: Vec<u32>,
    pub gadget_id: u32,
    pub init_disable_interact: bool,
    pub is_forbid_guest: bool,
    pub item_id: u32,
    pub point_type: u32,
    pub refresh_id: u32,
}

pub trait GatherExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, GatherExcelConfig>;
}

impl GatherExcelConfigKeyed<u32> for GatherExcelConfig {
    fn key(&self) -> u32 {
        self.point_type
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, GatherExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/GatherExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<GatherExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

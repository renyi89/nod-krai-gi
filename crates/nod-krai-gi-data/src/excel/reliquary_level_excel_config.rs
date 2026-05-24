use crate::excel::common::AddProp;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliquaryLevelExcelConfig {
    pub add_props: Vec<AddProp>,
    #[serde(default)]
    pub rank: u32,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub exp: u32,
}

pub trait ReliquaryLevelExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, ReliquaryLevelExcelConfig>;
}

impl ReliquaryLevelExcelConfigKeyed<u32> for ReliquaryLevelExcelConfig {
    fn key(&self) -> u32 {
        (self.rank << 8) + self.level
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, ReliquaryLevelExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/ReliquaryLevelExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<ReliquaryLevelExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .filter(|item| item.rank != 0)
            .map(|item| (item.key(), item.clone()))
            .collect();
        data
    }
}

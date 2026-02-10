use serde::Deserialize;
use std::collections::HashMap;
use crate::prop_type::FightPropType;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliquaryAffixExcelConfig {
    pub depot_id: u32,
    pub group_id: u32,
    pub id: u32,
    pub prop_type: FightPropType,
    pub prop_value: f64,
}

pub trait ReliquaryAffixExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, ReliquaryAffixExcelConfig>;
}

impl crate::excel::ReliquaryAffixExcelConfigKeyed<u32> for ReliquaryAffixExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, ReliquaryAffixExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/ReliquaryAffixExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<ReliquaryAffixExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

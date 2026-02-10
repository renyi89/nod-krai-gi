use serde::Deserialize;
use std::collections::HashMap;
use common::string_util::InternString;
use crate::prop_type::FightPropType;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliquaryMainPropExcelConfig {
    pub affix_name: InternString,
    pub id: u32,
    pub prop_depot_id: u32,
    pub prop_type: FightPropType,
}

pub trait ReliquaryMainPropExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, ReliquaryMainPropExcelConfig>;
}

impl crate::excel::ReliquaryMainPropExcelConfigKeyed<u32> for ReliquaryMainPropExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, ReliquaryMainPropExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/ReliquaryMainPropExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<ReliquaryMainPropExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

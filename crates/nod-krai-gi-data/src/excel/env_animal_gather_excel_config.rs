use crate::excel::common::IdCountConfig;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvAnimalGatherExcelConfig {
    pub animal_id: u32,
    pub gather_item_list: Vec<IdCountConfig>,
}

pub trait EnvAnimalGatherExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, EnvAnimalGatherExcelConfig>;
}

impl EnvAnimalGatherExcelConfigKeyed<u32> for EnvAnimalGatherExcelConfig {
    fn key(&self) -> u32 {
        self.animal_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, EnvAnimalGatherExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/EnvAnimalGatherExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<EnvAnimalGatherExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

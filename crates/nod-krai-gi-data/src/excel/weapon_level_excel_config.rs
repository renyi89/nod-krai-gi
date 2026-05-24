use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponLevelExcelConfig {
    pub level: u32,
    pub required_exps: Vec<u32>,
}

pub trait WeaponLevelExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, WeaponLevelExcelConfig>;
}

impl WeaponLevelExcelConfigKeyed<u32> for WeaponLevelExcelConfig {
    fn key(&self) -> u32 {
        self.level
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, WeaponLevelExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/WeaponLevelExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<WeaponLevelExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

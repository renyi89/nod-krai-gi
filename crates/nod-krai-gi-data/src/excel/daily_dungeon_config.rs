use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyDungeonConfig {
    pub id: u32,
    pub monday: Vec<u32>,
    pub tuesday: Vec<u32>,
    pub wednesday: Vec<u32>,
    pub thursday: Vec<u32>,
    pub friday: Vec<u32>,
    pub saturday: Vec<u32>,
    pub sunday: Vec<u32>,
}

pub trait DailyDungeonConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, DailyDungeonConfig>;
}

impl DailyDungeonConfigKeyed<u32> for DailyDungeonConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, DailyDungeonConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/DailyDungeonConfigData.json"
        ))
        .unwrap();
        let list: Vec<DailyDungeonConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

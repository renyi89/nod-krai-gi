use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropItemConfig {
    pub item_id: u32,
    pub count_range: InternString,
    pub weight: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropSubTableExcelConfig {
    pub id: u32,
    pub random_type: u32,
    pub drop_level: u32,
    pub drop_vec: Vec<DropItemConfig>,
    pub node_type: u32,
}

pub trait DropTableLike {
    fn id(&self) -> u32;
    fn random_type(&self) -> u32;
    fn drop_vec(&self) -> &Vec<DropItemConfig>;
}

impl DropTableLike for DropSubTableExcelConfig {
    fn id(&self) -> u32 {
        self.id
    }
    fn random_type(&self) -> u32 {
        self.random_type
    }
    fn drop_vec(&self) -> &Vec<DropItemConfig> {
        &self.drop_vec
    }
}

pub trait DropSubTableExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(custom_output_path: &str) -> HashMap<K, DropSubTableExcelConfig>;
}

impl DropSubTableExcelConfigKeyed<u32> for DropSubTableExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(custom_output_path: &str) -> HashMap<u32, DropSubTableExcelConfig> {
        let json = std::fs::read(&format!(
            "{custom_output_path}/DropSubTableExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<DropSubTableExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

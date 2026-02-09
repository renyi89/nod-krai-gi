use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub static BIG_WORLD_MAP_LAYER_FLOOR_CONFIG: std::sync::OnceLock<Arc<Vec<u32>>> =
    std::sync::OnceLock::new();

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLayerFloorConfig {
    pub id: u32,
}

pub trait MapLayerFloorConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MapLayerFloorConfig>;
}

impl MapLayerFloorConfigKeyed<u32> for MapLayerFloorConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MapLayerFloorConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/MapLayerFloorExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MapLayerFloorConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();

        let sort_data = list
            .iter()
            .filter(|item| item.key().to_string().starts_with('3'))
            .map(|item| item.key())
            .collect::<HashSet<u32>>();
        let mut sort_data = sort_data.into_iter().collect::<Vec<u32>>();
        sort_data.sort();
        BIG_WORLD_MAP_LAYER_FLOOR_CONFIG
            .set(Arc::new(sort_data))
            .unwrap();

        data
    }
}

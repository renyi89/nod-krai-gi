use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub static BIG_WORLD_MAP_LAYER_CONFIG: std::sync::OnceLock<Arc<Vec<u32>>> =
    std::sync::OnceLock::new();

/// 安全获取地图层配置集合
/// 如果配置未初始化，返回空 Arc
pub fn get_map_layer_config_collection() -> Arc<Vec<u32>> {
    BIG_WORLD_MAP_LAYER_CONFIG
        .get()
        .cloned()
        .unwrap_or_else(|| Arc::new(Vec::new()))
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLayerConfig {
    pub id: u32,
}

pub trait MapLayerConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MapLayerConfig>;
}

impl MapLayerConfigKeyed<u32> for MapLayerConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MapLayerConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/MapLayerExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MapLayerConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        let sort_data = list
            .iter()
            .filter(|item| item.key().to_string().starts_with('3'))
            .map(|item| item.key())
            .collect::<HashSet<u32>>();
        let mut sort_data = sort_data.into_iter().collect::<Vec<u32>>();
        sort_data.sort();
        BIG_WORLD_MAP_LAYER_CONFIG.set(Arc::new(sort_data)).unwrap();
        data
    }
}

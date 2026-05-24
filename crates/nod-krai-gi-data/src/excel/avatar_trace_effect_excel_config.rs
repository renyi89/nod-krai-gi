use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarTraceEffectExcelConfig {
    #[serde(default)]
    pub trace_effect_id: u32,
    pub avatar_id: u32,
    pub item_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait AvatarTraceEffectExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarTraceEffectExcelConfig>;
}

impl AvatarTraceEffectExcelConfigKeyed<u32> for AvatarTraceEffectExcelConfig {
    fn key(&self) -> u32 {
        self.trace_effect_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarTraceEffectExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarTraceEffectExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarTraceEffectExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .filter(|item| item.trace_effect_id != 0)
            .map(|item| (item.key(), item.clone()))
            .collect();
        data
    }
}

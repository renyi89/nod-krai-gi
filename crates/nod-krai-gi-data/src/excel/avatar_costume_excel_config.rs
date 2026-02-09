use std::collections::HashMap;

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCostumeExcelConfig {
    pub skin_id: u32,
    pub item_id: u32,
    pub character_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait AvatarCostumeExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarCostumeExcelConfig>;
}

impl AvatarCostumeExcelConfigKeyed<u32> for AvatarCostumeExcelConfig {
    fn key(&self) -> u32 {
        self.skin_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarCostumeExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarCostumeExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarCostumeExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

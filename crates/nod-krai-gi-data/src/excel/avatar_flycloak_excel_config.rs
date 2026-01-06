use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarFlycloakExcelConfig {
    pub flycloak_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait AvatarFlycloakExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarFlycloakExcelConfig>;
}

impl AvatarFlycloakExcelConfigKeyed<u32> for AvatarFlycloakExcelConfig {
    fn key(&self) -> u32 {
        self.flycloak_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarFlycloakExcelConfig> {
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/AvatarFlycloakExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<AvatarFlycloakExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

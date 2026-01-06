use super::common::GrowCurveInfo;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCurveExcelConfig {
    pub level: u32,
    pub curve_infos: Vec<GrowCurveInfo>,
}

pub trait AvatarCurveExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarCurveExcelConfig>;
}

impl AvatarCurveExcelConfigKeyed<u32> for AvatarCurveExcelConfig {
    fn key(&self) -> u32 {
        self.level
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarCurveExcelConfig> {
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/AvatarCurveExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<AvatarCurveExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

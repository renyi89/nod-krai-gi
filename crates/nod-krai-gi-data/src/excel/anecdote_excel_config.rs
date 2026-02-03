use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnecdoteExcelConfig {
    pub anecdote_id: u32,
    pub parent_quest_id_list: Vec<u32>,
}

pub trait AnecdoteExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AnecdoteExcelConfig>;
}

impl AnecdoteExcelConfigKeyed<u32> for AnecdoteExcelConfig {
    fn key(&self) -> u32 {
        self.anecdote_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AnecdoteExcelConfig> {
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/AnecdoteExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<AnecdoteExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

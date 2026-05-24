use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestEncryptionKey {
    pub main_quest_id: u32,
    pub encryption_key: u64,
}

pub trait QuestEncryptionKeyKeyed<K> {
    fn key(&self) -> K;

    fn load(custom_output_path: &str) -> HashMap<K, QuestEncryptionKey>;
}

impl QuestEncryptionKeyKeyed<u32> for QuestEncryptionKey {
    fn key(&self) -> u32 {
        self.main_quest_id
    }

    fn load(custom_output_path: &str) -> HashMap<u32, QuestEncryptionKey> {
        let json =
            std::fs::read(&format!("{custom_output_path}/QuestEncryptionKeys.json")).unwrap();
        let list: Vec<QuestEncryptionKey> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

use crate::excel;
use std::collections::HashMap;
use std::sync::Arc;

static SCENE_TAG_ENTRIES: std::sync::OnceLock<Arc<HashMap<u32, Vec<SceneTagConfig>>>> =
    std::sync::OnceLock::new();

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneTagConfig {
    pub id: u32,
    pub scene_id: u32,
}

pub trait SceneTagConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, SceneTagConfig>;

    fn get_scene_tag_entries() -> &'static Arc<HashMap<u32, Vec<SceneTagConfig>>>;
}

impl SceneTagConfigKeyed<u32> for SceneTagConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, SceneTagConfig> {
        let json =
            std::fs::read(&format!("{excel_bin_output_path}/SceneTagConfigData.json"))
                .unwrap();
        let list: Vec<SceneTagConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }

    fn get_scene_tag_entries() -> &'static Arc<HashMap<u32, Vec<SceneTagConfig>>> {
        if SCENE_TAG_ENTRIES
            .get()
            .is_some()
        {
            SCENE_TAG_ENTRIES
                .get()
                .unwrap()
        } else {
            let scene_tag_config_collection_clone =
                std::sync::Arc::clone(excel::scene_tag_config_collection::get());

            let mut scene_tag_entries: HashMap<u32, Vec<SceneTagConfig>> = HashMap::new();

            for x in scene_tag_config_collection_clone.values() {
                if scene_tag_entries.contains_key(&x.scene_id) {
                    let mut list = scene_tag_entries.get(&x.scene_id).unwrap().clone();
                    list.push(x.clone());
                    scene_tag_entries.insert(x.scene_id, list);
                } else {
                    scene_tag_entries.insert(x.scene_id, vec![x.clone()]);
                }
            }

            let _ =
                SCENE_TAG_ENTRIES.set(Arc::new(scene_tag_entries));
            SCENE_TAG_ENTRIES
                .get()
                .unwrap()
        }
    }
}

use crate::scene::Position;
use common::string_util::InternString;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use rayon::prelude::*;

pub static SCENE_POINT_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, ScenePointConfig>>> =
    std::sync::OnceLock::new();

pub static SCENE_POINT_ENTRY_MAP_COLLECTION: std::sync::OnceLock<
    Arc<HashMap<u32, ScenePointData>>,
> = std::sync::OnceLock::new();

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ScenePointData {
    #[serde(alias = "$type")]
    pub r#point_type: InternString,
    pub gadget_id: u32,
    pub area_id: u32,
    pub tran_scene_id: u32,
    pub dungeon_ids: Vec<u32>,
    pub dungeon_random_list: Vec<u32>,
    pub pos: Position,
    pub tran_pos: Position,
    pub tran_rot: Position,
    pub size: Position,
    pub group_limit: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenePointConfig {
    #[serde(default)]
    pub points: HashMap<u32, ScenePointData>,
}

pub fn load_scene_point_configs_from_bin(bin_output_path: &str) {
    let entries: Vec<_> = fs::read_dir(format!("{bin_output_path}/Scene/Point/"))
        .unwrap()
        .collect();

    let processed_entries: Vec<_> = entries
        .into_par_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let file_name = entry
                    .file_name()
                    .to_string_lossy()
                    .replace("scene", "")
                    .replace("_point.json", "");

                match u32::from_str(file_name.as_str()) {
                    Ok(scene_id) => {
                        let json = std::fs::read(entry.path()).unwrap();
                        let result: serde_json::Result<ScenePointConfig> = 
                            serde_json::from_slice(&*json);
                        match result {
                            Ok(config) => {
                                let mut local_data = HashMap::new();
                                local_data.insert(scene_id, config.clone());

                                let mut local_entry_map = HashMap::new();
                                for (point_id, point_data) in config.points {
                                    if point_data.tran_scene_id != 0 {
                                        local_entry_map.insert(
                                            (scene_id << 16) + point_id,
                                            point_data.clone(),
                                        );
                                    }
                                }

                                Some((local_data, local_entry_map))
                            }
                            Err(error) => {
                                println!("error :{} scene_id:{}", error, scene_id);
                                None
                            }
                        }
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        })
        .collect();

    let (data, entry_map): (HashMap<u32, ScenePointConfig>, HashMap<u32, ScenePointData>) = 
        processed_entries.into_iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut acc_data, mut acc_entry_map), (local_data, local_entry_map)| {
                acc_data.extend(local_data);
                acc_entry_map.extend(local_entry_map);
                (acc_data, acc_entry_map)
            }
        );

    let _ = SCENE_POINT_CONFIG_COLLECTION.set(Arc::new(data));
    let _ = SCENE_POINT_ENTRY_MAP_COLLECTION.set(Arc::new(entry_map));
}

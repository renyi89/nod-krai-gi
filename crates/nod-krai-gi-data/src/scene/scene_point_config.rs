use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use common::string_util::InternString;

pub static SCENE_POINT_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, ScenePointConfig>>> =
    std::sync::OnceLock::new();

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Position {
    #[serde(alias = "_x", alias = "X")]
    pub x: f32,
    #[serde(alias = "_y", alias = "Y")]
    pub y: f32,
    #[serde(alias = "_z", alias = "Z")]
    pub z: f32,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ScenePointData {
    #[serde(alias = "$type")]
    pub r#point_type: InternString,
    pub gadget_id: u32,
    pub area_id: u32,
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
    let mut data: HashMap<u32, ScenePointConfig> = HashMap::new();

    for entry in fs::read_dir(format!("{bin_output_path}/Scene/Point/")).unwrap() {
        match entry {
            Ok(entry) => {
                let file_name = entry
                    .file_name()
                    .to_string_lossy()
                    .replace("scene", "")
                    .replace("_point.json", "");

                match u32::from_str(file_name.as_str()) {
                    Ok(scene_id) => {
                        let file = fs::File::open(entry.path()).unwrap();
                        let content = std::io::BufReader::new(file);
                        let result: serde_json::Result<ScenePointConfig> =
                            serde_json::from_reader(content);
                        match result {
                            Ok(config) => {
                                data.insert(scene_id, config.clone());
                            }
                            Err(error) => {
                                println!("error :{} scene_id:{}", error, scene_id);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            _ => {}
        }
    }

    let _ = SCENE_POINT_CONFIG_COLLECTION.set(Arc::new(data));
}

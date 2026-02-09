use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};
use common::string_util::InternString;

#[derive(Debug, serde::Deserialize)]
pub struct GadgetConfig {
    pub combat: Option<GadgetCombat>,
    #[serde(default)]
    pub abilities: Vec<GadgetAbility>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GadgetCombat {
    pub property: Option<serde_json::Value>,
    pub be_hit: Option<serde_json::Value>,
    pub combat_lock: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetAbility {
    pub ability_name: InternString,
    #[serde(default)]
    pub ability_override: InternString,
}

impl GadgetAbility {
    pub const TYPE_IDENTIFIER: u32 = 7;
    pub const DEFAULT_OVERRIDE: &str = "Default";
}

pub fn load_gadget_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    load_gadget_configs(fs::read_dir(format!("{bin_output_path}/Gadget/"))?)?;

    Ok(())
}

static GADGET_CONFIG_MAP: OnceLock<HashMap<InternString, GadgetConfig>> = OnceLock::new();

fn load_gadget_configs(gadget_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in gadget_config_dir {
        let entry = entry?;
        let json =  std::fs::read(entry.path())?;
        match serde_json::from_slice(&*json) {
            Ok(config) => {
                let configs: HashMap<InternString, GadgetConfig> = config;
                // 将所有解析出的配置添加到全局 map 中
                for (key, config) in configs {
                    map.insert(key, config);
                }
            }
            Err(e) => {
                println!("failed to parse gadget config: {:?} {:?}", e, entry.path());
            }
        }
    }

    let _ = GADGET_CONFIG_MAP.set(map);
    Ok(())
}

pub fn get_gadget_config(name: &InternString) -> Option<&GadgetConfig> {
    GADGET_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_gadget_config_map() -> std::collections::hash_map::Iter<'static, InternString, GadgetConfig> {
    GADGET_CONFIG_MAP.get().unwrap().iter()
}

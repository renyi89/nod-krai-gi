use common::string_util::InternString;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};

#[derive(Debug, serde::Deserialize)]
pub struct AvatarConfig {
    #[serde(default)]
    pub abilities: Vec<AvatarAbility>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAbility {
    pub ability_name: InternString,
    #[serde(default)]
    pub ability_override: InternString,
}

impl AvatarAbility {
    pub const TYPE_IDENTIFIER: u32 = 7;
    pub const DEFAULT_OVERRIDE: &str = "Default";
}

pub fn load_avatar_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    load_avatar_configs(fs::read_dir(format!("{bin_output_path}/Avatar/"))?)?;

    Ok(())
}

static AVATAR_CONFIG_MAP: OnceLock<HashMap<InternString, AvatarConfig>> = OnceLock::new();

fn load_avatar_configs(avatar_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in avatar_config_dir {
        let entry = entry?;
        let avatar_name = entry
            .file_name()
            .to_string_lossy()
            .replace("ConfigAvatar_", "")
            .replace(".json", "");

        let json =  std::fs::read(entry.path())?;
        let config: AvatarConfig = serde_json::from_slice(&*json)?;
        map.insert(avatar_name.into(), config);
    }

    let _ = AVATAR_CONFIG_MAP.set(map);
    Ok(())
}

pub fn get_avatar_config(name: &InternString) -> Option<&AvatarConfig> {
    AVATAR_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_avatar_config_map(
) -> std::collections::hash_map::Iter<'static, InternString, AvatarConfig> {
    AVATAR_CONFIG_MAP.get().unwrap().iter()
}

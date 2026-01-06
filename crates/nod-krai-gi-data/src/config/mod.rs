mod avatar;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};

pub use avatar::*;

pub fn load_avatar_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    load_avatar_configs(fs::read_dir(format!("{bin_output_path}/Avatar/"))?)?;

    Ok(())
}

static AVATAR_CONFIG_MAP: OnceLock<HashMap<String, AvatarConfig>> = OnceLock::new();

fn load_avatar_configs(avatar_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in avatar_config_dir {
        let entry = entry?;
        let avatar_name = entry
            .file_name()
            .to_string_lossy()
            .replace("ConfigAvatar_", "")
            .replace(".json", "");

        let data = fs::File::open(entry.path())?;
        let reader = std::io::BufReader::new(data);
        let config: AvatarConfig = serde_json::from_reader(reader)?;
        map.insert(avatar_name, config);
    }

    let _ = AVATAR_CONFIG_MAP.set(map);
    Ok(())
}

pub fn get_avatar_config(name: &str) -> Option<&AvatarConfig> {
    AVATAR_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_avatar_config_map() -> std::collections::hash_map::Iter<'static, String, AvatarConfig> {
    AVATAR_CONFIG_MAP.get().unwrap().iter()
}

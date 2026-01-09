use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};

pub use super::talent_types::{TalentConfig};

static AVATAR_TALENT_CONFIG_MAP: OnceLock<HashMap<String, TalentConfig>> = OnceLock::new();

fn load_avatar_talent_configs(talent_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in talent_config_dir {
        let entry = entry?;
        let talent_name = entry
            .file_name()
            .to_string_lossy()
            .replace("ConfigTalent_", "")
            .replace(".json", "");

        let data = fs::File::open(entry.path())?;
        let reader = std::io::BufReader::new(data);
        let config: TalentConfig = serde_json::from_reader(reader)?;
        map.insert(talent_name, config);
    }

    let _ = AVATAR_TALENT_CONFIG_MAP.set(map);
    Ok(())
}

pub fn load_avatar_talent_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    load_avatar_talent_configs(fs::read_dir(format!(
        "{bin_output_path}/Talent/AvatarTalents/"
    ))?)?;

    Ok(())
}

pub fn get_avatar_talent_config(name: &str) -> Option<&TalentConfig> {
    AVATAR_TALENT_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_avatar_talent_config_map() -> std::collections::hash_map::Iter<'static, String, TalentConfig> {
    AVATAR_TALENT_CONFIG_MAP.get().unwrap().iter()
}

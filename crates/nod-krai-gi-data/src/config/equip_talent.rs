use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};
use std::collections::hash_map::Iter;
use common::string_util::InternString;
use crate::config::TalentAction;
pub use super::talent_types::{TalentConfig};

static EQUIP_TALENT_CONFIG_MAP: OnceLock<HashMap<InternString, Vec<TalentAction>>> = OnceLock::new();

fn load_equip_talent_configs(talent_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in talent_config_dir {
        let entry = entry?;
        let json =  std::fs::read(entry.path())?;
        let config: TalentConfig = serde_json::from_slice(&*json)?;
        map.extend(config.talents);
    }

    let _ = EQUIP_TALENT_CONFIG_MAP.set(map);
    Ok(())
}

pub fn load_equip_talent_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    load_equip_talent_configs(fs::read_dir(format!(
        "{bin_output_path}/Talent/EquipTalents/"
    ))?)?;

    Ok(())
}

pub fn get_equip_talent_config(name: &InternString) -> Option<&Vec<TalentAction>> {
    EQUIP_TALENT_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_equip_talent_config_map() -> Iter<'static, InternString, Vec<TalentAction>> {
    EQUIP_TALENT_CONFIG_MAP.get().unwrap().iter()
}

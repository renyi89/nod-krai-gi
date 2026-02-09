pub use super::talent_types::TalentConfig;
use crate::config::TalentAction;
use crate::excel::{AvatarTalentExcelConfig, ProudSkillExcelConfig};
use common::string_util::InternString;
use std::collections::hash_map::Iter;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    sync::OnceLock,
};

static AVATAR_TALENT_CONFIG_MAP: OnceLock<HashMap<InternString, Vec<TalentAction>>> =
    OnceLock::new();

fn load_avatar_talent_configs(talent_config_dir: ReadDir) -> std::io::Result<()> {
    let mut map = HashMap::new();
    for entry in talent_config_dir {
        let entry = entry?;
        let json = std::fs::read(entry.path())?;
        let config: TalentConfig = serde_json::from_slice(&*json)?;
        map.extend(config.talents);
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

pub fn get_avatar_talent_config(name: &InternString) -> Option<&Vec<TalentAction>> {
    AVATAR_TALENT_CONFIG_MAP.get().unwrap().get(name)
}

pub fn iter_avatar_talent_config_map() -> Iter<'static, InternString, Vec<TalentAction>> {
    AVATAR_TALENT_CONFIG_MAP.get().unwrap().iter()
}
pub fn process_talent_ids(
    talent_id_list: &[u32],
    avatar_talent_collection: &std::sync::Arc<HashMap<u32, AvatarTalentExcelConfig>>,
) -> Vec<InternString> {
    let mut open_configs = Vec::new();
    for talent_id in talent_id_list {
        if let Some(talent_config) = avatar_talent_collection.get(talent_id) {
            open_configs.push(talent_config.open_config);
        }
    }
    open_configs
}

pub fn process_inherent_proud_skills(
    inherent_proud_skill_list: &[u32],
    proud_skill_collection: &std::sync::Arc<HashMap<u32, ProudSkillExcelConfig>>,
) -> Vec<InternString> {
    let mut open_configs = Vec::new();
    for proud_skill_id in inherent_proud_skill_list {
        if let Some(proud_skill_config) = proud_skill_collection.get(proud_skill_id) {
            open_configs.push(proud_skill_config.open_config);
        }
    }
    open_configs
}

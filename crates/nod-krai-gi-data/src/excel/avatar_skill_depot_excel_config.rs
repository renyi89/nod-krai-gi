use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProudSkillOpenConfig {
    #[serde(default)]
    pub proud_skill_group_id: u32,
    #[serde(default)]
    pub need_avatar_promote_level: u32,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvatarSkillDepotExcelConfig {
    pub id: u32,
    #[serde(default)]
    pub energy_skill: u32,
    #[serde(default)]
    pub skills: Vec<u32>,
    #[serde(default)]
    pub sub_skills: Vec<u32>,
    #[serde(default)]
    pub extra_abilities: Vec<InternString>,
    #[serde(default)]
    pub talents: Vec<u32>,
    #[serde(default)]
    pub talent_star_name: InternString,
    #[serde(default)]
    pub inherent_proud_skill_opens: Vec<ProudSkillOpenConfig>,
    #[serde(default)]
    pub skill_depot_ability_group: InternString,
}

pub trait AvatarSkillDepotExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarSkillDepotExcelConfig>;
}

impl AvatarSkillDepotExcelConfigKeyed<u32> for AvatarSkillDepotExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarSkillDepotExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarSkillDepotExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarSkillDepotExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

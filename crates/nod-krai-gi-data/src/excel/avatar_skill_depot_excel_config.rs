use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProudSkillOpenConfig {
    pub proud_skill_group_id: u32,
    pub need_avatar_promote_level: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarSkillDepotExcelConfig {
    pub id: u32,
    pub energy_skill: u32,
    pub skills: Vec<u32>,
    pub sub_skills: Vec<u32>,
    pub attack_mode_skill: u32,
    pub leader_talent: u32,
    pub extra_abilities: Vec<String>,
    pub talents: Vec<u32>,
    pub talent_star_name: String,
    pub inherent_proud_skill_opens: Vec<ProudSkillOpenConfig>,
    pub skill_depot_ability_group: String,
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
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/AvatarSkillDepotExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list:Vec<AvatarSkillDepotExcelConfig> =
            serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

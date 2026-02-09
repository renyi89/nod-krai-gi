use std::collections::HashMap;
use common::string_util::InternString;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarSkillExcelConfig {
    pub ability_name: InternString,
    pub cd_time: f32,
    pub cost_elem_type: InternString,
    pub cost_elem_val: u32,
    pub cost_stamina: u32,
    pub desc_text_map_hash: u64,
    pub energy_min: u32,
    pub id: u32,
    pub is_attack_camera_lock: bool,
    pub max_charge_num: u32,
    pub name_text_map_hash: u64,
    pub proud_skill_group_id: u32,
    #[serde(default)]
    pub special_energy_min: f32,
    #[serde(default)]
    pub special_energy_max: f32,
}

pub trait AvatarSkillExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarSkillExcelConfig>;
}

impl AvatarSkillExcelConfigKeyed<u32> for AvatarSkillExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarSkillExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarSkillExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarSkillExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

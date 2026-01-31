use super::common::PropGrowCurve;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HpDropConfig {
    pub drop_id: u32,
    pub hp_percent: f32,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum MonsterType {
    #[serde(alias = "MONSTER_NONE")]
    #[default]
    None,
    #[serde(alias = "MONSTER_ORDINARY")]
    Ordinary,
    #[serde(alias = "MONSTER_BOSS")]
    Boss,
    #[serde(alias = "MONSTER_ENV_ANIMAL")]
    EnvAnimal,
    #[serde(alias = "MONSTER_LITTLE_MONSTER")]
    LittleMonster,
    #[serde(alias = "MONSTER_FISH")]
    Fish,
    #[serde(alias = "MONSTER_PARTNER")]
    PARTNER,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum VisionLevelType {
    #[serde(alias = "VISION_LEVEL_NORMAL")]
    #[default]
    Normal,
    #[serde(alias = "VISION_LEVEL_LITTLE_REMOTE")]
    LittleRemote,
    #[serde(alias = "VISION_LEVEL_REMOTE")]
    Remote,
    #[serde(alias = "VISION_LEVEL_SUPER")]
    Super,
    #[serde(alias = "VISION_LEVEL_NEARBY")]
    Nearby,
    #[serde(alias = "VISION_LEVEL_SUPER_NEARBY")]
    SuperNearby,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterExcelConfig {
    pub id: u32,
    pub r#type: MonsterType,
    pub server_script: String,
    pub combat_config_hash: u64,
    pub affix: Vec<u32>,
    pub ai: String,
    pub equips: Vec<u32>,
    pub hp_drops: Vec<HpDropConfig>,
    pub kill_drop_id: u32,
    pub vision_level: VisionLevelType,
    pub is_invisible_reset: bool,
    pub exclude_weathers: String,
    #[serde(default)]
    pub feature_tag_group_id: u32,
    pub skin: String,
    pub hp_base: f32,
    pub attack_base: f32,
    pub defense_base: f32,
    pub critical: f32,
    pub critical_hurt: f32,
    pub prop_grow_curves: Vec<PropGrowCurve>,
    pub element_mastery: f32,
    #[serde(alias = "campID")]
    #[serde(default)]
    pub camp_id: u32,
    pub name_text_map_hash: u64,
}

pub trait MonsterExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MonsterExcelConfig>;
}

impl MonsterExcelConfigKeyed<u32> for MonsterExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MonsterExcelConfig> {
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/MonsterExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<MonsterExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

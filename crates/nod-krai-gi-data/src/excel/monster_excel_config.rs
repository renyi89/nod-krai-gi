use super::common::{PropGrowCurve, VisionLevelType};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HpDropConfig {
    #[serde(default)]
    pub drop_id: u32,
    #[serde(default)]
    pub hp_percent: f32,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MonsterDescribeExcelConfig {
    pub id: u32,
    #[serde(alias = "titleID")]
    pub title_id: u32,
    pub special_name_id: Option<u32>,
    #[serde(alias = "specialNameLabID")]
    pub special_name_lab_id: u32,
    #[serde(alias = "nameTextMapHash")]
    pub name_text_map_hash: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MonsterSpecialNameExcelConfig {
    #[serde(alias = "specialNameLabID")]
    pub special_name_lab_id: u32,
    #[serde(alias = "specialNameID")]
    pub special_name_id: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterExcelConfig {
    pub id: u32,
    pub r#type: MonsterType,
    #[serde(default)]
    pub describe_id: u32,
    #[serde(default)]
    pub describe: Option<MonsterDescribeExcelConfig>,
    #[serde(default)]
    pub affix: Vec<u32>,
    #[serde(default)]
    pub equips: Vec<u32>,
    #[serde(default)]
    pub kill_drop_id: u32,
    #[serde(default)]
    pub hp_drops: Vec<HpDropConfig>,
    #[serde(default)]
    pub vision_level: VisionLevelType,
    #[serde(default)]
    pub feature_tag_group_id: u32,
    #[serde(default)]
    pub hp_base: f32,
    #[serde(default)]
    pub attack_base: f32,
    #[serde(default)]
    pub defense_base: f32,
    #[serde(default)]
    pub critical: f32,
    #[serde(default)]
    pub critical_hurt: f32,
    #[serde(default)]
    pub element_mastery: f32,
    #[serde(alias = "campID")]
    #[serde(default)]
    pub camp_id: u32,
    pub prop_grow_curves: Vec<PropGrowCurve>,
}

pub trait MonsterDescribeExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MonsterDescribeExcelConfig>;
}

impl MonsterDescribeExcelConfigKeyed<u32> for MonsterDescribeExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MonsterDescribeExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/MonsterDescribeExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MonsterDescribeExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.into_iter().map(|item| (item.key(), item)).collect();
        data
    }
}

pub trait MonsterSpecialNameExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MonsterSpecialNameExcelConfig>;
}

impl MonsterSpecialNameExcelConfigKeyed<u32> for MonsterSpecialNameExcelConfig {
    fn key(&self) -> u32 {
        self.special_name_lab_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MonsterSpecialNameExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/MonsterSpecialNameExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MonsterSpecialNameExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.into_iter().map(|item| (item.key(), item)).collect();
        data
    }
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
        let special_name_configs = MonsterSpecialNameExcelConfig::load(excel_bin_output_path);
        let describe_configs = MonsterDescribeExcelConfig::load(excel_bin_output_path);

        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/MonsterExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MonsterExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .into_iter()
            .map(|mut item| {
                if let Some(describe) = describe_configs.get(&item.describe_id) {
                    let mut describe = describe.clone();
                    describe.special_name_id = special_name_configs
                        .get(&describe.special_name_lab_id)
                        .and_then(|s| Some(s.special_name_id));
                    item.describe = Some(describe);
                }
                (item.key(), item)
            })
            .collect();
        data
    }
}

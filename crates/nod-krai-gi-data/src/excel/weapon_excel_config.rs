use super::common::{ItemType, MaterialType, WeaponType};
use crate::prop_type::FightPropType;
use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProperty {
    #[serde(default)]
    pub prop_type: FightPropType,
    #[serde(default)]
    pub init_value: f32,
    #[serde(default)]
    pub r#type: InternString,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponExcelConfig {
    pub id: u32,
    pub item_type: ItemType,
    pub weapon_type: WeaponType,
    #[serde(default)]
    pub material_type: MaterialType,

    pub weapon_base_exp: u32,
    pub skill_affix: Vec<u32>,
    pub weapon_prop: Vec<WeaponProperty>,
    pub weapon_promote_id: u32,
    #[serde(default)]
    pub awaken_material: u32,
    #[serde(default)]
    pub awaken_costs: Vec<u32>,
    #[serde(default)]
    pub destroy_return_material: Vec<u32>,
    #[serde(default)]
    pub destroy_return_material_count: Vec<u32>,

    pub rank: u32,
    pub rank_level: u32,
    pub gadget_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait WeaponExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, WeaponExcelConfig>;
}

impl WeaponExcelConfigKeyed<u32> for WeaponExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, WeaponExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/WeaponExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<WeaponExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}

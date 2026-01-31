use super::common::{ItemType, MaterialType, WeaponType};
use crate::prop_type::FightPropType;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProperty {
    pub prop_type: FightPropType,
    pub init_value: f32,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponExcelConfig {
    pub weapon_type: WeaponType,
    pub rank_level: u32,
    pub material_type: MaterialType,
    pub weapon_base_exp: u32,
    pub skill_affix: Vec<u32>,
    pub awaken_material: u32,
    pub weapon_prop: Vec<WeaponProperty>,
    pub awaken_texture: String,
    pub awaken_light_map_texture: String,
    pub awaken_icon: String,
    pub un_rotate: bool,
    pub weapon_promote_id: u32,
    pub story_id: u32,
    pub awaken_costs: Vec<u32>,
    pub gacha_card_name_hash: u64,
    pub destroy_return_material: Vec<u32>,
    pub destroy_return_material_count: Vec<u32>,
    pub initial_lock_state: u32,
    pub id: u32,
    pub icon: String,
    pub item_type: ItemType,
    pub weight: u32,
    pub rank: u32,
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
        let file = std::fs::File::open(&format!(
            "{excel_bin_output_path}/WeaponExcelConfigData.json"
        ))
        .unwrap();
        let content = std::io::BufReader::new(file);
        let list: Vec<WeaponExcelConfig> = serde_json::from_reader(content).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}

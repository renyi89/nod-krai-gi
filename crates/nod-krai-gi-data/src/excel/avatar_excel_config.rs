use super::common::{PropGrowCurve, WeaponType};
use std::collections::HashMap;
use common::string_util::InternString;

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum AvatarUseType {
    #[default]
    #[serde(alias = "AVATAR_TEST")]
    Test,
    #[serde(alias = "AVATAR_SYNC_TEST")]
    SyncTest,
    #[serde(alias = "AVATAR_FORMAL")]
    Formal,
    #[serde(alias = "AVATAR_ABANDON")]
    Abandon,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum AvatarBodyType {
    #[default]
    None,
    #[serde(alias = "BODY_BOY")]
    Boy,
    #[serde(alias = "BODY_GIRL")]
    Girl,
    #[serde(alias = "BODY_LADY")]
    Lady,
    #[serde(alias = "BODY_MALE")]
    Male,
    #[serde(alias = "BODY_LOLI")]
    Loli,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum QualityType {
    #[default]
    #[serde(alias = "QUALITY_NONE")]
    None,
    #[serde(alias = "QUALITY_WHITE")]
    White,
    #[serde(alias = "QUALITY_GREEN")]
    Green,
    #[serde(alias = "QUALITY_BLUE")]
    Blue,
    #[serde(alias = "QUALITY_PURPLE")]
    Purple,
    #[serde(alias = "QUALITY_ORANGE")]
    Orange,
    #[serde(alias = "QUALITY_ORANGE_SP")]
    OrangeSp,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum AvatarIdentityType {
    #[default]
    #[serde(alias = "AVATAR_IDENTITY_MASTER")]
    Master,
    #[serde(alias = "AVATAR_IDENTITY_NORMAL")]
    Normal,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarExcelConfig {
    pub id: u32,
    pub use_type: AvatarUseType,
    pub body_type: AvatarBodyType,
    pub icon_name: InternString,
    pub quality_type: QualityType,
    pub charge_efficiency: f32,
    pub combat_config_hash: u64,
    pub is_range_attack: bool,
    pub initial_weapon: u32,
    pub weapon_type: WeaponType,
    pub image_name: InternString,
    pub gacha_card_name_hash: u64,
    pub gacha_image_name_hash: u64,
    pub coop_pic_name_hash: u64,
    pub skill_depot_id: u32,
    pub stamina_recover_speed: f32,
    pub cand_skill_depot_ids: Vec<u32>,
    pub avatar_identity_type: AvatarIdentityType,
    pub avatar_promote_id: u32,
    pub avatar_promote_reward_level_list: Vec<u32>,
    pub avatar_promote_reward_id_list: Vec<u32>,
    #[serde(default)]
    pub feature_tag_group_id: u32,
    pub hp_base: f32,
    pub attack_base: f32,
    pub defense_base: f32,
    pub critical: f32,
    pub critical_hurt: f32,
    pub prop_grow_curves: Vec<PropGrowCurve>,
    pub element_mastery: f32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait AvatarExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, AvatarExcelConfig>;
}

impl AvatarExcelConfigKeyed<u32> for AvatarExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, AvatarExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/AvatarExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<AvatarExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .filter(|(key, _value)| {
                if *key < 10000002
                    || *key >= 11000000
                    || (*key <= 10000910 && *key >= 10000900)
                    || *key == 10000075
                {
                    return false;
                } else {
                    return true;
                }
            })
            .collect();
        data
    }
}

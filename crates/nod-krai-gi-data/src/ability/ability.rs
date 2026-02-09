use crate::{
    dynamic_float::{any_to_float, any_to_float_hashmap},
    DynamicFloat,
};
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
pub struct AbilityConfigWrapper {
    #[serde(rename = "Default")]
    pub default: AbilityData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityData {
    #[serde(rename = "$type")]
    pub type_name: Option<InternString>,
    pub ability_name: InternString,
    #[serde(default, deserialize_with = "deserialize_modifiers")]
    pub modifiers: IndexMap<InternString, AbilityModifier>,
    #[serde(default)]
    pub ability_mixins: Vec<AbilityMixinData>,
    #[serde(default, deserialize_with = "any_to_float_hashmap")]
    pub ability_specials: HashMap<InternString, f32>,
    #[serde(default)]
    pub is_dynamic_ability: bool,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_added: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_removed: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_ability_start: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_kill: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_field_enter: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_field_exit: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_attach: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_detach: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_avatar_in: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_avatar_out: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_trigger_avatar_ray: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_vehicle_in: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_vehicle_out: Vec<AbilityModifierAction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifier {
    #[serde(skip)]
    pub modifier_name: InternString,
    #[serde(default)]
    pub bonus_critical: Option<f32>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_added: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_removed: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_being_hit: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_attack_landed: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_hitting_other: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_think_interval: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_kill: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_crash: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_avatar_in: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_avatar_out: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_reconnect: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_change_authority: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_vehicle_in: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_vehicle_out: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_zone_enter: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_zone_exit: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_heal: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub on_being_healed: Vec<AbilityModifierAction>,
    #[serde(default)]
    pub modifier_mixins: Vec<AbilityMixinData>,
    #[serde(default)]
    pub properties: Option<AbilityModifierProperty>,
    #[serde(default)]
    pub stacking: Option<InternString>,
    #[serde(default)]
    pub duration: Option<DynamicFloat>,
    #[serde(default)]
    pub think_interval: Option<DynamicFloat>,
    #[serde(default)]
    pub element_durability: Option<DynamicFloat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifierAction {
    #[serde(rename = "$type")]
    pub type_name: Option<InternString>,
    #[serde(default)]
    pub target: Option<AbilityTargettingEnum>,
    pub amount: Option<DynamicFloat>,
    #[serde(rename = "amountByCasterAttackRatio")]
    pub amount_by_caster_attack_ratio: Option<DynamicFloat>,
    #[serde(rename = "amountByCasterCurrentHPRatio")]
    pub amount_by_caster_current_hp_ratio: Option<DynamicFloat>,
    #[serde(rename = "amountByCasterMaxHPRatio")]
    pub amount_by_caster_max_hp_ratio: Option<DynamicFloat>,
    #[serde(rename = "amountByGetDamage")]
    pub amount_by_get_damage: Option<DynamicFloat>,
    #[serde(rename = "amountByTargetCurrentHPRatio")]
    pub amount_by_target_current_hp_ratio: Option<DynamicFloat>,
    #[serde(rename = "amountByTargetMaxHPRatio")]
    pub amount_by_target_max_hp_ratio: Option<DynamicFloat>,
    #[serde(rename = "limboByTargetMaxHPRatio")]
    pub limbo_by_target_max_hp_ratio: Option<DynamicFloat>,
    pub heal_ratio: Option<DynamicFloat>,
    pub speed: Option<DynamicFloat>,
    #[serde(default)]
    pub ignore_ability_property: Option<bool>,
    #[serde(default)]
    pub modifier_name: InternString,
    #[serde(default)]
    #[serde(rename = "enableLockHP")]
    pub enable_lock_hp: Option<bool>,
    #[serde(default)]
    pub lethal: Option<bool>,
    #[serde(default)]
    pub by_server: Option<bool>,
    #[serde(default)]
    pub gadget_id: Option<u32>,
    #[serde(default)]
    pub state_id: Option<u32>,
    #[serde(default)]
    pub skill_id: Option<u32>,
    #[serde(default)]
    pub config_id: Option<u32>,
    #[serde(default)]
    pub camp_id: Option<u32>,
    #[serde(default)]
    pub monster_id: Option<u32>,
    #[serde(default)]
    pub summon_tag: Option<u32>,
    #[serde(default)]
    pub min_value: Option<DynamicFloat>,
    #[serde(default)]
    pub max_value: Option<DynamicFloat>,
    #[serde(default)]
    pub target_value: Option<DynamicFloat>,
    #[serde(default)]
    pub cost_stamina_ratio: Option<DynamicFloat>,
    #[serde(default)]
    pub use_limit_range: Option<bool>,
    #[serde(default)]
    pub base_energy: Option<DynamicFloat>,
    #[serde(default)]
    pub ratio: Option<DynamicFloat>,
    #[serde(default, deserialize_with = "any_to_float")]
    pub value_range_min: f32,
    #[serde(default, deserialize_with = "any_to_float")]
    pub value_range_max: f32,
    #[serde(default)]
    pub determine_type: Option<InternString>,
    #[serde(default)]
    pub override_map_key: Option<InternString>,
    #[serde(default)]
    pub param_num: Option<u32>,
    #[serde(default)]
    pub param1: Option<DynamicFloat>,
    #[serde(default)]
    pub param2: Option<DynamicFloat>,
    #[serde(default)]
    pub param3: Option<DynamicFloat>,
    #[serde(default)]
    pub key: Option<InternString>,
    #[serde(default)]
    pub ability_name: Option<InternString>,
    #[serde(default)]
    pub global_value_key: Option<InternString>,
    #[serde(default)]
    pub ability_formula: Option<InternString>,
    #[serde(default)]
    pub src_target: Option<InternString>,
    #[serde(default)]
    pub dst_target: Option<InternString>,
    #[serde(default)]
    pub src_key: Option<InternString>,
    #[serde(default)]
    pub dst_key: Option<InternString>,
    #[serde(default)]
    pub heal_tag: Option<InternString>,
    #[serde(default)]
    pub camp_target_type: Option<InternString>,
    #[serde(default)]
    pub func_name: Option<InternString>,
    #[serde(default)]
    pub lua_call_type: Option<InternString>,
    #[serde(default)]
    pub content: Option<InternString>,
    #[serde(default)]
    pub parameter: Option<InternString>,
    #[serde(default)]
    pub value: Option<DynamicFloat>,
    #[serde(default)]
    pub type_field: Option<InternString>,
    #[serde(default)]
    #[serde(rename = "healLimitedByCasterMaxHPRatio")]
    pub heal_limited_by_caster_max_hp_ratio: Option<DynamicFloat>,
    #[serde(default)]
    pub effect_templete_id: Option<f32>,
    #[serde(default)]
    pub actions: Vec<AbilityModifierAction>,
    #[serde(default)]
    pub success_actions: Vec<AbilityModifierAction>,
    #[serde(default)]
    pub fail_actions: Vec<AbilityModifierAction>,
    #[serde(default)]
    pub other_targets: Option<Box<AbilityModifierAction>>,
    #[serde(default)]
    pub call_param_list: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityMixinData {
    #[serde(rename = "$type")]
    pub type_name: Option<InternString>,
    #[serde(default)]
    pub modifier_name: Option<serde_json::Value>,
    #[serde(default)]
    pub state_ids: Vec<InternString>,
    #[serde(default)]
    pub global_value_key: Option<InternString>,
    #[serde(default)]
    pub speed: Option<DynamicFloat>,
    #[serde(default)]
    pub cost_stamina_delta: Option<DynamicFloat>,
    #[serde(default)]
    pub ratio: Option<DynamicFloat>,
    #[serde(default)]
    pub default_global_value_on_create: Option<DynamicFloat>,
    #[serde(default)]
    pub modifier_name_steps: Vec<InternString>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifierProperty {
    #[serde(rename = "Actor_HpThresholdRatio")]
    #[serde(default)]
    pub actor_hp_threshold_ratio: Option<DynamicFloat>,
    #[serde(rename = "Actor_MaxHPRatio")]
    #[serde(default)]
    pub actor_max_hp_ratio: Option<DynamicFloat>,
    #[serde(rename = "Actor_AttackSRatio")]
    #[serde(default)]
    pub actor_attack_s_ratio: Option<DynamicFloat>,
    #[serde(rename = "Actor_HealedAddDelta")]
    #[serde(default)]
    pub actor_healed_add_delta: Option<DynamicFloat>,
}

#[derive(Default, Clone, serde::Deserialize, Debug)]
pub enum AbilityTargettingEnum {
    #[default]
    #[serde(alias = "Self")]
    Self_ = 0,
    #[serde(alias = "Caster")]
    Caster = 1,
    #[serde(alias = "Target")]
    Target = 2,
    #[serde(alias = "SelfAttackTarget")]
    SelfAttackTarget = 3,
    #[serde(alias = "Other")]
    Other = 4,
    #[serde(alias = "Applier")]
    Applier = 5,
    #[serde(alias = "Owner")]
    Owner = 6,
    #[serde(alias = "CurTeamAvatars")]
    CurTeamAvatars = 7,
    #[serde(alias = "CurLocalAvatar")]
    CurLocalAvatar = 8,
    #[serde(alias = "OriginOwner")]
    OriginOwner = 9,
    #[serde(alias = "Team")]
    Team = 10,
    #[serde(alias = "TargetOwner")]
    TargetOwner = 11,
    #[serde(alias = "TargetOriginOwner")]
    TargetOriginOwner = 12,
    #[serde(alias = "AllPlayerAvatars")]
    AllPlayerAvatars = 13,
    #[serde(alias = "AllTeams")]
    AllTeams = 14,
    #[serde(alias = "RemoteTeams")]
    RemoteTeams = 15,
    #[serde(alias = "TargetTeam")]
    TargetTeam = 16,
    #[serde(alias = "CasterOwner")]
    CasterOwner = 17,
    #[serde(alias = "CasterOriginOwner")]
    CasterOriginOwner = 18,
    #[serde(alias = "MPLevel")]
    MPLevel = 19,
}

use common::string_util::InternString;
use serde::Deserializer;
use serde_json::Value;

fn deserialize_modifiers<'de, D>(
    deserializer: D,
) -> Result<IndexMap<InternString, AbilityModifier>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut map = IndexMap::<InternString, AbilityModifier>::deserialize(deserializer)?;

    for (k, v) in map.iter_mut() {
        v.modifier_name = k.clone();
    }

    Ok(map)
}

pub fn skip_strings_in_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => {
            let mut result = vec![];
            for item in arr {
                if !item.is_string() {
                    if let Ok(t) = T::deserialize(item) {
                        result.push(t);
                    }
                } else {
                    println!("skip string: {:?}", item);
                }
            }
            Ok(result)
        }
        Value::Null => Ok(vec![]),
        _ => Ok(vec![]),
    }
}
static ABILITY_DATA_MAP: std::sync::OnceLock<HashMap<InternString, AbilityData>> =
    std::sync::OnceLock::new();

static ABILITY_HASH_MAP: std::sync::OnceLock<HashMap<u32, InternString>> =
    std::sync::OnceLock::new();

fn load_ability_configs_recursive(
    dir: std::fs::ReadDir,
    map: &Mutex<HashMap<InternString, AbilityData>>,
) -> std::io::Result<()> {
    let entries: Vec<_> = dir.filter_map(Result::ok).collect();

    let mut subdirs = Vec::new();
    let mut json_files = Vec::new();

    for entry in &entries {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if path.extension().map_or(false, |ext| ext == "json") {
            json_files.push(path);
        }
    }

    for subdir in subdirs {
        load_ability_configs_recursive(std::fs::read_dir(&subdir)?, map)?;
    }

    json_files.par_iter().for_each(|path| {
        if let Ok(json) = std::fs::read(path) {
            let path_str = path.to_string_lossy().to_string();
            if let Ok(wrappers) = serde_json::from_slice::<Vec<AbilityConfigWrapper>>(&*json) {
                let mut map_guard = map.lock().unwrap();
                for wrapper in wrappers {
                    let ability_name = wrapper.default.ability_name.clone();
                    let ability_data = wrapper.default;
                    map_guard.insert(ability_name, ability_data);
                }
            } else {
                eprintln!("Failed to parse {}", path_str);
            }
        }
    });

    Ok(())
}

pub fn load_ability_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    let map = Mutex::new(HashMap::new());
    let ability_dir = std::fs::read_dir(format!("{bin_output_path}/Ability/Temp/"))?;
    load_ability_configs_recursive(ability_dir, &map)?;

    let mut hash_map = HashMap::new();
    {
        let map_guard = map.lock().unwrap();
        // println!("Ability Name -> Hash Mapping:");
        // println!("================================");
        for (name, _) in map_guard.iter() {
            let hash = common::string_util::get_string_hash(name.as_str());
            // println!("{} -> {}", name, hash);
            hash_map.insert(hash, name.clone());
        }

        // println!("================================");
        // println!("Total abilities loaded: {}", map_guard.len());
    }

    let map_inner = map.into_inner().unwrap();
    let _ = ABILITY_DATA_MAP.set(map_inner);
    let _ = ABILITY_HASH_MAP.set(hash_map);
    Ok(())
}

pub fn get_ability_data(name: &InternString) -> Option<&'static AbilityData> {
    ABILITY_DATA_MAP.get().and_then(|map| map.get(name))
}

pub fn get_ability_name_by_hash(hash: u32) -> Option<InternString> {
    ABILITY_HASH_MAP
        .get()
        .and_then(|map| map.get(&hash).cloned())
}

pub fn iter_ability_data_map(
) -> std::collections::hash_map::Iter<'static, InternString, AbilityData> {
    ABILITY_DATA_MAP.get().unwrap().iter()
}

use crate::dynamic_float::{dynamic_float, dynamic_float_hashmap, dynamic_float_vec, skip_strings_in_vec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityConfigWrapper {
    #[serde(rename = "Default")]
    pub default: AbilityData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityData {
    #[serde(rename = "$type")]
    pub type_name: Option<String>,
    pub ability_name: String,
    #[serde(default)]
    pub modifiers: HashMap<String, AbilityModifier>,
    #[serde(default)]
    pub ability_mixins: Vec<AbilityMixinData>,
    #[serde(default, deserialize_with = "dynamic_float_hashmap")]
    pub ability_specials: HashMap<String, Option<f32>>,
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
    pub on_exit: Vec<AbilityModifierAction>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifier {
    #[serde(default)]
    pub modifier_name: String,
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
    pub stacking: Option<String>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub duration: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub think_interval: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub element_durability: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifierAction {
    #[serde(rename = "$type")]
    pub type_name: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_caster_attack_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_caster_current_hp_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_caster_max_hp_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_get_damage: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_target_current_hp_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub amount_by_target_max_hp_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub limbo_by_target_max_hp_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub heal_ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub speed: Option<f32>,
    #[serde(default)]
    pub ignore_ability_property: Option<bool>,
    #[serde(default)]
    pub modifier_name: Option<String>,
    #[serde(default)]
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
    #[serde(default, deserialize_with = "dynamic_float")]
    pub min_value: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub max_value: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub target_value: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub cost_stamina_ratio: Option<f32>,
    #[serde(default)]
    pub use_limit_range: Option<bool>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub base_energy: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub value_range_min: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub value_range_max: Option<f32>,
    #[serde(default)]
    pub determine_type: Option<String>,
    #[serde(default)]
    pub override_map_key: Option<String>,
    #[serde(default)]
    pub param_num: Option<u32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub param1: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub param2: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub param3: Option<f32>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub ability_name: Option<String>,
    #[serde(default)]
    pub global_value_key: Option<String>,
    #[serde(default)]
    pub ability_formula: Option<String>,
    #[serde(default)]
    pub src_target: Option<String>,
    #[serde(default)]
    pub dst_target: Option<String>,
    #[serde(default)]
    pub src_key: Option<String>,
    #[serde(default)]
    pub dst_key: Option<String>,
    #[serde(default)]
    pub heal_tag: Option<String>,
    #[serde(default)]
    pub camp_target_type: Option<String>,
    #[serde(default)]
    pub func_name: Option<String>,
    #[serde(default)]
    pub lua_call_type: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub parameter: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub type_field: Option<String>,
    #[serde(default)]
    pub heal_limited_by_caster_max_hp_ratio: Option<String>,
    #[serde(default)]
    pub effect_templete_id: Option<f32>,
    #[serde(default)]
    pub born: Option<serde_json::Value>,
    #[serde(default)]
    pub predicates: Option<Vec<serde_json::Value>>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub actions: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub success_actions: Vec<AbilityModifierAction>,
    #[serde(default, deserialize_with = "skip_strings_in_vec")]
    pub fail_actions: Vec<AbilityModifierAction>,
    #[serde(default)]
    pub other_targets: Option<Box<AbilityModifierAction>>,
    #[serde(default)]
    pub call_param_list: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityMixinData {
    #[serde(rename = "$type")]
    pub type_name: Option<String>,
    #[serde(default)]
    pub modifier_name: Option<serde_json::Value>,
    #[serde(default)]
    pub state_ids: Vec<String>,
    #[serde(default)]
    pub global_value_key: Option<String>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub speed: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub cost_stamina_delta: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub ratio: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float")]
    pub default_global_value_on_create: Option<f32>,
    #[serde(default, deserialize_with = "dynamic_float_vec")]
    pub ratio_steps: Vec<Option<f32>>,
    #[serde(default)]
    pub modifier_name_steps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifierProperty {
    #[serde(rename = "Actor_HpThresholdRatio")]
    #[serde(default, deserialize_with = "dynamic_float")]
    pub actor_hp_threshold_ratio: Option<f32>,
    #[serde(rename = "Actor_MaxHPRatio")]
    #[serde(default, deserialize_with = "dynamic_float")]
    pub actor_max_hp_ratio: Option<f32>,
    #[serde(rename = "Actor_AttackSRatio")]
    #[serde(default, deserialize_with = "dynamic_float")]
    pub actor_attack_s_ratio: Option<f32>,
    #[serde(rename = "Actor_HealedAddDelta")]
    #[serde(default, deserialize_with = "dynamic_float")]
    pub actor_healed_add_delta: Option<f32>,
}

static ABILITY_DATA_MAP: std::sync::OnceLock<HashMap<String, AbilityData>> =
    std::sync::OnceLock::new();

static ABILITY_HASH_MAP: std::sync::OnceLock<HashMap<u32, String>> = std::sync::OnceLock::new();

fn load_ability_configs_recursive(
    dir: std::fs::ReadDir,
    map: &mut HashMap<String, AbilityData>,
) -> std::io::Result<()> {
    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            load_ability_configs_recursive(std::fs::read_dir(&path)?, map)?;
        } else if path.extension().map_or(false, |ext| ext == "json") {
            let data = std::fs::File::open(&path)?;
            let reader = std::io::BufReader::new(data);
            let path_str = path.to_string_lossy().to_string();

            let wrappers: Vec<AbilityConfigWrapper> = match serde_json::from_reader(reader) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", path_str, e);
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
                }
            };

            for wrapper in wrappers {
                let ability_name = wrapper.default.ability_name.clone();
                let ability_data = wrapper.default;
                map.insert(ability_name, ability_data);
            }
        }
    }

    Ok(())
}

pub fn load_ability_configs_from_bin(bin_output_path: &str) -> std::io::Result<()> {
    let mut map = HashMap::new();
    let ability_dir = std::fs::read_dir(format!("{bin_output_path}/Ability/Temp/"))?;
    load_ability_configs_recursive(ability_dir, &mut map)?;

    println!("Ability Name -> Hash Mapping:");
    println!("================================");
    let mut hash_map = HashMap::new();
    for (name, _) in &map {
        let hash = common::string_util::get_string_hash(name);
        println!("{} -> {}", name, hash);
        hash_map.insert(hash, name.clone());
    }

    println!("================================");
    println!("Total abilities loaded: {}", map.len());

    let _ = ABILITY_DATA_MAP.set(map);
    let _ = ABILITY_HASH_MAP.set(hash_map);
    Ok(())
}

pub fn get_ability_data(name: &str) -> Option<&AbilityData> {
    ABILITY_DATA_MAP.get().and_then(|map| map.get(name))
}

pub fn iter_ability_data_map() -> std::collections::hash_map::Iter<'static, String, AbilityData> {
    ABILITY_DATA_MAP.get().unwrap().iter()
}

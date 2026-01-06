use nod_krai_gi_data::excel::{GadgetExcelConfig, MonsterExcelConfig};
use nod_krai_gi_entity::{common::FightProperties, fight_props};

pub fn create_fight_properties_by_monster_config(config: &MonsterExcelConfig) -> FightProperties {
    fight_props! {
        FIGHT_PROP_BASE_HP: config.hp_base,
        FIGHT_PROP_HP: config.hp_base,
        FIGHT_PROP_BASE_ATTACK: config.attack_base,
        FIGHT_PROP_ATTACK: config.attack_base,
        FIGHT_PROP_BASE_DEFENSE: config.defense_base,
        FIGHT_PROP_DEFENSE: config.defense_base,
        FIGHT_PROP_CUR_HP: config.hp_base,
        FIGHT_PROP_MAX_HP: config.hp_base,
        FIGHT_PROP_CUR_ATTACK: config.attack_base,
        FIGHT_PROP_CUR_DEFENSE: config.defense_base,
        FIGHT_PROP_ELEMENT_MASTERY: config.element_mastery,
        FIGHT_PROP_CRITICAL: config.critical,
        FIGHT_PROP_CRITICAL_HURT: config.critical_hurt
    }
}

pub fn create_fight_properties_by_gadget_config(_config: &GadgetExcelConfig) -> FightProperties {
    fight_props! {
        FIGHT_PROP_BASE_HP: 50000f32,
        FIGHT_PROP_BASE_ATTACK: 50000f32,
        FIGHT_PROP_BASE_DEFENSE: 50000f32
    }
}

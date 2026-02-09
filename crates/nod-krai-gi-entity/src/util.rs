use crate::common::FightProperties;
use crate::common::ProtocolEntityID;
use nod_krai_gi_data::excel::{GadgetExcelConfig, MonsterExcelConfig};

pub const fn to_protocol_entity_id(
    ty: nod_krai_gi_proto::normal::ProtEntityType,
    index: u32,
) -> ProtocolEntityID {
    ProtocolEntityID(((ty as u32) << 22) | index)
}

#[macro_export]
macro_rules! fight_props {
    ($($prop_ty:ident: $val:expr),*) => {
        FightProperties(::std::collections::HashMap::from([$(
            (::nod_krai_gi_data::prop_type::FightPropType::$prop_ty, $val),
        )*]), ::std::collections::HashSet::new())
    };
}

#[macro_export]
macro_rules! int_prop_value {
    ($prop_name:ident, $value:expr) => {
        ::nod_krai_gi_proto::PropValue {
            r#type: ::nod_krai_gi_data::prop_type::$prop_name,
            val: $value as i64,
            value: Some(::nod_krai_gi_proto::prop_value::Value::Ival($value as i64)),
        }
    };
}

#[macro_export]
macro_rules! int_prop_pair {
    ($prop_name:ident, $value:expr) => {
        ::nod_krai_gi_proto::normal::PropPair {
            r#type: ::nod_krai_gi_data::prop_type::$prop_name,
            prop_value: Some(::nod_krai_gi_proto::normal::PropValue {
                r#type: ::nod_krai_gi_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::nod_krai_gi_proto::normal::prop_value::Value::Ival($value as i64)),
            }),
        }
    };
}

#[macro_export]
macro_rules! int_prop_map {
    ($($prop_name:ident: $value:expr;)*) => {
        ::std::collections::HashMap::from([$((
            ::nod_krai_gi_data::prop_type::$prop_name,
            ::nod_krai_gi_proto::normal::PropValue {
                r#type: ::nod_krai_gi_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::nod_krai_gi_proto::normal::prop_value::Value::Ival($value as i64)),
            },
        ),)*])
    };
}

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

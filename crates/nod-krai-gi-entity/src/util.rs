use crate::common::ProtocolEntityID;

pub const fn to_protocol_entity_id(
    ty: nod_krai_gi_proto::ProtEntityType,
    index: u32,
) -> ProtocolEntityID {
    ProtocolEntityID(((ty as u32) << 22) | index)
}

#[macro_export]
macro_rules! fight_props {
    ($($prop_ty:ident: $val:expr),*) => {
        FightProperties(::std::collections::HashMap::from([$(
            (::nod_krai_gi_data::prop_type::FightPropType::$prop_ty, $val),
        )*]))
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
        ::nod_krai_gi_proto::PropPair {
            r#type: ::nod_krai_gi_data::prop_type::$prop_name,
            prop_value: Some(::nod_krai_gi_proto::PropValue {
                r#type: ::nod_krai_gi_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::nod_krai_gi_proto::prop_value::Value::Ival($value as i64)),
            }),
        }
    };
}

#[macro_export]
macro_rules! int_prop_map {
    ($($prop_name:ident: $value:expr;)*) => {
        ::std::collections::HashMap::from([$((
            ::nod_krai_gi_data::prop_type::$prop_name,
            ::nod_krai_gi_proto::PropValue {
                r#type: ::nod_krai_gi_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::nod_krai_gi_proto::prop_value::Value::Ival($value as i64)),
            },
        ),)*])
    };
}

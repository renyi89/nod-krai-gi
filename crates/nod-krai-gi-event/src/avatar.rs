use bevy_ecs::message::Message;
use nod_krai_gi_data::excel::common::EquipType;

#[derive(Message)]
pub struct AvatarEquipChangeEvent {
    pub player_uid: u32,
    pub avatar_guid: u64,
    pub equip_type: EquipType,
}

pub enum AvatarAppearanceChange {
    Costume(u32),
    TraceEffect(u32),
}

#[derive(Message)]
pub struct AvatarAppearanceChangeEvent {
    pub player_uid: u32,
    pub avatar_guid: u64,
    pub change: AvatarAppearanceChange,
}

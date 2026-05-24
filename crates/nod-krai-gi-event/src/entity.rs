use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_proto::normal::{ChangeEnergyReason, ChangeHpDebtsReason, ChangHpReason, PropChangeReason, VisionType};

#[derive(Message)]
pub struct EntityPropertyUpdateEvent(pub Entity, pub FightPropType, pub f32);

#[derive(Message)]
pub struct EntityPropertySeparateUpdateEvent(pub Entity, pub FightPropType, pub f32);

#[derive(Message)]
pub struct EntityDisappearEvent(pub u32, pub VisionType);

pub enum ChangeReason {
    ChangeHpReason(ChangHpReason),
    ChangeHpDebtsReason(ChangeHpDebtsReason),
    ChangeEnergyReason(ChangeEnergyReason),
}
#[derive(Message)]
pub struct EntityFightPropChangeReasonNotifyEvent {
    pub entity_id: u32,
    pub prop_type: FightPropType,
    pub value: f32,
    pub param_list: Option<Vec<u32>>,
    pub reason: PropChangeReason,
    pub change_reason: ChangeReason,
}

#[derive(Message)]
pub struct GadgetInteractEvent(pub u32, pub u32, pub u32);

#[derive(Message)]
pub struct GadgetStateChangeEvent {
    pub entity: Entity,
    pub state_id: u32,
    pub previous_state_id: Option<u32>,
}

#[derive(Message)]
pub struct SetWorktopOptionsEvent {
    pub player_uid: u32,
    pub group_id: u32,
    pub config_id: u32,
    pub gadget_entity_id: u32,
    pub option_list: Vec<u32>,
    pub del_option: u32,
}

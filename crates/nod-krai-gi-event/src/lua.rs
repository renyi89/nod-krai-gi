use bevy_ecs::message::Message;
use nod_krai_gi_data::scene::{EventType, LuaEvt};

#[derive(Message)]
pub struct LuaTriggerEvent {
    pub group_id: u32,
    pub event_type: EventType,
    pub evt: LuaEvt,
}

#[derive(Message)]
pub struct SpawnGroupEntityEvent {
    pub scene_id: u32,
    pub block_id: u32,
    pub group_id: u32,
}

#[derive(Message)]
pub struct DespawnGroupEntityEvent {
    pub group_id: u32,
}

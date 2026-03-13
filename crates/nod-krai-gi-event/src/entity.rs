use bevy_ecs::message::Message;
use bevy_ecs::entity::Entity;

#[derive(Message)]
pub struct GadgetInteractEvent(pub u32, pub u32, pub u32);

#[derive(Message)]
pub struct EntityStateChangeEvent {
    pub entity: Entity,
    pub state_id: u32,
    pub previous_state_id: Option<u32>,
}

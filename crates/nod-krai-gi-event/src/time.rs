use bevy_ecs::message::Message;

#[derive(Message)]
pub struct UpdateClientTimeEvent(pub u32, pub u32);
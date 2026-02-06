use bevy_ecs::message::Message;

#[derive(Message)]
pub struct QuestBeginEvent(pub u32, pub u32);

#[derive(Message)]
pub struct QuestListUpdateEvent(pub u32, pub u32);
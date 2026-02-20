use bevy_ecs::message::Message;
use nod_krai_gi_proto::normal::{AttackResult, EntityMoveInfo};

#[derive(Message)]
pub struct EntityMoveEvent(pub u32, pub EntityMoveInfo);
#[derive(Message)]
pub struct EntityBeingHitEvent(pub u32, pub AttackResult);
#[derive(Message)]
pub struct PlayerMoveEvent(pub u32, pub u32, pub (f32, f32, f32),pub bool);

use bevy_ecs::message::Message;
use nod_krai_gi_proto::{AttackResult, EntityMoveInfo};
#[derive(Message)]
pub struct EntityMoveEvent(pub u32, pub EntityMoveInfo);
#[derive(Message)]
pub struct EntityBeingHitEvent(pub u32, pub AttackResult);

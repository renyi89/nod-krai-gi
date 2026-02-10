use bevy_ecs::message::Message;
use std::collections::HashMap;

#[derive(Message)]
pub struct StoreItemChangeEvent(pub u32, pub HashMap<u64,u32>);
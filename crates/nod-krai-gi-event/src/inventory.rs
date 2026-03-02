use bevy_ecs::message::Message;
use std::collections::HashMap;

#[derive(Message)]
pub struct StoreItemChangeEvent(pub u32, pub HashMap<u64, u32>);

#[derive(Message)]
pub struct ItemAddEvent(
    pub u32,
    pub  Vec<(
        u32,
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<u32>,
        HashMap<u32, u32>,
    )>,
);

#[derive(Message)]
pub struct ItemDropEvent(pub u32, pub Option<(f32, f32, f32)>, pub Vec<u32>);

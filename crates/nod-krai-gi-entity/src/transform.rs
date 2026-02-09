use std::fmt;

use bevy_ecs::prelude::*;
use nod_krai_gi_proto::server_only::VectorBin;

#[derive(Component, Clone)]
pub struct Transform {
    pub position: VectorBin,
    pub rotation: VectorBin,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[pos:{}|rot:{}]", self.position, self.rotation)
    }
}

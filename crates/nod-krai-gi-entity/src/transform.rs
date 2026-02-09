use std::fmt;

use bevy_ecs::prelude::*;
use nod_krai_gi_proto::server_only::Vector;

#[derive(Component, Clone)]
pub struct Transform {
    pub position: Vector,
    pub rotation: Vector,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[pos:{}|rot:{}]", self.position, self.rotation)
    }
}

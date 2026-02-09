use std::collections::{hash_map::Keys, HashMap};

use bevy_ecs::prelude::Resource;
use nod_krai_gi_proto::server_only::PlayerDataBin;

#[derive(Resource)]
pub struct Players(HashMap<u32, PlayerDataBin>);

impl Players {
    pub fn keys(&self) -> Keys<'_, u32, PlayerDataBin> {
        self.0.keys()
    }

    pub fn get(&self, uid: u32) -> Option<&PlayerDataBin>  {
        self.0.get(&uid)
    }

    pub fn get_mut(&mut self, uid: u32) -> Option<&mut PlayerDataBin> {
        self.0.get_mut(&uid)
    }
}

impl From<HashMap<u32, PlayerDataBin>> for Players {
    fn from(value: HashMap<u32, PlayerDataBin>) -> Self {
        Self(value)
    }
}

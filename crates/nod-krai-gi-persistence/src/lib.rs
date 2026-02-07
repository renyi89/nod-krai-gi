use std::collections::{hash_map::Keys, HashMap};

use bevy_ecs::prelude::Resource;
use player_information::PlayerInformation;

pub mod player_information;

#[derive(Resource)]
pub struct Players(HashMap<u32, PlayerInformation>);

impl Players {
    pub fn keys(&self) -> Keys<'_, u32, PlayerInformation> {
        self.0.keys()
    }

    pub fn get(&self, uid: u32) -> Option<&PlayerInformation>  {
        self.0.get(&uid)
    }

    pub fn get_mut(&mut self, uid: u32) -> Option<&mut PlayerInformation> {
        self.0.get_mut(&uid)
    }
}

impl From<HashMap<u32, PlayerInformation>> for Players {
    fn from(value: HashMap<u32, PlayerInformation>) -> Self {
        Self(value)
    }
}

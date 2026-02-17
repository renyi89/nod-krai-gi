use crate::scene::Position;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BlockGroup {
    pub id: u32,
    pub refresh_id: Option<u32>,
    pub pos: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneBlockTemplate {
    pub groups: Vec<BlockGroup>,
}

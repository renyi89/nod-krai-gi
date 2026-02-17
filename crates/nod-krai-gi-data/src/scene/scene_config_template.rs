use crate::scene::Position;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SceneConfig {
    pub begin_pos: Position,
    pub size: Position,
    pub born_pos: Position,
    pub born_rot: Position,
    pub die_y: Option<f32>,
    pub city_id: Option<u32>,
    pub vision_anchor: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockRect {
    pub min: Position,
    pub max: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneConfigTemplate {
    pub scene_config: SceneConfig,
    pub blocks: Vec<u32>,
    pub block_rects: Vec<BlockRect>,
}

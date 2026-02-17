use crate::excel::common::VisionLevelType;
use crate::scene::{EventType, GadgetState, Position, RegionShape};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BaseInfo {
    pub scene_id: u32,
    pub block_id: u32,
    pub group_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Monster {
    pub config_id: u32,
    pub monster_id: u32,
    pub pos: Position,
    pub rot: Position,
    pub level: Option<u32>,
    pub title_id: Option<u32>,
    pub special_name_id: Option<u32>,
    pub drop_tag: Option<String>,
    pub disable_wander: Option<bool>,
    pub pose_id: Option<u32>,
    pub area_id: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gadget {
    pub config_id: u32,
    pub gadget_id: u32,
    pub pos: Position,
    pub rot: Position,
    pub level: Option<u32>,
    pub vision_level: Option<VisionLevelType>,
    pub state: Option<GadgetState>,
    pub persistent: Option<bool>,
    pub area_id: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    pub config_id: u32,
    pub shape: RegionShape,
    pub radius: Option<f32>,
    pub pos: Position,
    pub area_id: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Trigger {
    pub config_id: u32,
    pub name: String,
    pub event: EventType,
    pub source: String,
    pub condition: String,
    pub action: String,
    pub trigger_count: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Variable {
    pub config_id: u32,
    pub name: String,
    pub value: i32,
    pub no_refresh: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Suite {
    pub monsters: Vec<u32>,
    pub gadgets: Vec<u32>,
    pub regions: Vec<u32>,
    pub triggers: Vec<String>,
    pub rand_weight: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InitConfig {
    pub suite: u32,
    pub end_suite: u32,
    pub rand_suite: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SceneGroupTemplate {
    pub monsters: Vec<Monster>,
    pub gadgets: Vec<Gadget>,
    pub regions: Vec<Region>,
    pub triggers: Vec<Trigger>,
    pub variables: Vec<Variable>,

    pub init_config: InitConfig,
    pub suites: Vec<Suite>,

    pub base_info: BaseInfo,
}

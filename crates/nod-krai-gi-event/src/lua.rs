use bevy_ecs::message::Message;
use nod_krai_gi_data::scene::{EventType, LuaContext, LuaEvt, ScriptCommand};

#[derive(Message)]
pub struct LuaTriggerEvent {
    pub group_id: u32,
    pub event_type: EventType,
    pub evt: LuaEvt,
}

#[derive(Message)]
pub struct ScriptCommandEvent {
    pub command: ScriptCommand,
}

#[derive(Message)]
pub struct SpawnGroupEntityEvent {
    pub scene_id: u32,
    pub block_id: u32,
    pub group_id: u32,
    pub refresh_suite_id: u32,
}

#[derive(Message)]
pub struct DespawnGroupEntityEvent {
    pub group_id: u32,
}

#[derive(Message)]
pub struct SpawnSuiteEntitiesEvent {
    pub group_id: u32,
    pub block_id: u32,
    pub suite_id: u32,
}

#[derive(Message)]
pub struct DespawnSuiteEntitiesEvent {
    pub group_id: u32,
    pub suite_id: u32,
}

#[derive(Message)]
pub struct RefreshGroupEntityEvent {
    pub group_id: u32,
    pub suite_id: u32,
    pub block_id: u32,
}

#[derive(Message)]
pub struct OnClientExecuteReqEvent {
    pub lua_name: String,
    pub param1: u32,
    pub param2: u32,
    pub param3: u32,
    pub lua_context: LuaContext,
}

#[derive(Message)]
pub struct OnBeHurtEvent {
    pub lua_name: String,
    pub element_type: u32,
    pub strike_type: u32,
    pub is_host: bool,
    pub lua_context: LuaContext,
}

#[derive(Message)]
pub struct ChallengeStartEvent {
    pub group_id: u32,
    pub challenge_id: u32,
    pub challenge_index: u32,
    pub param_list: Vec<u32>,
}

#[derive(Message)]
pub struct ChallengeFinishEvent {
    pub group_id: u32,
    pub challenge_id: u32,
    pub challenge_index: u32,
    pub is_success: bool,
    pub time_cost: u32,
}

#[derive(Message)]
pub struct ChallengeProgressEvent {
    pub group_id: u32,
    pub challenge_index: u32,
    pub challenge_type: nod_krai_gi_data::excel::ChallengeType,
    pub param_index: u32,
    pub value: u32,
}

#[derive(Message)]
pub struct MonsterKillEvent {
    pub group_id: u32,
    pub config_id: u32,
    pub monster_id: u32,
}

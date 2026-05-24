use bevy_ecs::prelude::*;
use crossbeam_queue::SegQueue;
use mlua::{Function, Table};
pub(crate) use nod_krai_gi_data::scene::{LuaContext, LuaEvt, ScriptCommand};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[allow(unused)]
pub trait ScriptLib: Send + Sync + 'static {
    fn add_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32);
    fn remove_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32);

    // group variable methods
    fn get_group_variable_value(&self, group_id: u32, name: &str) -> i32;
    fn set_group_variable_value(&self, group_id: u32, name: &str, value: i32) -> i32;
    fn change_group_variable_value(&self, group_id: u32, name: &str, delta: i32) -> i32;

    // refresh group
    fn refresh_group(&self, group_id: u32, suite_id: u32);
    fn del_worktop_option_by_group_id(&self, group_id: u32, config_id: u32, option: u32);
    fn set_worktop_options_by_group_id(&self, group_id: u32, config_id: u32, option_list: Vec<u32>);

    // gadget state methods
    fn get_gadget_state_by_config_id(&self, uid: u32, group_id: u32, config_id: u32) -> i32;
    fn set_gadget_state_by_config_id(&self, uid: u32, group_id: u32, config_id: u32, state: u32);

    fn get_group_monster_count_by_config_id(&self, uid: u32, group_id: u32) -> i32;

    // challenge methods
    fn active_challenge(
        &self,
        group_id: u32,
        source: u32,
        challenge_id: u32,
        challenge_index: u32,
        param1: u32,
        param2: u32,
        param3: u32,
        param4: u32,
    );
    fn stop_challenge(&self, group_id: u32, challenge_index: u32, is_success: bool);
    fn add_challenge_progress(&self, group_id: u32, challenge_index: u32, progress: u32);

    // quest progress
    fn add_quest_progress(&self, group_id: u32, quest_param: &str);

    // entity creation/removal
    fn create_gadget(&self, group_id: u32, config_id: u32);
    fn create_monster(&self, group_id: u32, config_id: u32);
    fn kill_entity_by_config_id(&self, group_id: u32, config_id: u32);

    // skill control
    fn set_is_allow_use_skill(&self, allow: bool);

    // notifications
    fn show_reminder(&self, reminder_id: u32);
    fn play_cut_scene(&self, cutscene_id: u32);
    fn show_client_guide(&self, guide_name: &str);
    fn show_common_tips(&self, title: &str, content: &str, close_time: u32);
    fn close_common_tips(&self);

    // dungeon
    fn cause_dungeon_result(&self, is_success: bool);

    // movement
    fn move_player_to_pos(&self, uid: u32, pos: (f32, f32, f32), rot: (f32, f32, f32), scene_id: u32);

    // audio/weather
    fn scene_play_sound(&self, sound_name: &str, pos: (f32, f32, f32), play_type: u32);
    fn set_weather_area_state(&self, area_id: u32, climate_type: u32);

    // platform
    fn set_platform_route_id(&self, group_id: u32, config_id: u32, route_id: u32);
    fn start_platform(&self, group_id: u32, config_id: u32);
    fn stop_platform(&self, group_id: u32, config_id: u32);

    // lua notification
    fn notify_group_lua(&self, group_id: u32, event_type: u32, param1: u32, param2: u32, param3: u32);

    // server global values
    fn set_entity_server_global_value(&self, group_id: u32, config_id: u32, sgv_name: &str, value: u32);

    // force lock/unlock
    fn unlock_force(&self, force_id: u32);
    fn lock_force(&self, force_id: u32);

    // group entity kill with policy
    fn kill_group_entity(&self, group_id: u32, kill_policy: u32);

    // suite switching
    fn go_to_group_suite(&self, group_id: u32, suite_id: u32);
    fn get_group_suite(&self, group_id: u32) -> u32;

    // cross-group variable access
    fn set_group_variable_value_by_group(&self, group_id: u32, name: &str, value: i32) -> i32;
    fn change_group_variable_value_by_group(&self, group_id: u32, name: &str, delta: i32) -> i32;

    // timer
    fn create_group_timer_event(&self, group_id: u32, source: &str, time: f64);
    fn cancel_group_timer_event(&self, group_id: u32, source: &str);
    fn init_time_axis(&self, identifier: &str, delays: Vec<f64>, should_loop: bool);
    fn end_time_axis(&self, identifier: &str);
    fn end_all_time_axis(&self);

    // teleport
    fn trans_player_to_pos(&self, uid_list: Vec<u32>, pos: (f32, f32, f32), rot: (f32, f32, f32), scene_id: u32, radius: f32);

    // gadget state (cross-group)
    fn set_group_gadget_state_by_config_id(&self, group_id: u32, config_id: u32, state: u32);
    fn set_gadget_enable_interact(&self, group_id: u32, config_id: u32, enable: bool);

    // monster tide
    fn auto_monster_tide(&self, group_id: u32, source_id: u32, orders_config_id: Vec<u32>, tide_count: u32, scene_limit: u32);
}

pub struct GroupVariableStore {
    // group_id -> (variable_name -> value)
    pub variables: RwLock<HashMap<u32, HashMap<String, i32>>>,
}

impl GroupVariableStore {
    pub fn new() -> Self {
        Self {
            variables: RwLock::new(HashMap::new()),
        }
    }

    pub fn init_group_variables(&self, group_id: u32, vars: HashMap<String, i32>) {
        match self.variables.write() {
            Ok(mut store) => {
                store.insert(group_id, vars);
            }
            Err(_) => {
                eprintln!("Failed to lock variables store");
            }
        }
    }

    pub fn remove_group(&self, group_id: u32) {
        match self.variables.write() {
            Ok(mut store) => {
                store.remove(&group_id);
            }
            Err(_) => {
                eprintln!("Failed to lock variables store");
            }
        }
    }

    pub fn get_variable(&self, group_id: u32, name: &str) -> i32 {
        match self.variables.read() {
            Ok(store) => store
                .get(&group_id)
                .and_then(|vars| vars.get(name))
                .copied()
                .unwrap_or(0),
            Err(_) => {
                eprintln!("Failed to lock variables store");
                return -1;
            }
        }
    }

    pub fn set_variable(&self, group_id: u32, name: &str, value: i32) -> i32 {
        match self.variables.write() {
            Ok(mut store) => {
                if let Some(vars) = store.get_mut(&group_id) {
                    if let Some(val) = vars.get_mut(name) {
                        *val = value;
                        return 0;
                    }
                }
                -1
            }
            Err(_) => {
                eprintln!("Failed to lock variables store");
                return -1;
            }
        }
    }

    pub fn change_variable(&self, group_id: u32, name: &str, delta: i32) -> i32 {
        match self.variables.write() {
            Ok(mut store) => {
                if let Some(vars) = store.get_mut(&group_id) {
                    if let Some(val) = vars.get_mut(name) {
                        *val += delta;
                        return 0;
                    }
                }
                -1
            }
            Err(_) => {
                eprintln!("Failed to lock variables store");
                return -1;
            }
        }
    }

    pub fn reset_group_variables(&self, group_id: u32, vars: &[(String, i32)]) {
        match self.variables.write() {
            Ok(mut store) => {
                if let Some(group_vars) = store.get_mut(&group_id) {
                    for (name, value) in vars {
                        group_vars.insert(name.clone(), *value);
                    }
                }
            }
            Err(_) => {
                eprintln!("Failed to lock variables store");
            }
        }
    }
}

#[derive(Resource, Clone)]
pub struct BevyScriptLib {
    pub queue: Arc<SegQueue<ScriptCommand>>,
    pub variable_store: Arc<GroupVariableStore>,
}

impl ScriptLib for BevyScriptLib {
    fn add_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32) {
        self.queue.push(ScriptCommand::AddExtraGroupSuite {
            ctx,
            group_id,
            suite_id,
        });
    }

    fn remove_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32) {
        self.queue.push(ScriptCommand::RemoveExtraGroupSuite {
            ctx,
            group_id,
            suite_id,
        });
    }

    fn get_group_variable_value(&self, group_id: u32, name: &str) -> i32 {
        self.variable_store.get_variable(group_id, name)
    }

    fn set_group_variable_value(&self, group_id: u32, name: &str, value: i32) -> i32 {
        self.variable_store.set_variable(group_id, name, value)
    }

    fn change_group_variable_value(&self, group_id: u32, name: &str, delta: i32) -> i32 {
        self.variable_store.change_variable(group_id, name, delta)
    }

    fn refresh_group(&self, group_id: u32, suite_id: u32) {
        self.queue
            .push(ScriptCommand::RefreshGroup { group_id, suite_id });
    }

    fn del_worktop_option_by_group_id(&self, group_id: u32, config_id: u32, option: u32) {
        self.queue.push(ScriptCommand::DelWorktopOptionByGroupId {
            group_id,
            config_id,
            option,
        });
    }

    fn set_worktop_options_by_group_id(
        &self,
        group_id: u32,
        config_id: u32,
        option_list: Vec<u32>,
    ) {
        self.queue.push(ScriptCommand::SetWorktopOptionsByGroupId {
            group_id,
            config_id,
            option_list,
        });
    }

    fn get_gadget_state_by_config_id(&self, uid: u32, group_id: u32, config_id: u32) -> i32 {
        nod_krai_gi_data::scene::group_entity_state_cache::get_group_entity_state_cache()
            .get_gadget_state(uid, group_id, config_id)
            .map(|s| s.gadget_state as i32)
            .unwrap_or(-1)
    }

    fn set_gadget_state_by_config_id(&self, _uid: u32, group_id: u32, config_id: u32, state: u32) {
        self.queue.push(ScriptCommand::SetGadgetStateByConfigId {
            group_id,
            config_id,
            state,
        });
    }

    fn get_group_monster_count_by_config_id(&self, uid: u32, group_id: u32) -> i32 {
        nod_krai_gi_data::scene::group_entity_state_cache::get_group_entity_state_cache()
            .get_alive_monster_count(uid, group_id) as i32
    }

    fn active_challenge(
        &self,
        group_id: u32,
        source: u32,
        challenge_id: u32,
        challenge_index: u32,
        param1: u32,
        param2: u32,
        param3: u32,
        param4: u32,
    ) {
        self.queue.push(ScriptCommand::ActiveChallenge {
            group_id,
            source,
            challenge_id,
            challenge_index,
            param1,
            param2,
            param3,
            param4,
        });
    }

    fn stop_challenge(&self, group_id: u32, challenge_index: u32, is_success: bool) {
        self.queue.push(ScriptCommand::StopChallenge {
            group_id,
            challenge_index,
            is_success,
        });
    }

    fn add_challenge_progress(&self, group_id: u32, challenge_index: u32, progress: u32) {
        self.queue.push(ScriptCommand::AddChallengeProgress {
            group_id,
            challenge_index,
            progress,
        });
    }

    fn add_quest_progress(&self, group_id: u32, quest_param: &str) {
        self.queue.push(ScriptCommand::AddQuestProgress {
            group_id,
            quest_param: quest_param.to_string(),
        });
    }

    fn create_gadget(&self, group_id: u32, config_id: u32) {
        self.queue.push(ScriptCommand::CreateGadget { group_id, config_id });
    }

    fn create_monster(&self, group_id: u32, config_id: u32) {
        self.queue.push(ScriptCommand::CreateMonster { group_id, config_id });
    }

    fn kill_entity_by_config_id(&self, group_id: u32, config_id: u32) {
        self.queue.push(ScriptCommand::KillEntityByConfigId { group_id, config_id });
    }

    fn set_is_allow_use_skill(&self, allow: bool) {
        self.queue.push(ScriptCommand::SetIsAllowUseSkill { allow });
    }

    fn show_reminder(&self, reminder_id: u32) {
        self.queue.push(ScriptCommand::ShowReminder { reminder_id });
    }

    fn play_cut_scene(&self, cutscene_id: u32) {
        self.queue.push(ScriptCommand::PlayCutScene { cutscene_id });
    }

    fn show_client_guide(&self, guide_name: &str) {
        self.queue.push(ScriptCommand::ShowClientGuide { guide_name: guide_name.to_string() });
    }

    fn show_common_tips(&self, title: &str, content: &str, close_time: u32) {
        self.queue.push(ScriptCommand::ShowCommonTips { title: title.to_string(), content: content.to_string(), close_time });
    }

    fn close_common_tips(&self) {
        self.queue.push(ScriptCommand::CloseCommonTips);
    }

    fn cause_dungeon_result(&self, is_success: bool) {
        self.queue.push(ScriptCommand::CauseDungeonResult { is_success });
    }

    fn move_player_to_pos(&self, uid: u32, pos: (f32, f32, f32), rot: (f32, f32, f32), scene_id: u32) {
        self.queue.push(ScriptCommand::MovePlayerToPos { uid, pos, rot, scene_id });
    }

    fn scene_play_sound(&self, sound_name: &str, pos: (f32, f32, f32), play_type: u32) {
        self.queue.push(ScriptCommand::ScenePlaySound { sound_name: sound_name.to_string(), pos, play_type });
    }

    fn set_weather_area_state(&self, area_id: u32, climate_type: u32) {
        self.queue.push(ScriptCommand::SetWeatherAreaState { area_id, climate_type });
    }

    fn set_platform_route_id(&self, group_id: u32, config_id: u32, route_id: u32) {
        self.queue.push(ScriptCommand::SetPlatformRouteId { group_id, config_id, route_id });
    }

    fn start_platform(&self, group_id: u32, config_id: u32) {
        self.queue.push(ScriptCommand::StartPlatform { group_id, config_id });
    }

    fn stop_platform(&self, group_id: u32, config_id: u32) {
        self.queue.push(ScriptCommand::StopPlatform { group_id, config_id });
    }

    fn notify_group_lua(&self, group_id: u32, event_type: u32, param1: u32, param2: u32, param3: u32) {
        self.queue.push(ScriptCommand::NotifyGroupLua { group_id, event_type, param1, param2, param3 });
    }

    fn set_entity_server_global_value(&self, group_id: u32, config_id: u32, sgv_name: &str, value: u32) {
        self.queue.push(ScriptCommand::SetEntityServerGlobalValue { group_id, config_id, sgv_name: sgv_name.to_string(), value });
    }

    fn unlock_force(&self, force_id: u32) {
        self.queue.push(ScriptCommand::UnlockForce { force_id });
    }

    fn lock_force(&self, force_id: u32) {
        self.queue.push(ScriptCommand::LockForce { force_id });
    }

    fn kill_group_entity(&self, group_id: u32, kill_policy: u32) {
        self.queue.push(ScriptCommand::KillGroupEntity { group_id, kill_policy });
    }

    fn go_to_group_suite(&self, group_id: u32, suite_id: u32) {
        self.queue.push(ScriptCommand::GoToGroupSuite { group_id, suite_id });
    }

    fn get_group_suite(&self, _group_id: u32) -> u32 {
        // TODO: read from group registry
        0
    }

    fn set_group_variable_value_by_group(&self, group_id: u32, name: &str, value: i32) -> i32 {
        self.variable_store.set_variable(group_id, name, value);
        self.queue.push(ScriptCommand::SetGroupVariableValueByGroup {
            group_id,
            name: name.to_string(),
            value,
        });
        0
    }

    fn change_group_variable_value_by_group(&self, group_id: u32, name: &str, delta: i32) -> i32 {
        self.variable_store.change_variable(group_id, name, delta);
        self.queue.push(ScriptCommand::ChangeGroupVariableValueByGroup {
            group_id,
            name: name.to_string(),
            delta,
        });
        0
    }

    fn create_group_timer_event(&self, group_id: u32, source: &str, time: f64) {
        self.queue.push(ScriptCommand::CreateGroupTimerEvent {
            group_id,
            source: source.to_string(),
            time,
        });
    }

    fn cancel_group_timer_event(&self, group_id: u32, source: &str) {
        self.queue.push(ScriptCommand::CancelGroupTimerEvent {
            group_id,
            source: source.to_string(),
        });
    }

    fn init_time_axis(&self, identifier: &str, delays: Vec<f64>, should_loop: bool) {
        self.queue.push(ScriptCommand::InitTimeAxis {
            identifier: identifier.to_string(),
            delays,
            should_loop,
        });
    }

    fn end_time_axis(&self, identifier: &str) {
        self.queue.push(ScriptCommand::EndTimeAxis {
            identifier: identifier.to_string(),
        });
    }

    fn end_all_time_axis(&self) {
        self.queue.push(ScriptCommand::EndAllTimeAxis);
    }

    fn trans_player_to_pos(&self, uid_list: Vec<u32>, pos: (f32, f32, f32), rot: (f32, f32, f32), scene_id: u32, radius: f32) {
        self.queue.push(ScriptCommand::TransPlayerToPos { uid_list, pos, rot, scene_id, radius });
    }

    fn set_group_gadget_state_by_config_id(&self, group_id: u32, config_id: u32, state: u32) {
        self.queue.push(ScriptCommand::SetGroupGadgetStateByConfigId { group_id, config_id, state });
    }

    fn set_gadget_enable_interact(&self, group_id: u32, config_id: u32, enable: bool) {
        self.queue.push(ScriptCommand::SetGadgetEnableInteract { group_id, config_id, enable });
    }

    fn auto_monster_tide(&self, group_id: u32, source_id: u32, orders_config_id: Vec<u32>, tide_count: u32, scene_limit: u32) {
        self.queue.push(ScriptCommand::AutoMonsterTide { group_id, source_id, orders_config_id, tide_count, scene_limit });
    }
}

pub fn call_lua_trigger_condition(
    func: &Function,
    context: LuaContext,
    evt: LuaEvt,
) -> mlua::Result<bool> {
    match func.call((context, evt)) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            tracing::debug!("Lua trigger function returned error: {:?}", e);
            Err(e)
        }
    }
}

pub fn call_lua_trigger_action(
    func: &Function,
    context: LuaContext,
    evt: LuaEvt,
) -> mlua::Result<i32> {
    match func.call((context, evt)) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            tracing::debug!("Lua trigger function returned error: {:?}", e);
            Err(e)
        }
    }
}

pub fn call_lua_on_client_execute_req(
    func: &Function,
    context: LuaContext,
    param1: u32,
    param2: u32,
    param3: u32,
) -> mlua::Result<bool> {
    match func.call((context, param1, param2, param3)) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            tracing::debug!("Lua trigger function returned error: {:?}", e);
            Err(e)
        }
    }
}

pub fn call_lua_on_be_hurt(
    func: &Function,
    context: LuaContext,
    param1: u32,
    param2: u32,
    param3: bool,
) -> mlua::Result<Option<bool>> {
    func.call((context, param1, param2, param3))
}

mod challenge_manager;
mod challenge_notify;
mod script_entity;
mod script_group_manager;
mod script_lib;
mod script_lib_handle;
mod script_load;
mod script_lua_vm;

use crate::challenge_manager::{
    challenge_timer_system, ChallengeManager, ChallengeTimer, ChallengeUpdateResult,
};
use crate::challenge_notify::{
    handle_challenge_finish_event, handle_challenge_progress_event, handle_challenge_start_event,
};
use crate::script_entity::{
    despawn_group_entity, despawn_suite_entities, refresh_group_entity, spawn_group_entity,
    spawn_suite_entities,
};
use crate::script_group_manager::{
    handle_player_move_for_group_loading, handle_player_move_for_region_entry, GroupLoadManager,
};
use crate::script_lib::{
    call_lua_on_be_hurt, call_lua_on_client_execute_req, call_lua_trigger_action, call_lua_trigger_condition,
    BevyScriptLib, GroupVariableStore,
};
use crate::script_load::{GroupLoadState, SceneGroupRuntime};
use crate::script_lua_vm::LuaRuntime;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use crossbeam_queue::SegQueue;
use mlua::Function;
use nod_krai_gi_data::scene::{EventType, LuaEvt, ScriptCommand};
use nod_krai_gi_entity::common::{ConfigId, GroupId, ProtocolEntityID};
use nod_krai_gi_entity::gadget::State;
use nod_krai_gi_event::entity::{GadgetStateChangeEvent, SetWorktopOptionsEvent};
use nod_krai_gi_event::lua::{
    ChallengeFinishEvent, ChallengeProgressEvent, ChallengeStartEvent, DespawnGroupEntityEvent,
    DespawnSuiteEntitiesEvent, LuaTriggerEvent, MonsterKillEvent, OnBeHurtEvent,
    OnClientExecuteReqEvent, RefreshGroupEntityEvent, ScriptCommandEvent, SpawnGroupEntityEvent,
    SpawnSuiteEntitiesEvent,
};
use nod_krai_gi_event::quest::QuestContentProgressEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_event::scene::{WorldOwnerUID, WorldVersionConfig};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ScriptPlugin;

#[derive(Resource)]
pub struct ScriptCommandQueue(pub Arc<SegQueue<ScriptCommand>>);

#[derive(Resource, Clone)]
pub struct SharedVariableStore(pub Arc<GroupVariableStore>);

#[derive(Default)]
pub struct SceneGroupRegistry {
    pub groups: HashMap<u32, GroupLoadState>,
}

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        let protocol_version = app.world_mut().get_resource::<WorldVersionConfig>().unwrap().protocol_version.clone();

        let queue = Arc::new(SegQueue::new());
        let variable_store = Arc::new(GroupVariableStore::new());

        let script_lib = BevyScriptLib {
            queue: queue.clone(),
            variable_store: variable_store.clone(),
        };

        let lua_vm = LuaRuntime::new(Arc::new(script_lib.clone()), protocol_version.to_string());

        app.insert_resource(script_lib)
            .insert_resource(lua_vm)
            .insert_resource(ScriptCommandQueue(queue))
            .insert_resource(SharedVariableStore(variable_store))
            .insert_non_send_resource(SceneGroupRegistry::default())
            .insert_resource(GroupLoadManager::default())
            .insert_resource(ChallengeManager::new())
            .insert_resource(ChallengeTimer::default())
            .add_systems(Update, script_command_system)
            .add_systems(Update, handle_script_command_group_event)
            .add_systems(Update, handle_script_command_challenge_event)
            .add_systems(Update, handle_script_command_quest_event)
            .add_systems(Update, lua_trigger_event_system)
            .add_systems(Update, gadget_lua_on_client_execute_req)
            .add_systems(Update, gadget_lua_on_be_hurt)
            .add_systems(Update, set_worktop_options_event)
            .add_systems(Update, set_gadget_state_event)
            .add_systems(Update, handle_player_move_for_group_loading)
            .add_systems(Update, handle_player_move_for_region_entry)
            .add_systems(Update, spawn_group_entity)
            .add_systems(Update, despawn_group_entity)
            .add_systems(Update, spawn_suite_entities)
            .add_systems(Update, despawn_suite_entities)
            .add_systems(Update, refresh_group_entity)
            .add_systems(Update, challenge_timer_system)
            .add_systems(Update, handle_challenge_start_event)
            .add_systems(Update, handle_challenge_finish_event)
            .add_systems(Update, handle_challenge_progress_event)
            .add_systems(Update, handle_monster_kill_challenge_event)
            .add_systems(Update, handle_monster_kill_quest_event)
            .add_systems(Update, handle_gadget_state_change_quest_event)
            .add_systems(Update, handle_gadget_interact_quest_event);
    }
}

fn script_command_system(
    queue: Res<ScriptCommandQueue>,
    mut script_command_events: MessageWriter<ScriptCommandEvent>,
) {
    while let Some(cmd) = queue.0.pop() {
        script_command_events.write(ScriptCommandEvent { command: cmd });
    }
}

fn handle_script_command_group_event(
    mut ev_reader: MessageReader<ScriptCommandEvent>,
    mut registry: NonSendMut<SceneGroupRegistry>,
    variable_store: Res<SharedVariableStore>,
    mut lua_vm: ResMut<LuaRuntime>,
    mut spawn_group_entity_event: MessageWriter<SpawnGroupEntityEvent>,
    mut despawn_group_entity_event: MessageWriter<DespawnGroupEntityEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut spawn_suite_entities_event: MessageWriter<SpawnSuiteEntitiesEvent>,
    mut despawn_suite_entities_event: MessageWriter<DespawnSuiteEntitiesEvent>,
    mut refresh_group_entity_event: MessageWriter<RefreshGroupEntityEvent>,
    mut challenge_manager: ResMut<ChallengeManager>,
    mut challenge_finish_events: MessageWriter<ChallengeFinishEvent>,
    mut gadget_state_change_events: MessageWriter<GadgetStateChangeEvent>,
    entities: Query<(Entity, &ProtocolEntityID, &GroupId, &ConfigId, &State)>,
    message_output: Res<MessageOutput>,
    world_owner_uid: Res<WorldOwnerUID>,
) {
    for event in ev_reader.read() {
        match &event.command {
            ScriptCommand::AddExtraGroupSuite {
                ctx,
                group_id,
                suite_id,
            } => {
                tracing::debug!("[Script] [AddExtraGroupSuite] context:{:#?}", ctx);

                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(&group_id) {
                    if rt.active_suites.contains(&suite_id) {
                        tracing::debug!(
                            "[Script] [AddExtraGroupSuite] suite {} already active in group {}",
                            suite_id,
                            group_id
                        );
                        continue;
                    }

                    rt.active_suites.push(*suite_id);
                    rt.recompute_active_triggers();

                    let block_id = rt.data.base_info.block_id;
                    spawn_suite_entities_event.write(SpawnSuiteEntitiesEvent {
                        group_id: *group_id,
                        block_id,
                        suite_id: *suite_id,
                    });
                }
            }

            ScriptCommand::RemoveExtraGroupSuite {
                ctx,
                group_id,
                suite_id,
            } => {
                tracing::debug!("[Script] [RemoveExtraGroupSuite] context:{:#?}", ctx);

                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(&group_id) {
                    if !rt.active_suites.contains(&suite_id) {
                        tracing::debug!(
                            "[Script] [RemoveExtraGroupSuite] suite {} not active in group {}",
                            suite_id,
                            group_id
                        );
                        continue;
                    }

                    despawn_suite_entities_event.write(DespawnSuiteEntitiesEvent {
                        group_id: *group_id,
                        suite_id: *suite_id,
                    });

                    rt.active_suites.retain(|s| *s != *suite_id);
                    rt.recompute_active_triggers();
                }
            }

            ScriptCommand::LoadGroup {
                scene_id,
                block_id,
                group_id,
            } => match registry.groups.get(&group_id) {
                Some(GroupLoadState::Loaded(_)) => {
                    tracing::debug!("[Extra] [LoadGroup] Group {} already loaded", group_id);
                }
                Some(GroupLoadState::Loading) => {
                    tracing::debug!("[Extra] [LoadGroup] Group {} is loading", group_id);
                }
                _ => {
                    tracing::debug!("[Extra] [LoadGroup] Loading group {}...", group_id);

                    registry.groups.insert(*group_id, GroupLoadState::Loading);

                    let runtime =
                        SceneGroupRuntime::new(*scene_id, *block_id, *group_id, &mut lua_vm);

                    match runtime {
                        Some(rt) => {
                            let vars = rt.variables.clone();
                            variable_store.0.init_group_variables(*group_id, vars);
                            registry
                                .groups
                                .insert(*group_id, GroupLoadState::Loaded(rt));
                            tracing::debug!("[Extra] [LoadGroup] Group {} loaded", *group_id);
                            spawn_group_entity_event.write(SpawnGroupEntityEvent {
                                scene_id: *scene_id,
                                block_id: *block_id,
                                group_id: *group_id,
                                refresh_suite_id: 0,
                            });
                        }
                        None => {
                            registry.groups.insert(*group_id, GroupLoadState::Failed);
                            tracing::debug!(
                                "[Extra] [LoadGroup] Group {} failed to load",
                                *group_id
                            );
                        }
                    }
                }
            },

            ScriptCommand::UnloadGroup { group_id } => {
                tracing::debug!("[Extra] [UnloadGroup] context:{}", group_id);

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: *group_id,
                    event_type: EventType::EventGroupWillUnload,
                    evt: LuaEvt {
                        param1: 0,
                        param2: 0,
                        param3: 0,
                        source_eid: 0,
                        target_eid: 0,
                    },
                });
                despawn_group_entity_event.write(DespawnGroupEntityEvent {
                    group_id: *group_id,
                });
                variable_store.0.remove_group(*group_id);
                registry.groups.remove(&group_id);

                let removed_challenges = challenge_manager.remove_group_challenges(*group_id);
                for finish_event in removed_challenges {
                    lua_trigger_events.write(LuaTriggerEvent {
                        group_id: finish_event.group_id,
                        event_type: EventType::EventChallengeFail,
                        evt: LuaEvt {
                            param1: finish_event.challenge_index,
                            param2: finish_event.challenge_id,
                            param3: 0,
                            source_eid: 0,
                            target_eid: 0,
                        },
                    });
                    challenge_finish_events.write(finish_event);
                }

                tracing::debug!("[Extra] [UnloadGroup] Group {} unloaded", group_id);
            }

            ScriptCommand::RefreshGroup { group_id, suite_id } => {
                tracing::debug!(
                    "[Script] [RefreshGroup] group={} suite={}",
                    group_id,
                    suite_id
                );

                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(group_id) {
                    let block_id = rt.data.base_info.block_id;

                    refresh_group_entity_event.write(RefreshGroupEntityEvent {
                        group_id: *group_id,
                        suite_id: *suite_id,
                        block_id,
                    });

                    rt.active_suites = vec![*suite_id];
                    rt.recompute_active_triggers();

                    let initial_vars: HashMap<String, i32> = rt
                        .data
                        .variables
                        .iter()
                        .map(|v| (v.name.clone(), v.value))
                        .collect();
                    rt.variables = initial_vars.clone();
                    variable_store
                        .0
                        .init_group_variables(*group_id, initial_vars);
                }
            }

            ScriptCommand::CreateGadget { group_id, config_id } => {
                tracing::debug!(
                    "[Script] [CreateGadget] group={} config={}",
                    group_id,
                    config_id
                );
                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(group_id) {
                    if rt.data.gadgets.iter().any(|g| g.config_id == *config_id) {
                        spawn_group_entity_event.write(SpawnGroupEntityEvent {
                            scene_id: rt.data.base_info.scene_id,
                            block_id: rt.data.base_info.block_id,
                            group_id: *group_id,
                            refresh_suite_id: 0,
                        });
                    }
                }
            }

            ScriptCommand::CreateMonster { group_id, config_id } => {
                tracing::debug!(
                    "[Script] [CreateMonster] group={} config={}",
                    group_id,
                    config_id
                );
                if let Some(GroupLoadState::Loaded(_rt)) = registry.groups.get(group_id) {
                    tracing::debug!("[Script] [CreateMonster] dispatched for group {}", group_id);
                }
            }

            ScriptCommand::KillEntityByConfigId { group_id, config_id } => {
                tracing::debug!(
                    "[Script] [KillEntityByConfigId] group={} config={}",
                    group_id,
                    config_id
                );
            }

            ScriptCommand::NotifyGroupLua {
                group_id,
                event_type,
                param1,
                param2,
                param3,
            } => {
                tracing::debug!(
                    "[Script] [NotifyGroupLua] group={} event_type={} params=({},{},{})",
                    group_id,
                    event_type,
                    param1,
                    param2,
                    param3
                );
                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: *group_id,
                    event_type: EventType::from(*event_type),
                    evt: LuaEvt {
                        param1: *param1,
                        param2: *param2,
                        param3: *param3,
                        source_eid: 0,
                        target_eid: 0,
                    },
                });
            }

            ScriptCommand::SetIsAllowUseSkill { allow } => {
                tracing::debug!("[Script] [SetIsAllowUseSkill] allow={}", allow);
                message_output.send(
                    world_owner_uid.0,
                    "CanUseSkillNotify",
                    nod_krai_gi_proto::normal::CanUseSkillNotify {
                        is_can_use_skill: *allow,
                    },
                );
            }

            ScriptCommand::ShowReminder { reminder_id } => {
                tracing::debug!("[Script] [ShowReminder] id={}", reminder_id);
                message_output.send(
                    world_owner_uid.0,
                    "DungeonShowReminderNotify",
                    nod_krai_gi_proto::normal::DungeonShowReminderNotify {
                        reminder_id: *reminder_id,
                    },
                );
            }

            ScriptCommand::PlayCutScene { cutscene_id } => {
                tracing::debug!("[Script] [PlayCutScene] id={}", cutscene_id);
            }

            ScriptCommand::CauseDungeonResult { is_success } => {
                tracing::debug!("[Script] [CauseDungeonResult] success={}", is_success);
                let result_param = if *is_success { 1 } else { 0 };
                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: 0,
                    event_type: EventType::EventDungeonSettle,
                    evt: LuaEvt {
                        param1: result_param,
                        ..Default::default()
                    },
                });
            }

            ScriptCommand::ScenePlaySound {
                sound_name,
                pos,
                play_type,
            } => {
                tracing::debug!(
                    "[Script] [ScenePlaySound] name={} pos=({},{},{}) type={}",
                    sound_name,
                    pos.0,
                    pos.1,
                    pos.2,
                    play_type
                );
            }

            ScriptCommand::SetWeatherAreaState {
                area_id,
                climate_type,
            } => {
                tracing::debug!(
                    "[Script] [SetWeatherAreaState] area={} climate={}",
                    area_id,
                    climate_type
                );
            }

            ScriptCommand::SetPlatformRouteId {
                group_id,
                config_id,
                route_id,
            } => {
                tracing::debug!(
                    "[Script] [SetPlatformRouteId] group={} config={} route={}",
                    group_id,
                    config_id,
                    route_id
                );
            }

            ScriptCommand::StartPlatform {
                group_id,
                config_id,
            } => {
                tracing::debug!(
                    "[Script] [StartPlatform] group={} config={}",
                    group_id,
                    config_id
                );
            }

            ScriptCommand::StopPlatform {
                group_id,
                config_id,
            } => {
                tracing::debug!(
                    "[Script] [StopPlatform] group={} config={}",
                    group_id,
                    config_id
                );
            }

            ScriptCommand::UnlockForce { force_id } => {
                tracing::debug!("[Script] [UnlockForce] force={}", force_id);
            }

            ScriptCommand::LockForce { force_id } => {
                tracing::debug!("[Script] [LockForce] force={}", force_id);
            }

            ScriptCommand::ShowClientGuide { guide_name } => {
                tracing::debug!("[Script] [ShowClientGuide] name={}", guide_name);
                message_output.send(
                    world_owner_uid.0,
                    "ShowClientGuideNotify",
                    nod_krai_gi_proto::normal::ShowClientGuideNotify {
                        guide_name: guide_name.clone(),
                    },
                );
            }

            ScriptCommand::ShowCommonTips {
                title,
                content,
                close_time,
            } => {
                tracing::debug!(
                    "[Script] [ShowCommonTips] title={} close_time={}",
                    title,
                    close_time
                );
                message_output.send(
                    world_owner_uid.0,
                    "ShowCommonTipsNotify",
                    nod_krai_gi_proto::normal::ShowCommonTipsNotify {
                        title: title.clone(),
                        content: content.clone(),
                        close_time: *close_time,
                    },
                );
            }

            ScriptCommand::CloseCommonTips => {
                tracing::debug!("[Script] [CloseCommonTips]");
            }

            ScriptCommand::MovePlayerToPos {
                uid,
                pos,
                rot: _,
                scene_id,
            } => {
                tracing::debug!(
                    "[Script] [MovePlayerToPos] uid={} pos=({},{},{}) scene={}",
                    uid,
                    pos.0,
                    pos.1,
                    pos.2,
                    scene_id
                );
            }

            ScriptCommand::SetEntityServerGlobalValue {
                group_id,
                config_id,
                sgv_name,
                value,
            } => {
                tracing::debug!(
                    "[Script] [SetEntityServerGlobalValue] group={} config={} key={} val={}",
                    group_id,
                    config_id,
                    sgv_name,
                    value
                );
            }

            ScriptCommand::SendServerMessageByLuaKey { key, params: _ } => {
                tracing::debug!("[Script] [SendServerMessageByLuaKey] key={}", key);
            }

            ScriptCommand::KillGroupEntity { group_id, kill_policy } => {
                tracing::debug!(
                    "[Script] [KillGroupEntity] group={} policy={}",
                    group_id,
                    kill_policy
                );
                // KillPolicy: 0=none, 1=all, 2=monster, 3=gadget
                let _ = variable_store;
            }

            ScriptCommand::GoToGroupSuite { group_id, suite_id } => {
                tracing::debug!(
                    "[Script] [GoToGroupSuite] group={} suite={}",
                    group_id,
                    suite_id
                );
                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(group_id) {
                    rt.active_suites = vec![*suite_id];
                    rt.recompute_active_triggers();
                }
            }

            ScriptCommand::SetGroupVariableValueByGroup { group_id, name, value } => {
                tracing::debug!(
                    "[Script] [SetGroupVariableValueByGroup] group={} name={} val={}",
                    group_id,
                    name,
                    value
                );
                variable_store.0.set_variable(*group_id, name, *value);
            }

            ScriptCommand::ChangeGroupVariableValueByGroup { group_id, name, delta } => {
                tracing::debug!(
                    "[Script] [ChangeGroupVariableValueByGroup] group={} name={} delta={}",
                    group_id,
                    name,
                    delta
                );
                variable_store.0.change_variable(*group_id, name, *delta);
            }

            ScriptCommand::CreateGroupTimerEvent { group_id, source, time } => {
                tracing::debug!(
                    "[Script] [CreateGroupTimerEvent] group={} source={} time={}",
                    group_id,
                    source,
                    time
                );
            }

            ScriptCommand::CancelGroupTimerEvent { group_id, source } => {
                tracing::debug!(
                    "[Script] [CancelGroupTimerEvent] group={} source={}",
                    group_id,
                    source
                );
            }

            ScriptCommand::InitTimeAxis { identifier, delays, should_loop } => {
                tracing::debug!(
                    "[Script] [InitTimeAxis] id={} delays={:?} loop={}",
                    identifier,
                    delays,
                    should_loop
                );
            }

            ScriptCommand::EndTimeAxis { identifier } => {
                tracing::debug!("[Script] [EndTimeAxis] id={}", identifier);
            }

            ScriptCommand::EndAllTimeAxis => {
                tracing::debug!("[Script] [EndAllTimeAxis]");
            }

            ScriptCommand::TransPlayerToPos { uid_list, pos, rot: _, scene_id, radius: _, } => {
                tracing::debug!(
                    "[Script] [TransPlayerToPos] uids={:?} pos=({},{},{}) scene={}",
                    uid_list,
                    pos.0, pos.1, pos.2,
                    scene_id
                );
            }

            ScriptCommand::SetGroupGadgetStateByConfigId { group_id, config_id, state } => {
                tracing::debug!(
                    "[Script] [SetGroupGadgetStateByConfigId] group={} config={} state={}",
                    group_id,
                    config_id,
                    state
                );
                entities
                    .iter()
                    .filter(|(_, _, e_group_id, e_config_id, _)| {
                        e_group_id.0 == *group_id && e_config_id.0 == *config_id
                    })
                    .for_each(|(entity, entity_id, _, _, gadget_state)| {
                        gadget_state_change_events.write(GadgetStateChangeEvent {
                            entity,
                            state_id: *state,
                            previous_state_id: Some(gadget_state.0),
                        });
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id: *group_id,
                            event_type: EventType::EventGadgetStateChange,
                            evt: LuaEvt {
                                param1: *state,
                                param2: *config_id,
                                param3: gadget_state.0,
                                source_eid: entity_id.0,
                                target_eid: entity_id.0,
                            },
                        });
                    });
            }

            ScriptCommand::SetGadgetEnableInteract { group_id, config_id, enable } => {
                tracing::debug!(
                    "[Script] [SetGadgetEnableInteract] group={} config={} enable={}",
                    group_id,
                    config_id,
                    enable
                );
            }

            ScriptCommand::AutoMonsterTide { group_id, source_id, orders_config_id: _, tide_count, scene_limit } => {
                tracing::debug!(
                    "[Script] [AutoMonsterTide] group={} source={} tide={} limit={}",
                    group_id,
                    source_id,
                    tide_count,
                    scene_limit
                );
            }

            _ => {}
        };
    }
}

fn handle_script_command_challenge_event(
    mut ev_reader: MessageReader<ScriptCommandEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut challenge_manager: ResMut<ChallengeManager>,
    mut challenge_start_events: MessageWriter<ChallengeStartEvent>,
    mut challenge_finish_events: MessageWriter<ChallengeFinishEvent>,
) {
    for event in ev_reader.read() {
        match &event.command {
            ScriptCommand::ActiveChallenge {
                group_id,
                source,
                challenge_id,
                challenge_index,
                param1,
                param2,
                param3,
                param4,
            } => {
                tracing::debug!(
                    "[Script] [ActiveChallenge] group={} challenge_id={} index={}",
                    group_id,
                    challenge_id,
                    challenge_index
                );

                let dungeon_challenge_config_collection_clone =
                    Arc::clone(nod_krai_gi_data::excel::dungeon_challenge_config_collection::get());

                let config = dungeon_challenge_config_collection_clone.get(challenge_id);

                let challenge_type = config.map(|c| c.challenge_type).unwrap_or_default();

                let index = if *challenge_index == 0 {
                    challenge_manager.get_next_challenge_index()
                } else {
                    *challenge_index
                };

                let challenge = nod_krai_gi_data::scene::challenge::ActiveChallenge::new(
                    *source,
                    *group_id,
                    *challenge_id,
                    *param1,
                    *param2,
                    *param3,
                    *param4,
                    index,
                    challenge_type,
                );

                let start_event = challenge_manager.start_challenge(challenge);

                challenge_start_events.write(start_event);
            }

            ScriptCommand::StopChallenge {
                group_id,
                challenge_index,
                is_success,
            } => {
                tracing::debug!(
                    "[Script] [StopChallenge] group={} index={} is_success={}",
                    group_id,
                    challenge_index,
                    is_success
                );

                if let Some((challenge, finish_event)) =
                    challenge_manager.stop_challenge(*challenge_index, *is_success)
                {
                    let event_type = if *is_success {
                        EventType::EventChallengeSuccess
                    } else {
                        EventType::EventChallengeFail
                    };

                    lua_trigger_events.write(LuaTriggerEvent {
                        group_id: *group_id,
                        event_type,
                        evt: LuaEvt {
                            param1: *challenge_index,
                            param2: challenge.challenge_id,
                            param3: 0,
                            source_eid: 0,
                            target_eid: 0,
                        },
                    });

                    challenge_finish_events.write(finish_event);
                }
            }

            _ => {}
        };
    }
}

fn handle_script_command_quest_event(
    mut ev_reader: MessageReader<ScriptCommandEvent>,
    world_owner_uid: Res<WorldOwnerUID>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
) {
    use nod_krai_gi_data::quest::quest_config::QuestContent;

    for event in ev_reader.read() {
        match &event.command {
            ScriptCommand::AddQuestProgress {
                group_id,
                quest_param,
            } => {
                let param = quest_param.parse::<u32>().unwrap_or(0);
                tracing::debug!(
                    "[Script] [AddQuestProgress] group={} param={} parsed_param={}",
                    group_id,
                    quest_param,
                    param
                );
                quest_content_events.write(QuestContentProgressEvent {
                    player_uid: world_owner_uid.0,
                    content_type: QuestContent::AddQuestProgress,
                    param,
                    param2: *group_id,
                    param3: 0,
                    add_progress: 1,
                });
            }
            _ => {}
        };
    }
}

fn handle_monster_kill_challenge_event(
    mut ev_reader: MessageReader<MonsterKillEvent>,
    mut challenge_manager: ResMut<ChallengeManager>,
    mut challenge_progress_events: MessageWriter<ChallengeProgressEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut challenge_finish_events: MessageWriter<ChallengeFinishEvent>,
) {
    for event in ev_reader.read() {
        if let Some(challenge_index) =
            challenge_manager.get_challenge_for_monster_kill(event.group_id)
        {
            tracing::debug!(
                "[MonsterKillChallenge] Monster kill matched challenge {} for group {}",
                challenge_index,
                event.group_id
            );

            if let Some(progress_event) = challenge_manager.kill_monster(challenge_index, 1) {
                challenge_progress_events.write(progress_event);

                let result = challenge_manager.update_challenges_by_index(challenge_index);

                match result {
                    ChallengeUpdateResult::Running { .. } => {}
                    ChallengeUpdateResult::Success {
                        challenge_index,
                        group_id,
                        challenge_id,
                        time_cost,
                    } => {
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id,
                            event_type: EventType::EventChallengeSuccess,
                            evt: LuaEvt {
                                ..Default::default()
                            },
                        });
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id,
                            event_type: EventType::EventDungeonSettle,
                            evt: LuaEvt {
                                param1: 1,
                                ..Default::default()
                            },
                        });
                        challenge_finish_events.write(ChallengeFinishEvent {
                            group_id,
                            challenge_id,
                            challenge_index: challenge_index,
                            is_success: true,
                            time_cost: time_cost,
                        });
                    }
                    ChallengeUpdateResult::Failed {
                        challenge_index,
                        group_id,
                        challenge_id,
                        time_cost,
                    } => {
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id,
                            event_type: EventType::EventChallengeFail,
                            evt: LuaEvt {
                                ..Default::default()
                            },
                        });
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id,
                            event_type: EventType::EventDungeonSettle,
                            evt: LuaEvt {
                                param1: 0,
                                ..Default::default()
                            },
                        });
                        challenge_finish_events.write(ChallengeFinishEvent {
                            group_id,
                            challenge_id,
                            challenge_index,
                            is_success: false,
                            time_cost,
                        });
                    }
                }
            }
        }
    }
}

fn lua_trigger_event_system(
    mut ev_reader: MessageReader<LuaTriggerEvent>,
    mut registry: NonSendMut<SceneGroupRegistry>,
) {
    use std::collections::hash_map::Entry;

    for event in ev_reader.read() {
        let group_id = event.group_id;

        match registry.groups.entry(group_id) {
            Entry::Vacant(_v) => {
                //...
            }

            Entry::Occupied(mut o) => match o.get_mut() {
                GroupLoadState::Loading => continue,
                GroupLoadState::Failed => continue,

                GroupLoadState::Loaded(rt) => {
                    let Some(all_triggers) = rt.triggers_by_event.get(&event.event_type) else {
                        continue;
                    };

                    tracing::debug!("[LuaTrigger] all_triggers:{}", all_triggers.len());

                    for trig in all_triggers {
                        if !rt.active_trigger_names.contains(&trig.name) {
                            continue;
                        }

                        tracing::debug!("[LuaTrigger] trigger event:{:#?}", trig.name);

                        let should_execute = match &trig.condition {
                            Some(cond) => call_lua_trigger_condition(cond, rt.context, event.evt)
                                .unwrap_or(false),
                            None => true,
                        };
                        if should_execute {
                            if let Some(action) = &trig.action {
                                let _ = call_lua_trigger_action(action, rt.context, event.evt);
                            }
                        }
                    }
                }

                GroupLoadState::Unloaded => unreachable!(),
            },
        }
    }
}

fn gadget_lua_on_client_execute_req(
    mut ev_reader: MessageReader<OnClientExecuteReqEvent>,
    lua_runtime: Res<LuaRuntime>,
) {
    for event in ev_reader.read() {
        let lua = &lua_runtime.lua;
        let fun: Option<Function> = lua
            .globals()
            .get::<mlua::Table>(format!("{}.lua", event.lua_name))
            .ok()
            .and_then(|env| env.get("OnClientExecuteReq").ok());
        match fun {
            None => {}
            Some(fun) => {
                let _ = call_lua_on_client_execute_req(
                    &fun,
                    event.lua_context,
                    event.param1,
                    event.param2,
                    event.param3,
                );
            }
        }
    }
}

fn gadget_lua_on_be_hurt(
    mut ev_reader: MessageReader<OnBeHurtEvent>,
    lua_runtime: Res<LuaRuntime>,
) {
    for event in ev_reader.read() {
        let lua = &lua_runtime.lua;

        let fun: Option<Function> = lua
            .globals()
            .get::<mlua::Table>(format!("{}.lua", event.lua_name))
            .ok()
            .and_then(|env| env.get("OnBeHurt").ok());

        match fun {
            None => {}
            Some(fun) => {
                let _ = call_lua_on_be_hurt(
                    &fun,
                    event.lua_context,
                    event.element_type,
                    event.strike_type,
                    event.is_host,
                );
            }
        }
    }
}

fn set_worktop_options_event(
    mut ev_reader: MessageReader<ScriptCommandEvent>,
    mut select_worktop_option_events: MessageWriter<SetWorktopOptionsEvent>,
    world_owner_uid: Res<WorldOwnerUID>,
    entities: Query<(&ProtocolEntityID, &GroupId, &ConfigId)>,
) {
    for event in ev_reader.read() {
        match &event.command {
            ScriptCommand::SetWorktopOptionsByGroupId {
                group_id,
                config_id,
                option_list,
            } => {
                entities
                    .iter()
                    .filter(|(_, e_group_id, e_config_id)| {
                        e_group_id.0 == *group_id && e_config_id.0 == *config_id
                    })
                    .for_each(|(entity_id, _, _)| {
                        select_worktop_option_events.write(SetWorktopOptionsEvent {
                            player_uid: world_owner_uid.0,
                            group_id: *group_id,
                            config_id: *config_id,
                            gadget_entity_id: entity_id.0,
                            option_list: option_list.clone(),
                            del_option: 0,
                        });
                    });
            }
            ScriptCommand::DelWorktopOptionByGroupId {
                group_id,
                config_id,
                option,
            } => {
                entities
                    .iter()
                    .filter(|(_, e_group_id, e_config_id)| {
                        e_group_id.0 == *group_id && e_config_id.0 == *config_id
                    })
                    .for_each(|(entity_id, _, _)| {
                        select_worktop_option_events.write(SetWorktopOptionsEvent {
                            player_uid: world_owner_uid.0,
                            group_id: *group_id,
                            config_id: *config_id,
                            gadget_entity_id: entity_id.0,
                            option_list: vec![],
                            del_option: *option,
                        });
                    });
            }
            _ => {}
        };
    }
}

fn set_gadget_state_event(
    mut ev_reader: MessageReader<ScriptCommandEvent>,
    mut gadget_state_change_events: MessageWriter<GadgetStateChangeEvent>,
    entities: Query<(Entity, &ProtocolEntityID, &GroupId, &ConfigId, &State)>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
) {
    for event in ev_reader.read() {
        match &event.command {
            ScriptCommand::SetGadgetStateByConfigId {
                group_id,
                config_id,
                state,
            } => {
                entities
                    .iter()
                    .filter(|(_, _, e_group_id, e_config_id, _)| {
                        e_group_id.0 == *group_id && e_config_id.0 == *config_id
                    })
                    .for_each(|(entity, entity_id, _, _, gadget_state)| {
                        gadget_state_change_events.write(GadgetStateChangeEvent {
                            entity,
                            state_id: *state,
                            previous_state_id: Some(gadget_state.0),
                        });

                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id: *group_id,
                            event_type: EventType::EventGadgetStateChange,
                            evt: LuaEvt {
                                param1: *state,
                                param2: *config_id,
                                param3: gadget_state.0,
                                source_eid: entity_id.0,
                                target_eid: entity_id.0,
                            },
                        });
                    });
            }
            _ => {}
        };
    }
}

fn handle_monster_kill_quest_event(
    mut ev_reader: MessageReader<MonsterKillEvent>,
    world_owner_uid: Res<WorldOwnerUID>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
) {
    use nod_krai_gi_data::quest::quest_config::QuestContent;

    for event in ev_reader.read() {
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: world_owner_uid.0,
            content_type: QuestContent::MonsterDie,
            param: event.monster_id,
            param2: event.group_id,
            param3: event.config_id,
            add_progress: 1,
        });
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: world_owner_uid.0,
            content_type: QuestContent::KillMonster,
            param: event.monster_id,
            param2: event.group_id,
            param3: event.config_id,
            add_progress: 1,
        });
    }
}

fn handle_gadget_state_change_quest_event(
    mut ev_reader: MessageReader<GadgetStateChangeEvent>,
    world_owner_uid: Res<WorldOwnerUID>,
    entities: Query<(&GroupId, &ConfigId)>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
) {
    use nod_krai_gi_data::quest::quest_config::QuestContent;

    for event in ev_reader.read() {
        let Ok((group_id, config_id)) = entities.get(event.entity) else {
            continue;
        };
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: world_owner_uid.0,
            content_type: QuestContent::DestroyGadget,
            param: config_id.0,
            param2: group_id.0,
            param3: 0,
            add_progress: 1,
        });
    }
}

fn handle_gadget_interact_quest_event(
    mut ev_reader: MessageReader<nod_krai_gi_event::entity::GadgetInteractEvent>,
    world_owner_uid: Res<WorldOwnerUID>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
) {
    use nod_krai_gi_data::quest::quest_config::QuestContent;

    for event in ev_reader.read() {
        // GadgetInteractEvent(entity_id, gadget_id, interact_id)
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: world_owner_uid.0,
            content_type: QuestContent::InteractGadget,
            param: event.1,
            param2: 0,
            param3: 0,
            add_progress: 1,
        });
    }
}

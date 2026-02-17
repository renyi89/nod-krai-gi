mod script_entity;
mod script_group_manager;
mod script_lib;
mod script_lib_handle;
mod script_load;
mod script_lua_vm;

use crate::script_entity::{despawn_group_entity, spawn_group_entity};
use crate::script_group_manager::{on_player_move, GroupLoadManager};
use crate::script_lib::{
    call_lua_trigger_action, call_lua_trigger_condition, BevyScriptLib, ScriptCommand,
};
use crate::script_load::{GroupLoadState, SceneGroupRuntime};
use crate::script_lua_vm::LuaRuntime;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use crossbeam_queue::SegQueue;
use nod_krai_gi_event::lua::{DespawnGroupEntityEvent, LuaTriggerEvent, SpawnGroupEntityEvent};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ScriptPlugin;

#[derive(Resource)]
pub struct ScriptCommandQueue(pub Arc<SegQueue<ScriptCommand>>);

#[derive(Default)]
pub struct SceneGroupRegistry {
    pub groups: HashMap<u32, GroupLoadState>,
}

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        let queue = Arc::new(SegQueue::new());

        let script_lib = BevyScriptLib {
            queue: queue.clone(),
        };

        let lua_vm = LuaRuntime::new(Arc::new(script_lib.clone()));

        app.insert_resource(script_lib)
            .insert_resource(lua_vm.clone())
            .insert_resource(ScriptCommandQueue(queue))
            .insert_non_send_resource(SceneGroupRegistry::default())
            .insert_resource(GroupLoadManager::default())
            .add_systems(Update, script_command_system)
            .add_systems(Update, lua_trigger_event_system)
            .add_systems(Update, on_player_move)
            .add_systems(Update, spawn_group_entity)
            .add_systems(Update, despawn_group_entity);
    }
}

fn script_command_system(
    mut registry: NonSendMut<SceneGroupRegistry>,
    queue: Res<ScriptCommandQueue>,
    lua_vm: Res<LuaRuntime>,
    mut spawn_group_entity_event: MessageWriter<SpawnGroupEntityEvent>,
    mut despawn_group_entity_event: MessageWriter<DespawnGroupEntityEvent>,
) {
    while let Some(cmd) = queue.0.pop() {
        match cmd {
            ScriptCommand::AddExtraGroupSuite {
                ctx,
                group_id,
                suite_id,
            } => {
                tracing::debug!("[Script] [AddExtraGroupSuite] context:{:#?}", ctx);

                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(&group_id) {
                    if !rt.active_suites.contains(&suite_id) {
                        rt.active_suites.push(suite_id);
                        rt.recompute_active_triggers();
                    }
                }
            }

            ScriptCommand::RemoveExtraGroupSuite {
                ctx,
                group_id,
                suite_id,
            } => {
                tracing::debug!("[Script] [RemoveExtraGroupSuite] context:{:#?}", ctx);

                if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(&group_id) {
                    rt.active_suites.retain(|s| *s != suite_id);
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

                    registry.groups.insert(group_id, GroupLoadState::Loading);

                    let runtime =
                        SceneGroupRuntime::new(scene_id, block_id, group_id, lua_vm.clone());

                    match runtime {
                        Some(rt) => {
                            registry.groups.insert(group_id, GroupLoadState::Loaded(rt));
                            tracing::debug!("[Extra] [LoadGroup] Group {} loaded", group_id);
                            spawn_group_entity_event.write(SpawnGroupEntityEvent {
                                scene_id,
                                block_id,
                                group_id,
                            });
                        }
                        None => {
                            registry.groups.insert(group_id, GroupLoadState::Failed);
                            tracing::debug!(
                                "[Extra] [LoadGroup] Group {} failed to load",
                                group_id
                            );
                        }
                    }
                }
            },

            ScriptCommand::UnloadGroup { group_id } => {
                tracing::debug!("[Extra] [UnloadGroup] context:{}", group_id);
                despawn_group_entity_event.write(DespawnGroupEntityEvent { group_id });
                registry.groups.remove(&group_id);
                tracing::debug!("[Extra] [UnloadGroup] Group {} unloaded", group_id);
            }
        }
    }
}

pub fn lua_trigger_event_system(
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

                        if let Some(cond) = &trig.condition {
                            if let Ok(ret) = call_lua_trigger_condition(cond, rt.context, event.evt)
                            {
                                if ret {
                                    if let Some(action) = &trig.action {
                                        let _ =
                                            call_lua_trigger_action(action, rt.context, event.evt);
                                    }
                                }
                            }
                        }
                    }
                }

                GroupLoadState::Unloaded => unreachable!(),
            },
        }
    }
}

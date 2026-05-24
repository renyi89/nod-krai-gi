use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::*;
use common::player_cache::cache_get_scene_level;
use nod_krai_gi_data::scene::group_entity_state_cache::get_group_entity_state_cache;
use nod_krai_gi_data::scene::{EventType, GadgetState, LuaEvt};
use nod_krai_gi_entity::common::{
    BlockId, ConfigId, EntityCounter, GroupId, ProtocolEntityID, ToBeRemovedMarker, Visible,
};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_entity::monster::spawn_monster_entity;
use nod_krai_gi_event::entity::EntityDisappearEvent;
use nod_krai_gi_event::lua::{
    DespawnGroupEntityEvent, DespawnSuiteEntitiesEvent, LuaTriggerEvent, RefreshGroupEntityEvent,
    SpawnGroupEntityEvent, SpawnSuiteEntitiesEvent,
};
use nod_krai_gi_event::scene::{WorldOwnerUID, WorldVersionConfig};

use crate::script_load::GroupLoadState;
use crate::SceneGroupRegistry;
use nod_krai_gi_proto::normal::scene_gadget_info::Content;
use nod_krai_gi_proto::normal::{GatherGadgetInfo, VisionType};
use nod_krai_gi_proto::server_only::VectorBin;

pub fn spawn_suite_entities(
    mut spawn_suite_events: MessageReader<SpawnSuiteEntitiesEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    world_owner_uid: Res<WorldOwnerUID>,
    world_version_config: Res<WorldVersionConfig>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
) {
    let scene_group_collection = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    for event in spawn_suite_events.read() {
        let Some(scene_group_template) = scene_group_collection.get(&event.group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        let show_level =
            cache_get_scene_level(world_owner_uid.0, scene_group_template.base_info.scene_id)
                .unwrap_or(1);

        let suite_index = event.suite_id.saturating_sub(1) as usize;
        let Some(suite) = scene_group_template.suites.get(suite_index) else {
            tracing::debug!(
                "[SpawnSuiteEntities] suite {} not found in group {}",
                event.suite_id,
                event.group_id
            );
            continue;
        };

        for monster in scene_group_template.monsters.iter() {
            if suite.monsters.contains(&monster.config_id) {
                let mut level = monster.level.unwrap_or(103) + 67;
                level += show_level - 1;
                let Some((entity_id, monster_entity, cur_hp, max_hp)) = spawn_monster_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: monster.pos.x,
                        y: monster.pos.y,
                        z: monster.pos.z,
                    },
                    VectorBin {
                        x: monster.rot.x,
                        y: monster.rot.y,
                        z: monster.rot.z,
                    },
                    monster.monster_id,
                    level,
                    monster.title_id.unwrap_or(0),
                    monster.special_name_id.unwrap_or(0),
                    monster.drop_tag.clone(),
                    monster.chest_drop_id.unwrap_or(0),
                ) else {
                    continue;
                };
                commands
                    .entity(monster_entity)
                    .insert(BlockId(event.block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(monster.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_monster_spawn(
                    world_owner_uid.0,
                    event.group_id,
                    monster.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: event.group_id,
                    event_type: EventType::EventAnyMonsterLive,
                    evt: LuaEvt {
                        param1: monster.config_id,
                        param2: monster.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });

                tracing::debug!(
                    "spawn group_id {} config_id {} suite {} entity_id {}",
                    event.group_id,
                    monster.config_id,
                    event.suite_id,
                    entity_id
                );
            }
        }

        for gadget in scene_group_template.gadgets.iter() {
            if suite.gadgets.contains(&gadget.config_id) {
                let gadget_id = gadget.gadget_id;
                let is_interactive = false;
                let Some((entity_id, gadget_entity, cur_hp, max_hp)) = spawn_gadget_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: gadget.pos.x,
                        y: gadget.pos.y,
                        z: gadget.pos.z,
                    },
                    VectorBin {
                        x: gadget.rot.x,
                        y: gadget.rot.y,
                        z: gadget.rot.z,
                    },
                    gadget_id,
                    gadget.level.unwrap_or(90),
                    gadget.is_enable_interact.unwrap_or(is_interactive),
                    None,
                    gadget.drop_tag.clone(),
                    gadget.chest_drop_id.unwrap_or(0),
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                ) else {
                    continue;
                };
                commands
                    .entity(gadget_entity)
                    .insert(BlockId(event.block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(gadget.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_gadget_spawn(
                    world_owner_uid.0,
                    event.group_id,
                    gadget.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: event.group_id,
                    event_type: EventType::EventGadgetCreate,
                    evt: LuaEvt {
                        param1: gadget.config_id,
                        param2: gadget.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });

                tracing::debug!(
                    "spawn group_id {} config_id {} suite {} entity_id {}",
                    event.group_id,
                    gadget.config_id,
                    event.suite_id,
                    entity_id
                );
            }
        }
    }
}

pub fn despawn_suite_entities(
    mut despawn_suite_events: MessageReader<DespawnSuiteEntitiesEvent>,
    mut commands: Commands,
    mut entities: Query<(Entity, &ProtocolEntityID, &GroupId, &ConfigId)>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    let scene_group_collection = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    for event in despawn_suite_events.read() {
        let Some(scene_group_template) = scene_group_collection.get(&event.group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        let suite_index = event.suite_id.saturating_sub(1) as usize;
        let Some(suite) = scene_group_template.suites.get(suite_index) else {
            continue;
        };

        let mut config_ids = suite.monsters.clone();
        config_ids.extend(suite.gadgets.clone());

        entities
            .iter_mut()
            .filter(|(_, _, group_id, config_id)| {
                group_id.0 == event.group_id && config_ids.contains(&config_id.0)
            })
            .for_each(|(entity, entity_id, _, config_id)| {
                disappear_events.write(EntityDisappearEvent(
                    entity_id.0,
                    VisionType::VisionMiss.into(),
                ));
                commands.entity(entity).insert(ToBeRemovedMarker);

                tracing::debug!(
                    "despawn group_id {} config_id {} suite {} entity_id {}",
                    event.group_id,
                    config_id.0,
                    event.suite_id,
                    entity_id.0
                );
            });
    }
}

pub fn spawn_group_entity(
    mut spawn_group_entity_event: MessageReader<SpawnGroupEntityEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    world_owner_uid: Res<WorldOwnerUID>,
    world_version_config: Res<WorldVersionConfig>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut registry: NonSendMut<SceneGroupRegistry>,
) {
    let scene_group_collection_clone = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    let gather_excel_config_collection_clone =
        std::sync::Arc::clone(nod_krai_gi_data::excel::gather_excel_config_collection::get());

    for event in spawn_group_entity_event.read() {
        // Handle suite refresh for quest RefreshGroupSuite exec
        if event.refresh_suite_id != 0 {
            if let Some(GroupLoadState::Loaded(rt)) = registry.groups.get_mut(&event.group_id) {
                rt.active_suites = vec![event.refresh_suite_id];
                rt.recompute_active_triggers();
                let initial_vars: std::collections::HashMap<String, i32> = rt
                    .data
                    .variables
                    .iter()
                    .map(|v| (v.name.clone(), v.value))
                    .collect();
                rt.variables = initial_vars;
            }
        }

        let Some(scene_group_template) = scene_group_collection_clone.get(&event.group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        let block_id = if event.block_id != 0 {
            event.block_id
        } else {
            scene_group_template.base_info.block_id
        };

        let show_level =
            cache_get_scene_level(world_owner_uid.0, scene_group_template.base_info.scene_id)
                .unwrap_or(1);

        let suite_index = scene_group_template.init_config.suite;
        if suite_index < 1 || suite_index > scene_group_template.suites.len() as u32 {
            continue;
        }
        let suite = &scene_group_template.suites[(suite_index - 1) as usize];

        for monster in scene_group_template.monsters.iter() {
            if suite.monsters.contains(&monster.config_id) {
                let mut level = monster.level.unwrap_or(103) + 67;
                level += show_level - 1;
                let Some((entity_id, monster_entity, cur_hp, max_hp)) = spawn_monster_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: monster.pos.x,
                        y: monster.pos.y,
                        z: monster.pos.z,
                    },
                    VectorBin {
                        x: monster.rot.x,
                        y: monster.rot.y,
                        z: monster.rot.z,
                    },
                    monster.monster_id,
                    level,
                    monster.title_id.unwrap_or(0),
                    monster.special_name_id.unwrap_or(0),
                    monster.drop_tag.clone(),
                    monster.chest_drop_id.unwrap_or(0),
                ) else {
                    continue;
                };
                commands
                    .entity(monster_entity)
                    .insert(BlockId(block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(monster.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_monster_spawn(
                    world_owner_uid.0,
                    event.group_id,
                    monster.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: event.group_id,
                    event_type: nod_krai_gi_data::scene::EventType::EventAnyMonsterLive,
                    evt: nod_krai_gi_data::scene::LuaEvt {
                        param1: monster.config_id,
                        param2: monster.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });
            }
        }
        for gadget in scene_group_template.gadgets.iter() {
            if suite.gadgets.contains(&gadget.config_id) {
                let mut gadget_id = gadget.gadget_id;
                let mut is_interactive = false;
                let mut gadget_content = None;
                if gadget_id == 70500000 && gadget.point_type.is_some() {
                    let Some(gather_config) = gather_excel_config_collection_clone
                        .get(&gadget.point_type.unwrap_or_default())
                    else {
                        tracing::debug!(
                            "gather config {} doesn't exist",
                            gadget.point_type.unwrap_or_default()
                        );
                        continue;
                    };
                    gadget_id = gather_config.gadget_id;
                    is_interactive = true;
                    gadget_content = Some(Content::GatherGadget(GatherGadgetInfo {
                        is_forbid_guest: gather_config.is_forbid_guest,
                        item_id: gather_config.item_id,
                    }));
                }
                let Some((entity_id, gadget_entity, cur_hp, max_hp)) = spawn_gadget_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: gadget.pos.x,
                        y: gadget.pos.y,
                        z: gadget.pos.z,
                    },
                    VectorBin {
                        x: gadget.rot.x,
                        y: gadget.rot.y,
                        z: gadget.rot.z,
                    },
                    gadget_id,
                    gadget.level.unwrap_or(90),
                    gadget.is_enable_interact.unwrap_or(is_interactive),
                    gadget_content,
                    gadget.drop_tag.clone(),
                    gadget.chest_drop_id.unwrap_or(0),
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                ) else {
                    continue;
                };
                commands
                    .entity(gadget_entity)
                    .insert(BlockId(block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(gadget.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_gadget_spawn(
                    world_owner_uid.0,
                    event.group_id,
                    gadget.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id: event.group_id,
                    event_type: EventType::EventGadgetCreate,
                    evt: LuaEvt {
                        param1: gadget.config_id,
                        param2: gadget.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });
            }
        }

        lua_trigger_events.write(LuaTriggerEvent {
            group_id: event.group_id,
            event_type: EventType::EventGroupLoad,
            evt: LuaEvt {
                param1: 0,
                param2: 0,
                param3: 0,
                source_eid: 0,
                target_eid: 0,
            },
        });
    }
}

pub fn despawn_group_entity(
    mut commands: Commands,
    mut despawn_group_entity_event: MessageReader<DespawnGroupEntityEvent>,
    mut entities: Query<(Entity, &ProtocolEntityID, &GroupId)>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for event in despawn_group_entity_event.read() {
        entities
            .iter_mut()
            .filter(|(_, _, group_id)| group_id.0 == event.group_id)
            .for_each(|(entity, entity_id, _)| {
                disappear_events.write(EntityDisappearEvent(
                    entity_id.0,
                    VisionType::VisionMiss.into(),
                ));
                commands.entity(entity).insert(ToBeRemovedMarker);
            });
    }
}

pub fn refresh_group_entity(
    mut refresh_group_events: MessageReader<RefreshGroupEntityEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    world_owner_uid: Res<WorldOwnerUID>,
    world_version_config: Res<WorldVersionConfig>,
    mut entities: Query<(Entity, &ProtocolEntityID, &GroupId)>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    let scene_group_collection = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    let gather_excel_config_collection =
        std::sync::Arc::clone(nod_krai_gi_data::excel::gather_excel_config_collection::get());

    for event in refresh_group_events.read() {
        let group_id = event.group_id;
        let suite_id = event.suite_id;
        let block_id = event.block_id;

        entities
            .iter_mut()
            .filter(|(_, _, g_id)| g_id.0 == group_id)
            .for_each(|(entity, entity_id, _)| {
                disappear_events.write(EntityDisappearEvent(
                    entity_id.0,
                    VisionType::VisionMiss.into(),
                ));
                commands.entity(entity).insert(ToBeRemovedMarker);
            });

        let Some(scene_group_template) = scene_group_collection.get(&group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        let show_level =
            cache_get_scene_level(world_owner_uid.0, scene_group_template.base_info.scene_id)
                .unwrap_or(1);

        let suite_index = suite_id.saturating_sub(1) as usize;
        let Some(suite) = scene_group_template.suites.get(suite_index) else {
            tracing::debug!(
                "[RefreshGroup] suite {} not found in group {}",
                suite_id,
                group_id
            );
            continue;
        };

        for monster in scene_group_template.monsters.iter() {
            if suite.monsters.contains(&monster.config_id) {
                let mut level = monster.level.unwrap_or(103) + 67;
                level += show_level - 1;
                let Some((entity_id, monster_entity, cur_hp, max_hp)) = spawn_monster_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: monster.pos.x,
                        y: monster.pos.y,
                        z: monster.pos.z,
                    },
                    VectorBin {
                        x: monster.rot.x,
                        y: monster.rot.y,
                        z: monster.rot.z,
                    },
                    monster.monster_id,
                    level,
                    monster.title_id.unwrap_or(0),
                    monster.special_name_id.unwrap_or(0),
                    monster.drop_tag.clone(),
                    monster.chest_drop_id.unwrap_or(0),
                ) else {
                    continue;
                };
                commands
                    .entity(monster_entity)
                    .insert(BlockId(block_id))
                    .insert(GroupId(group_id))
                    .insert(ConfigId(monster.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_monster_spawn(
                    world_owner_uid.0,
                    group_id,
                    monster.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id,
                    event_type: EventType::EventAnyMonsterLive,
                    evt: LuaEvt {
                        param1: monster.config_id,
                        param2: monster.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });
            }
        }

        for gadget in scene_group_template.gadgets.iter() {
            if suite.gadgets.contains(&gadget.config_id) {
                let mut gadget_id = gadget.gadget_id;
                let mut is_interactive = false;
                let mut gadget_content = None;
                if gadget_id == 70500000 && gadget.point_type.is_some() {
                    let Some(gather_config) =
                        gather_excel_config_collection.get(&gadget.point_type.unwrap_or_default())
                    else {
                        tracing::debug!(
                            "gather config {} doesn't exist",
                            gadget.point_type.unwrap_or_default()
                        );
                        continue;
                    };
                    gadget_id = gather_config.gadget_id;
                    is_interactive = true;
                    gadget_content = Some(Content::GatherGadget(GatherGadgetInfo {
                        is_forbid_guest: gather_config.is_forbid_guest,
                        item_id: gather_config.item_id,
                    }));
                }
                let Some((entity_id, gadget_entity, cur_hp, max_hp)) = spawn_gadget_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: gadget.pos.x,
                        y: gadget.pos.y,
                        z: gadget.pos.z,
                    },
                    VectorBin {
                        x: gadget.rot.x,
                        y: gadget.rot.y,
                        z: gadget.rot.z,
                    },
                    gadget_id,
                    gadget.level.unwrap_or(90),
                    gadget.is_enable_interact.unwrap_or(is_interactive),
                    gadget_content,
                    gadget.drop_tag.clone(),
                    gadget.chest_drop_id.unwrap_or(0),
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                ) else {
                    continue;
                };
                commands
                    .entity(gadget_entity)
                    .insert(BlockId(block_id))
                    .insert(GroupId(group_id))
                    .insert(ConfigId(gadget.config_id))
                    .insert(Visible);

                get_group_entity_state_cache().on_gadget_spawn(
                    world_owner_uid.0,
                    group_id,
                    gadget.config_id,
                    entity_id,
                    cur_hp,
                    max_hp,
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                );

                lua_trigger_events.write(LuaTriggerEvent {
                    group_id,
                    event_type: EventType::EventGadgetCreate,
                    evt: LuaEvt {
                        param1: gadget.config_id,
                        param2: gadget.config_id,
                        param3: 0,
                        source_eid: entity_id,
                        target_eid: entity_id,
                    },
                });
            }
        }
    }
}

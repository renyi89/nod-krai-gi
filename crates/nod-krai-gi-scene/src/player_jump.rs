use bevy_ecs::prelude::*;
use nod_krai_gi_entity::avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker};
use nod_krai_gi_entity::common::Visible;
use nod_krai_gi_entity::transform::Transform;
use nod_krai_gi_entity::EntityDisappearEvent;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::VisionType;
use nod_krai_gi_proto::server_only::VectorBin;
use std::sync::Arc;

pub fn player_jump(
    mut events: MessageReader<ScenePlayerJumpEvent>,
    mut players: ResMut<Players>,
    mut enter_events: MessageWriter<BeginEnterSceneEvent>,
    mut commands: Commands,
    player_avatar_entities: Query<(Entity, AvatarQueryReadOnly)>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for ScenePlayerJumpEvent(uid, scene_id, enter_reason, destination) in events.read() {
        let Some(player_info) = players.get_mut(*uid) else {
            continue;
        };
        let Some(ref mut player_scene_bin) = player_info.scene_bin else {
            continue;
        };
        let Some(ref player_avatar_bin) = player_info.avatar_bin else {
            continue;
        };

        let mut enter_type = nod_krai_gi_proto::normal::EnterType::EnterJump;
        if *scene_id == player_scene_bin.my_cur_scene_id {
            enter_type = nod_krai_gi_proto::normal::EnterType::EnterGoto;
        }

        let destination = VectorBin::from((destination.0, destination.1, destination.2));

        // Move directly in the same scene.
        if (*enter_reason == EnterReason::Gm
            || *enter_reason == EnterReason::Lua
            || *enter_reason == EnterReason::LuaSkipUi)
            && player_scene_bin.my_cur_scene_id == *scene_id
        {
            player_scene_bin.my_cur_scene_id = *scene_id;
            player_scene_bin.my_prev_pos = destination.into();

            for (avatar_entity, avatar_data) in player_avatar_entities.iter().filter(|(_, data)| {
                data.owner_player_uid.0 == *uid && data.guid.0 == player_avatar_bin.cur_avatar_guid
            }) {
                commands
                    .entity(avatar_entity)
                    .remove::<CurrentPlayerAvatarMarker>()
                    .remove::<Visible>();

                disappear_events.write(EntityDisappearEvent(
                    avatar_data.entity_id.0,
                    VisionType::VisionMiss.into(),
                ));

                commands
                    .entity(avatar_entity)
                    .insert(CurrentPlayerAvatarMarker)
                    .insert(Visible)
                    .insert(Transform {
                        position: player_scene_bin.my_prev_pos.unwrap_or_default().into(),
                        rotation: player_scene_bin.my_prev_rot.unwrap_or_default().into(),
                    });
            }
            continue;
        };

        player_scene_bin.my_cur_scene_id = *scene_id;
        player_scene_bin.my_prev_pos = destination.into();
        enter_events.write(BeginEnterSceneEvent {
            uid: *uid,
            scene_id: *scene_id,
            dungeon_id: 0,
            enter_type,
            enter_reason: *enter_reason,
            position: destination,
        });
    }
}

pub fn player_jump_by_point(
    mut events: MessageReader<ScenePlayerJumpByPointEvent>,
    mut players: ResMut<Players>,
    mut enter_events: MessageWriter<BeginEnterSceneEvent>,
) {
    let scene_point_config_collection_clone = Arc::clone(
        nod_krai_gi_data::scene::scene_point_config::SCENE_POINT_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );
    for ScenePlayerJumpByPointEvent(uid, scene_id, point_id) in events.read() {
        match scene_point_config_collection_clone.get(&scene_id) {
            None => {}
            Some(scene_config) => match scene_config.points.get(&point_id) {
                None => {}
                Some(point_config) => {
                    let mut tran_scene_id = point_config.tran_scene_id;

                    if tran_scene_id == 0 {
                        tran_scene_id = *scene_id;
                    }

                    let Some(player_info) = players.get_mut(*uid) else {
                        continue;
                    };

                    let mut enter_type = nod_krai_gi_proto::normal::EnterType::EnterJump;
                    let Some(ref player_scene_bin) = player_info.scene_bin else {
                        continue;
                    };
                    let mut enter_reason = EnterReason::TransPoint;
                    if !point_config.dungeon_ids.is_empty() {
                        enter_type = nod_krai_gi_proto::normal::EnterType::EnterDungeon;
                        enter_reason = EnterReason::DungeonEnter;
                    } else if tran_scene_id == player_scene_bin.my_cur_scene_id {
                        enter_type = nod_krai_gi_proto::normal::EnterType::EnterGoto;
                        enter_reason = EnterReason::TransPoint;
                    }

                    let destination = VectorBin::from((
                        point_config.tran_pos.x,
                        point_config.tran_pos.y,
                        point_config.tran_pos.z,
                    ));

                    if let Some(ref mut player_scene_bin) = player_info.scene_bin {
                        player_scene_bin.my_cur_scene_id = tran_scene_id;
                        player_scene_bin.my_prev_pos = destination.into();
                    }
                    enter_events.write(BeginEnterSceneEvent {
                        uid: *uid,
                        scene_id: tran_scene_id,
                        dungeon_id: 0,
                        enter_type,
                        enter_reason,
                        position: destination,
                    });
                }
            },
        }
    }
}

pub fn player_enter_dungeon(
    mut events: MessageReader<ScenePlayerEnterDungeonEvent>,
    mut players: ResMut<Players>,
    mut enter_events: MessageWriter<BeginEnterSceneEvent>,
) {
    let dungeon_excel_config_collection_clone =
        Arc::clone(nod_krai_gi_data::excel::dungeon_excel_config_collection::get());

    let scene_config_collection_clone = Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );

    for ScenePlayerEnterDungeonEvent(uid, dungeon_id) in events.read() {
        match dungeon_excel_config_collection_clone.get(&dungeon_id) {
            None => {}
            Some(dungeon_config) => {
                let tran_scene_id = dungeon_config.scene_id;

                let Some(scene_config) = scene_config_collection_clone.get(&tran_scene_id) else {
                    continue;
                };

                let Some(player_info) = players.get_mut(*uid) else {
                    continue;
                };

                let enter_type = nod_krai_gi_proto::normal::EnterType::EnterJump;
                let enter_reason = EnterReason::DungeonEnter;

                let destination = VectorBin::from((
                    scene_config.scene_config.born_pos.x,
                    scene_config.scene_config.born_pos.y,
                    scene_config.scene_config.born_pos.z,
                ));

                if let Some(ref mut player_scene_bin) = player_info.scene_bin {
                    player_scene_bin.my_cur_scene_id = tran_scene_id;
                    player_scene_bin.my_prev_pos = destination.into();
                }
                enter_events.write(BeginEnterSceneEvent {
                    uid: *uid,
                    scene_id: tran_scene_id,
                    dungeon_id: *dungeon_id,
                    enter_type,
                    enter_reason,
                    position: destination,
                });
            }
        }
    }
}

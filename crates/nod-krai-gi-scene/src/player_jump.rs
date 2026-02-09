use bevy_ecs::prelude::*;
use nod_krai_gi_entity::avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker};
use nod_krai_gi_entity::common::Visible;
use nod_krai_gi_entity::transform::{Transform, Vector3};
use nod_krai_gi_entity::EntityDisappearEvent;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::VisionType;
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

        let mut enter_type = nod_krai_gi_proto::EnterType::EnterJump;
        if *scene_id == player_info.world_position.scene_id {
            enter_type = nod_krai_gi_proto::EnterType::EnterGoto;
        }

        let destination = Vector3::from((destination.0, destination.1, destination.2));

        // Move directly in the same scene.
        if (*enter_reason == EnterReason::Gm
            || *enter_reason == EnterReason::Lua
            || *enter_reason == EnterReason::LuaSkipUi)
            && player_info.world_position.scene_id == *scene_id
        {
            player_info.world_position.scene_id = *scene_id;
            player_info.world_position.position = destination.into();

            for (avatar_entity, avatar_data) in player_avatar_entities.iter().filter(|(_, data)| {
                data.owner_player_uid.0 == *uid
                    && data.guid.0 == player_info.avatar_module.cur_avatar_guid
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
                        position: player_info.world_position.position.into(),
                        rotation: player_info.world_position.rotation.into(),
                    });
            }
            continue;
        };

        player_info.world_position.scene_id = *scene_id;
        player_info.world_position.position = destination.into();
        enter_events.write(BeginEnterSceneEvent {
            uid: *uid,
            scene_id: *scene_id,
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
                    let Some(player_info) = players.get_mut(*uid) else {
                        continue;
                    };

                    let mut enter_type = nod_krai_gi_proto::EnterType::EnterJump;
                    let mut enter_reason = EnterReason::TransPoint;
                    if !point_config.dungeon_ids.is_empty() {
                        enter_type = nod_krai_gi_proto::EnterType::EnterDungeon;
                        enter_reason = EnterReason::DungeonEnter;
                    } else if *scene_id == player_info.world_position.scene_id {
                        enter_type = nod_krai_gi_proto::EnterType::EnterGoto;
                        enter_reason = EnterReason::TransPoint;
                    }

                    let destination = Vector3::from((
                        point_config.tran_pos.x,
                        point_config.tran_pos.y,
                        point_config.tran_pos.z,
                    ));

                    player_info.world_position.scene_id = *scene_id;
                    player_info.world_position.position = destination.into();
                    enter_events.write(BeginEnterSceneEvent {
                        uid: *uid,
                        scene_id: *scene_id,
                        enter_type,
                        enter_reason,
                        position: destination,
                    });
                }
            },
        }
    }
}

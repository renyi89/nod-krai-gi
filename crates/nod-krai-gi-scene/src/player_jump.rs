use bevy_ecs::prelude::*;
use nod_krai_gi_entity::transform::Vector3;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_persistence::Players;
use std::sync::Arc;

pub fn player_jump(
    mut events: MessageReader<ScenePlayerJumpEvent>,
    mut players: ResMut<Players>,
    mut enter_events: MessageWriter<BeginEnterSceneEvent>,
) {
    for ScenePlayerJumpEvent(uid, scene_id, destination) in events.read() {
        let player = players.get_mut(*uid);
        player.world_position.scene_id = *scene_id;
        player.world_position.position = (*destination).into();

        enter_events.write(BeginEnterSceneEvent {
            uid: *uid,
            scene_id: *scene_id,
            enter_type: nod_krai_gi_proto::EnterType::EnterJump,
            position: *destination,
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
                    let player = players.get_mut(*uid);
                    let mut enter_type = nod_krai_gi_proto::EnterType::EnterJump;
                    if !point_config.dungeon_ids.is_empty() {
                        enter_type = nod_krai_gi_proto::EnterType::EnterDungeon;
                    } else if *scene_id == player.world_position.scene_id {
                        enter_type = nod_krai_gi_proto::EnterType::EnterGoto;
                    }

                    let destination = Vector3::from((
                        point_config.tran_pos.x,
                        point_config.tran_pos.y + 3.5,
                        point_config.tran_pos.z,
                    ));
                    player.world_position.scene_id = *scene_id;
                    player.world_position.position = destination.into();
                    enter_events.write(BeginEnterSceneEvent {
                        uid: *uid,
                        scene_id: *scene_id,
                        enter_type,
                        position: destination,
                    });
                }
            },
        }
    }
}

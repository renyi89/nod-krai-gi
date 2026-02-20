use crate::common::ScenePeerManager;
use bevy_ecs::prelude::*;
use nod_krai_gi_entity::avatar::{
    spawn_avatar_entity, AvatarQueryReadOnly, ControlPeer, CurrentTeam,
};
use nod_krai_gi_entity::{
    avatar::{CurrentPlayerAvatarMarker, IndexInSceneTeam},
    common::*,
    transform::Transform,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_persistence::Players;

pub fn player_join_team(
    mut events: MessageReader<PlayerJoinTeamEvent>,
    mut commands: Commands,
    players: Res<Players>,
    peer_mgr: Res<ScenePeerManager>,
    mut entity_counter: ResMut<EntityCounter>,
    mut scene_team_update_events: MessageWriter<SceneTeamUpdateEvent>,
    avatars: Query<(Entity, AvatarQueryReadOnly)>,
) {
    let is_empty = events.is_empty();

    for event in events.read() {
        let uid = event.player_uid;
        let Some(player_info) = players.get(uid) else {
            continue;
        };

        let Some(ref player_scene_bin) = player_info.scene_bin else {
            continue;
        };

        let Some(ref player_avatar_bin) = player_info.avatar_bin else {
            continue;
        };

        for (idx, to_spawn_guid) in event.avatar_guid_list.iter().enumerate() {
            match avatars.iter().find(|(_, avatar_data)| {
                avatar_data.guid.0 == *to_spawn_guid && avatar_data.owner_player_uid.0 == uid
            }) {
                Some((avatar_entity, _)) => {
                    commands
                        .entity(avatar_entity)
                        .insert(ControlPeer(peer_mgr.get_peer_id_by_uid(uid)))
                        .insert(IndexInSceneTeam(idx as u8))
                        .insert(CurrentTeam)
                        .insert(Transform {
                            position: player_scene_bin.my_cur_scene_pos.unwrap_or_default().into(),
                            rotation: player_scene_bin.my_cur_scene_rot.unwrap_or_default().into(),
                        });

                    if *to_spawn_guid == event.appear_avatar_guid {
                        commands
                            .entity(avatar_entity)
                            .insert(Visible)
                            .insert(CurrentPlayerAvatarMarker);
                    }
                }
                None => {
                    let Some(avatar_bin) = player_avatar_bin.avatar_map.get(&to_spawn_guid) else {
                        tracing::debug!("avatar guid {} doesn't exist", to_spawn_guid);
                        continue;
                    };

                    let Some((entity, _weapon_entity)) = spawn_avatar_entity(
                        &mut commands,
                        &mut entity_counter,
                        avatar_bin,
                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().into(),
                        player_scene_bin.my_cur_scene_rot.unwrap_or_default().into(),
                        uid,
                        peer_mgr.get_peer_id_by_uid(uid),
                        idx as u8,
                    ) else {
                        continue;
                    };

                    let mut avatar_entity = commands.entity(entity);
                    avatar_entity.insert(CurrentTeam);

                    if *to_spawn_guid == event.appear_avatar_guid {
                        avatar_entity
                            .insert(Visible)
                            .insert(CurrentPlayerAvatarMarker);
                    }
                }
            }
        }
    }

    if !is_empty {
        scene_team_update_events.write(SceneTeamUpdateEvent);
    }
}

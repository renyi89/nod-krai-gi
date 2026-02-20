use bevy_ecs::prelude::*;
use common::player_cache::cache_get_is_tp;
use nod_krai_gi_entity::common::{EntityById, Visible};
use nod_krai_gi_entity::{
    avatar::CurrentPlayerAvatarMarker,
    common::{OwnerPlayerUID, ProtocolEntityID},
    transform::Transform,
};
use nod_krai_gi_event::combat::*;
use nod_krai_gi_persistence::Players;

pub fn entity_movement(
    index: Res<EntityById>,
    mut events: MessageReader<EntityMoveEvent>,
    mut entities: Query<(&mut Transform, &ProtocolEntityID, Option<&OwnerPlayerUID>)>,
) {
    for EntityMoveEvent(originator_uid, info) in events.read() {
        let move_entity = match index.0.get(&info.entity_id) {
            Some(e) => *e,
            None => continue,
        };

        let Ok((mut transform, _, owner_uid)) = entities.get_mut(move_entity) else {
            tracing::debug!("entity with id {} not found", info.entity_id);
            continue;
        };

        if let Some(owner_uid) = owner_uid {
            if owner_uid.0 != *originator_uid {
                tracing::debug!(
                    "fail: entity owner uid mismatch! owner uid: {}, event originator uid: {}",
                    owner_uid.0,
                    originator_uid
                );
                continue;
            }
        }

        if cache_get_is_tp(*originator_uid).unwrap_or(true) {
            continue;
        }

        if let Some((Some(pos), Some(rot))) = info.motion_info.as_ref().map(|i| (i.pos, i.rot)) {
            transform.position = pos.into();
            transform.rotation = rot.into();
        }
    }
}

pub fn track_player_position(
    moved_player_avatars: Query<
        (&Transform, &OwnerPlayerUID),
        (
            With<Visible>,
            With<CurrentPlayerAvatarMarker>,
            Changed<Transform>,
        ),
    >,
    mut events: MessageWriter<PlayerMoveEvent>,
    mut players: ResMut<Players>,
) {
    for (transform, owner_uid) in moved_player_avatars.iter() {
        let Some(player_info) = players.get_mut(owner_uid.0) else {
            continue;
        };

        if let Some(ref mut player_scene_bin) = player_info.scene_bin {
            player_scene_bin.my_cur_scene_pos = transform.position.into();
            player_scene_bin.my_cur_scene_rot = transform.rotation.into();
            events.write(PlayerMoveEvent(
                owner_uid.0,
                player_scene_bin.my_cur_scene_id,
                (
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                ),
                false
            ));

            tracing::trace!(
                "player with uid {} player.scene_id {} moved to {}",
                owner_uid.0,
                player_scene_bin.my_cur_scene_id,
                transform
            );
        }
    }
}

use bevy_ecs::prelude::*;
use common::game_server_config::cache_get_is_tp;
use nod_krai_gi_entity::common::{EntityById, Visible};
use nod_krai_gi_entity::{
    avatar::CurrentPlayerAvatarMarker,
    common::{OwnerPlayerUID, ProtocolEntityID},
    transform::Transform,
};
use nod_krai_gi_event::combat::*;
use nod_krai_gi_persistence::Players;
use tracing::log::trace;
use tracing::{debug, instrument};

#[instrument(skip_all)]
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
            debug!("entity with id {} not found", info.entity_id);
            continue;
        };

        if let Some(owner_uid) = owner_uid {
            if owner_uid.0 != *originator_uid {
                debug!(
                    "fail: entity owner uid mismatch! owner uid: {}, event originator uid: {}",
                    owner_uid.0, originator_uid
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
    mut players: ResMut<Players>,
) {
    for (transform, owner_uid) in moved_player_avatars.iter() {
        let Some(player_info) = players.get_mut(owner_uid.0) else {
            continue;
        };

        player_info.world_position.position = transform.position.into();
        player_info.world_position.rotation = transform.rotation.into();

        trace!(
            "player with uid {} player.scene_id {} moved to {}",
            owner_uid.0,
            player_info.world_position.scene_id,
            transform
        );
    }
}

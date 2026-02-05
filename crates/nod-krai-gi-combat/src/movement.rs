use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, Visible};
use nod_krai_gi_entity::{
    avatar::CurrentPlayerAvatarMarker,
    common::{OwnerPlayerUID, ProtocolEntityID},
    transform::Transform,
};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::EntityMoveInfo;
use tracing::log::trace;
use tracing::{debug, instrument};

#[derive(Message)]
pub struct EntityMoveEvent(pub u32, pub EntityMoveInfo);

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
        let player = players.get_mut(owner_uid.0);
        if player.cache.is_tp {
            continue;
        }
        player.world_position.position = transform.position.into();
        player.world_position.rotation = transform.rotation.into();

        trace!(
            "player with uid {} player.scene_id {} moved to {}",
            owner_uid.0,
            player.world_position.scene_id,
            transform
        );
    }
}

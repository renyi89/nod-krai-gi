use std::collections::HashMap;

use bevy_ecs::prelude::*;
use nod_krai_gi_entity::{
    avatar::CurrentPlayerAvatarMarker,
    common::{OwnerPlayerUID, ProtocolEntityID, ToBeRemovedMarker},
};
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_proto::{
    retcode::Retcode, EvtAvatarLockChairReq, EvtAvatarLockChairRsp, EvtAvatarStandUpNotify,
};
use tracing::{debug, instrument};

#[derive(Resource, Default)]
pub struct ChairLockMap(HashMap<u64, (u32, u32)>);

#[instrument(skip_all)]
pub fn avatar_lock_chair(
    mut events: MessageReader<ClientMessageEvent>,
    out: Res<MessageOutput>,
    mut lock: ResMut<ChairLockMap>,
    active_entities: Query<
        (&OwnerPlayerUID, &ProtocolEntityID),
        (With<CurrentPlayerAvatarMarker>, Without<ToBeRemovedMarker>),
    >,
) {
    for message in events.read() {
        match message.message_name() {
            "EvtAvatarLockChairReq" => {
                if let Some(request) = message.decode::<EvtAvatarLockChairReq>() {
                    let uid = message.sender_uid();
                    let mut rsp = EvtAvatarLockChairRsp::default();

                    if let std::collections::hash_map::Entry::Vacant(e) =
                        lock.0.entry(request.chair_id)
                    {
                        let entity_id = active_entities
                            .iter()
                            .find(|(owner_uid, _)| owner_uid.0 == uid)
                            .unwrap()
                            .1;

                        e.insert((message.sender_uid(), entity_id.0));

                        rsp.chair_id = request.chair_id;
                        rsp.entity_id = entity_id.0;
                        rsp.direction = request.direction;
                        rsp.position = request.position;

                        debug!(
                            "chair id {} is now locked by player: {uid}",
                            request.chair_id
                        );
                    } else {
                        debug!("chair with id {} is already locked", request.chair_id);
                        rsp.retcode = Retcode::RetFail.into();
                    }


                    out.send(message.sender_uid(), "EvtAvatarLockChairRsp",rsp);
                }
            }
            "EvtAvatarStandUpNotify" => {
                if message.decode::<EvtAvatarStandUpNotify>().is_some() {
                    let uid = message.sender_uid();

                    lock.0.retain(|id, (locked_by, _)| {
                        if *locked_by == uid {
                            debug!("chair id {id} is now unlocked");
                            false
                        } else {
                            true
                        }
                    });
                }
            }
            &_ => {}
        }
    }
}

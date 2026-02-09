use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::{
    avatar_costume_excel_config_collection, avatar_flycloak_excel_config_collection,
    avatar_trace_effect_excel_config_collection,
};
use nod_krai_gi_entity::avatar::{AvatarAppearanceChange, AvatarAppearanceChangeEvent};
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::{player_information::PlayerInformation, Players};
use nod_krai_gi_proto::{
    retcode::Retcode, AvatarChangeCostumeReq, AvatarChangeCostumeRsp, AvatarChangeTraceEffectReq,
    AvatarChangeTraceEffectRsp, AvatarFlycloakChangeNotify, AvatarWearFlycloakReq,
    AvatarWearFlycloakRsp,
};
use tracing::{debug, warn, instrument};

#[instrument(skip_all)]
pub fn handle_appearance_change_request(
    mut events: MessageReader<ClientMessageEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut change_events: MessageWriter<AvatarAppearanceChangeEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "AvatarWearFlycloakReq" => {
                if let Some(request) = message.decode::<AvatarWearFlycloakReq>() {
                    let avatar_flycloak_excel_config_collection_clone =
                        std::sync::Arc::clone(avatar_flycloak_excel_config_collection::get());

                    if avatar_flycloak_excel_config_collection_clone
                        .contains_key(&request.flycloak_id)
                    {
                        let Some(player_info) = players.get_mut(message.sender_uid()) else {
                            continue;
                        };
                        let mut rsp = AvatarWearFlycloakRsp::default();

                        if let Some(notify) = wear_flycloak(player_info, request, &mut rsp) {
                            message_output.send_to_all("AvatarFlycloakChangeNotify", notify);
                        }

                        message_output.send(message.sender_uid(), "AvatarWearFlycloakRsp", rsp);
                    } else {
                        warn!("Flycloak for {} was not found", request.flycloak_id);
                    }
                }
            }
            "AvatarChangeCostumeReq" => {
                if let Some(request) = message.decode::<AvatarChangeCostumeReq>() {
                    let avatar_costume_excel_config_collection_clone =
                        std::sync::Arc::clone(avatar_costume_excel_config_collection::get());

                    if avatar_costume_excel_config_collection_clone
                        .contains_key(&request.costume_id)
                    {
                        let Some(player_info) = players.get_mut(message.sender_uid()) else {
                            continue;
                        };
                        let mut rsp = AvatarChangeCostumeRsp::default();

                        if let Some(change_event) = change_costume(player_info, request, &mut rsp) {
                            change_events.write(change_event);
                        }

                        message_output.send(message.sender_uid(), "AvatarChangeCostumeRsp", rsp);
                    } else {
                        warn!("costume for {} was not found", request.costume_id);
                    }
                }
            }
            "AvatarChangeTraceEffectReq" => {
                if let Some(request) = message.decode::<AvatarChangeTraceEffectReq>() {
                    let avatar_trace_effect_excel_config_collection_clone =
                        std::sync::Arc::clone(avatar_trace_effect_excel_config_collection::get());
                    if avatar_trace_effect_excel_config_collection_clone
                        .contains_key(&request.trace_effect_id)
                    {
                        let Some(player_info) = players.get_mut(message.sender_uid()) else {
                            continue;
                        };
                        let mut rsp = AvatarChangeTraceEffectRsp::default();

                        if let Some(change_event) = change_trace_effect(player_info, request, &mut rsp) {
                            change_events.write(change_event);
                        }

                        message_output.send(
                            message.sender_uid(),
                            "AvatarChangeTraceEffectRsp",
                            rsp,
                        );
                    } else {
                        warn!("trace_effect for {} was not found", request.trace_effect_id);
                    }
                }
            }
            &_ => {}
        }
    }
}

#[instrument(skip(player, response))]
fn wear_flycloak(
    player: &mut PlayerInformation,
    request: AvatarWearFlycloakReq,
    response: &mut AvatarWearFlycloakRsp,
) -> Option<AvatarFlycloakChangeNotify> {
    if !player
        .avatar_module
        .owned_flycloak_set
        .contains(&request.flycloak_id)
    {
        debug!("flycloak id {} is not owned", request.flycloak_id);
        response.retcode = Retcode::RetNotHasFlycloak.into();
        return None;
    }

    response.avatar_guid_list = vec![];
    for avatar_guid in request.avatar_guid_list {
        let Some(avatar) = player.avatar_module.avatar_map.get_mut(&avatar_guid) else {
            debug!("avatar guid {} doesn't exist", avatar_guid);
            response.retcode = Retcode::RetCanNotFindAvatar.into();
            return None;
        };

        response.avatar_guid_list.push(avatar_guid);

        avatar.wearing_flycloak_id = request.flycloak_id;
        debug!(
            "wear flycloak_id: {}, avatar_guid: {}",
            request.flycloak_id, avatar_guid
        );
    }

    response.flycloak_id = request.flycloak_id;

    Some(AvatarFlycloakChangeNotify {
        avatar_guid_list: response.avatar_guid_list.clone(),
        flycloak_id: request.flycloak_id,
    })
}

#[instrument(skip(player, response))]
fn change_costume(
    player: &mut PlayerInformation,
    request: AvatarChangeCostumeReq,
    response: &mut AvatarChangeCostumeRsp,
) -> Option<AvatarAppearanceChangeEvent> {
    response.retcode = Retcode::RetFail.into();

    let avatar_costume_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_costume_excel_config_collection::get());

    let config = (request.costume_id != 0)
        .then(|| avatar_costume_excel_config_collection_clone.get(&request.costume_id))
        .flatten();

    if request.costume_id != 0 && config.is_none() {
        debug!("costume_id {} config doesn't exist", request.costume_id);
        return None;
    };

    if !player
        .avatar_module
        .owned_costume_set
        .contains(&request.costume_id)
        && config.is_some()
    {
        debug!("costume is not unlocked, id: {}", request.costume_id);
        response.retcode = Retcode::RetNotHasCostume.into();
        return None;
    }

    let Some(avatar) = player
        .avatar_module
        .avatar_map
        .get_mut(&request.avatar_guid)
    else {
        debug!("avatar guid {} doesn't exist", request.avatar_guid);
        return None;
    };

    if let Some(config) = config {
        if config.character_id != avatar.avatar_id {
            debug!(
                "avatar costume mismatch, config: {}, requested: {}",
                config.character_id, avatar.avatar_id
            );
            response.retcode = Retcode::RetCostumeAvatarError.into();
            return None;
        }
    }

    response.avatar_guid = request.avatar_guid;
    response.costume_id = request.costume_id;
    response.retcode = Retcode::RetSucc.into();
    avatar.costume_id = request.costume_id;

    debug!(
        "change costume for avatar {} to {}",
        avatar.avatar_id, request.costume_id
    );

    Some(AvatarAppearanceChangeEvent {
        player_uid: player.uid,
        avatar_guid: request.avatar_guid,
        change: AvatarAppearanceChange::Costume(request.costume_id),
    })
}

#[instrument(skip(player, response))]
fn change_trace_effect(
    player: &mut PlayerInformation,
    request: AvatarChangeTraceEffectReq,
    response: &mut AvatarChangeTraceEffectRsp,
) -> Option<AvatarAppearanceChangeEvent> {
    response.retcode = Retcode::RetFail.into();

    let avatar_trace_effect_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_trace_effect_excel_config_collection::get());

    let config = (request.trace_effect_id != 0)
        .then(|| avatar_trace_effect_excel_config_collection_clone.get(&request.trace_effect_id))
        .flatten();

    if request.trace_effect_id != 0 && config.is_none() {
        debug!(
            "trace_effect_id {} config doesn't exist",
            request.trace_effect_id
        );
        return None;
    };

    if !player
        .avatar_module
        .owned_trace_effect_set
        .contains(&request.trace_effect_id)
        && config.is_some()
    {
        debug!(
            "trace effect is not unlocked, id: {}",
            request.trace_effect_id
        );
        response.retcode = Retcode::RetNotHasTraceEffect.into();
        return None;
    }

    let Some(avatar) = player
        .avatar_module
        .avatar_map
        .get_mut(&request.avatar_guid)
    else {
        debug!("avatar guid {} doesn't exist", request.avatar_guid);
        return None;
    };

    if let Some(config) = config {
        if config.avatar_id != avatar.avatar_id {
            debug!(
                "avatar trace effect mismatch, config: {}, requested: {}",
                config.avatar_id, avatar.avatar_id
            );
            response.retcode = Retcode::RetTraceEffectAvatarError.into();
            return None;
        }
    }

    response.avatar_guid = request.avatar_guid;
    response.trace_effect_id = request.trace_effect_id;
    response.retcode = Retcode::RetSucc.into();
    avatar.trace_effect_id = request.trace_effect_id;

    debug!(
        "change trace effect for avatar {} to {}",
        avatar.avatar_id, request.trace_effect_id
    );

    Some(AvatarAppearanceChangeEvent {
        player_uid: player.uid,
        avatar_guid: request.avatar_guid,
        change: AvatarAppearanceChange::TraceEffect(request.trace_effect_id),
    })
}

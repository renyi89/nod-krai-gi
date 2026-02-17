use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::common::{EquipType, ItemType};
use nod_krai_gi_data::excel::reliquary_excel_config_collection;
use nod_krai_gi_entity::avatar::AvatarEquipChangeEvent;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{TakeoffEquipReq, TakeoffEquipRsp, WearEquipReq, WearEquipRsp};
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::server_only::item_bin;
use tracing::{debug, instrument, warn};

#[instrument(skip_all)]
pub fn change_avatar_equip(
    mut events: MessageReader<ClientMessageEvent>,
    mut equip_change_events: MessageWriter<AvatarEquipChangeEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        match message.message_name() {
            "WearEquipReq" => {
                if let Some(request) = message.decode::<WearEquipReq>() {
                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };

                    let Some(ref mut player_avatar_bin) = player_info.avatar_bin else {
                        continue;
                    };

                    let Some(wear_avatar_bin) =
                        player_avatar_bin.avatar_map.get_mut(&request.avatar_guid)
                    else {
                        debug!("avatar with guid {} doesn't exist", request.avatar_guid);
                        continue;
                    };

                    let Some(ref mut player_item_bin) = player_info.item_bin else {
                        continue;
                    };

                    let Some(mut wear_item_bin) =
                        player_item_bin.get_item(&request.equip_guid).cloned()
                    else {
                        debug!("item with guid {} doesn't exist", request.equip_guid);
                        continue;
                    };

                    let Some(item_bin::Detail::Equip(ref _equip)) = wear_item_bin.detail else {
                        debug!("item with guid {} is not equip", request.equip_guid);
                        continue;
                    };

                    let wear_equip_type;

                    if wear_item_bin.item_type == ItemType::WEAPON as u32 {
                        wear_equip_type = EquipType::Weapon;
                    } else {
                        let reliquary_excel_config_collection_clone =
                            std::sync::Arc::clone(reliquary_excel_config_collection::get());

                        match reliquary_excel_config_collection_clone.get(&wear_item_bin.item_id) {
                            None => {
                                debug!("reliquary config {} doesn't exist", wear_item_bin.item_id);
                                continue;
                            }
                            Some(reliquary_config) => {
                                wear_equip_type = reliquary_config.equip_type
                            }
                        }
                    }

                    let replace_avatar_guid;

                    if wear_equip_type != EquipType::None {
                        if wear_item_bin.owner_guid == request.avatar_guid {
                            warn!(
                                "wear_item_bin.owner_guid == request.avatar_guid:{}",
                                request.avatar_guid
                            );
                            wear_item_bin.owner_guid = 0;
                            player_item_bin.add_item(wear_item_bin.guid, wear_item_bin.clone());
                            continue;
                        }

                        replace_avatar_guid = wear_item_bin.owner_guid;

                        wear_item_bin.owner_guid = request.avatar_guid;
                        let replace_item_bin = wear_avatar_bin
                            .equip_map
                            .insert(wear_equip_type as u32, wear_item_bin.clone());
                        player_item_bin.add_item(wear_item_bin.guid, wear_item_bin.clone());

                        if replace_avatar_guid == 0 {
                            match replace_item_bin {
                                None => {}
                                Some(mut replace_item_bin) => {
                                    replace_item_bin.owner_guid = 0;
                                    player_item_bin
                                        .add_item(replace_item_bin.guid, replace_item_bin.clone());
                                }
                            }
                        } else {
                            match player_avatar_bin.avatar_map.get_mut(&replace_avatar_guid) {
                                None => {
                                    debug!(
                                        "avatar with guid {} doesn't exist",
                                        replace_avatar_guid
                                    );
                                    match replace_item_bin {
                                        None => {}
                                        Some(mut replace_item_bin) => {
                                            replace_item_bin.owner_guid = 0;
                                            player_item_bin.add_item(
                                                replace_item_bin.guid,
                                                replace_item_bin.clone(),
                                            );
                                        }
                                    }
                                }
                                Some(other_avatar_bin) => match replace_item_bin {
                                    None => {
                                        other_avatar_bin
                                            .equip_map
                                            .remove(&(wear_equip_type as u32));
                                    }
                                    Some(mut replace_item_bin) => {
                                        replace_item_bin.owner_guid = replace_avatar_guid;
                                        other_avatar_bin.equip_map.insert(
                                            wear_equip_type as u32,
                                            replace_item_bin.clone(),
                                        );
                                        player_item_bin.add_item(
                                            replace_item_bin.guid,
                                            replace_item_bin.clone(),
                                        );
                                    }
                                },
                            }
                        }
                    } else {
                        continue;
                    }

                    equip_change_events.write(AvatarEquipChangeEvent {
                        player_uid: message.sender_uid(),
                        avatar_guid: request.avatar_guid,
                        equip_type: wear_equip_type,
                    });

                    if replace_avatar_guid != 0 {
                        equip_change_events.write(AvatarEquipChangeEvent {
                            player_uid: message.sender_uid(),
                            avatar_guid: replace_avatar_guid,
                            equip_type: wear_equip_type,
                        });
                    }

                    message_output.send(
                        message.sender_uid(),
                        "WearEquipRsp",
                        WearEquipRsp {
                            retcode: Retcode::RetSucc.into(),
                            avatar_guid: request.avatar_guid,
                            equip_guid: request.equip_guid,
                        },
                    );
                }
            }
            "TakeoffEquipReq" => {
                if let Some(request) = message.decode::<TakeoffEquipReq>() {
                    let take_off_equip_type = EquipType::from(request.slot);
                    if take_off_equip_type == EquipType::None {
                        continue;
                    }

                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };

                    let Some(ref mut player_avatar_bin) = player_info.avatar_bin else {
                        continue;
                    };

                    let Some(take_off_avatar_bin) =
                        player_avatar_bin.avatar_map.get_mut(&request.avatar_guid)
                    else {
                        debug!("avatar with guid {} doesn't exist", request.avatar_guid);
                        continue;
                    };

                    let Some(ref mut player_item_bin) = player_info.item_bin else {
                        continue;
                    };

                    let take_off_item_bin = take_off_avatar_bin
                        .equip_map
                        .remove(&(take_off_equip_type as u32));

                    match take_off_item_bin {
                        None => {}
                        Some(mut take_off_item_bin) => {
                            take_off_item_bin.owner_guid = 0;
                            player_item_bin.add_item(take_off_item_bin.guid, take_off_item_bin);
                        }
                    }

                    equip_change_events.write(AvatarEquipChangeEvent {
                        player_uid: message.sender_uid(),
                        avatar_guid: request.avatar_guid,
                        equip_type: take_off_equip_type,
                    });

                    message_output.send(
                        message.sender_uid(),
                        "TakeoffEquipRsp",
                        TakeoffEquipRsp {
                            retcode: Retcode::RetSucc.into(),
                            avatar_guid: request.avatar_guid,
                            slot: request.slot,
                        },
                    );
                }
            }
            &_ => {}
        }
    }
}

use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::common::{EquipType, ItemType};
use nod_krai_gi_data::excel::reliquary_excel_config_collection;
use nod_krai_gi_entity::avatar::AvatarEquipChangeEvent;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{WearEquipReq, WearEquipRsp};
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::server_only::item_bin;
use tracing::{debug, instrument};

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
                    let Some(ref item_bin) = player_info.item_bin else {
                        continue;
                    };
                    if !item_bin
                        .get_item(&request.equip_guid)
                        .map(|item| {
                            let Some(item_bin::Detail::Equip(ref _equip)) = item.detail else {
                                return false;
                            };
                            true
                        })
                        .unwrap_or(false)
                    {
                        debug!("equip with guid {} doesn't exist", request.equip_guid);
                        continue;
                    }

                    let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                        continue;
                    };

                    let Some(ref mut item_bin) = player_info.item_bin else {
                        continue;
                    };

                    let Some(avatar) = avatar_bin.avatar_map.get_mut(&request.avatar_guid) else {
                        debug!("avatar with guid {} doesn't exist", request.avatar_guid);
                        continue;
                    };

                    let Some(item_bin) = item_bin.get_item(&request.equip_guid) else {
                        debug!("avatar with guid {} doesn't exist", request.avatar_guid);
                        continue;
                    };

                    if item_bin.item_type == ItemType::WEAPON as u32 {
                        avatar
                            .equip_map
                            .insert(EquipType::Weapon as u32, item_bin.clone());
                    } else {
                        let reliquary_excel_config_collection_clone =
                            std::sync::Arc::clone(reliquary_excel_config_collection::get());

                        match reliquary_excel_config_collection_clone.get(&item_bin.item_id) {
                            None => {
                                debug!("reliquary excel {} doesn't exist", item_bin.item_id);
                                continue;
                            }
                            Some(reliquary_config) => {
                                if reliquary_config.equip_type != EquipType::None {
                                    avatar.equip_map.insert(
                                        reliquary_config.equip_type.clone() as u32,
                                        item_bin.clone(),
                                    );
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    equip_change_events.write(AvatarEquipChangeEvent {
                        player_uid: message.sender_uid(),
                        avatar_guid: request.avatar_guid,
                    });

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
            &_ => {}
        }
    }
}

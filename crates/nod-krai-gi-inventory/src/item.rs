use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::message::{MessageReader, MessageWriter};
use common::gm_util::ItemAction;
use nod_krai_gi_data::excel::common::ItemType;
use nod_krai_gi_data::excel::{
    material_excel_config_collection, reliquary_affix_excel_config_collection,
    reliquary_excel_config_collection, reliquary_main_prop_excel_config_collection,
    weapon_excel_config_collection,
};
use nod_krai_gi_event::command::{CommandItemEvent, ConsoleChatNotifyEvent};
use nod_krai_gi_event::inventory::StoreItemChangeEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{ItemAddHintNotify, ItemHint, StoreItemChangeNotify, StoreType};
use nod_krai_gi_proto::server_only::{
    equip_bin, item_bin, EquipBin, ItemBin, MaterialBin, ReliquaryBin,
};
use std::collections::HashMap;

pub fn item_command_handler(
    mut events: MessageReader<CommandItemEvent>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut store_item_change_events: MessageWriter<StoreItemChangeEvent>,
    mut players: ResMut<Players>,
) {
    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());
    let reliquary_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_excel_config_collection::get());
    let material_excel_config_collection_clone =
        std::sync::Arc::clone(material_excel_config_collection::get());

    let reliquary_main_prop_config_collection_clone =
        std::sync::Arc::clone(reliquary_main_prop_excel_config_collection::get());

    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());

    for CommandItemEvent(player_uid, action) in events.read() {
        let Some(player_info) = players.get_mut(*player_uid) else {
            continue;
        };

        let new_guid = player_info.next_guid();
        let mut change_map: HashMap<u64, u32> = HashMap::new();
        let Some(ref mut item_bin) = player_info.item_bin else {
            continue;
        };

        match action {
            ItemAction::Add {
                id,
                num,
                level,
                main_prop_id,
                append_prop_id_list,
            } => {
                let mut item_type = ItemType::NONE;
                if weapon_excel_config_collection_clone.contains_key(&id) {
                    item_type = ItemType::WEAPON;
                } else if reliquary_excel_config_collection_clone.contains_key(&id) {
                    item_type = ItemType::RELIQUARY;
                } else if material_excel_config_collection_clone.contains_key(&id) {
                    item_type = ItemType::MATERIAL;
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("unknown id:{}", id),
                    ));
                }

                match item_type {
                    ItemType::NONE => {}
                    ItemType::VIRTUAL => {}
                    ItemType::MATERIAL => {
                        let Some(material_data) = material_excel_config_collection_clone.get(&id)
                        else {
                            continue;
                        };
                        if material_data.use_on_gain {
                            continue;
                        }
                        let guid = item_bin.has_material(*id);
                        if guid.is_none() {
                            item_bin.add_item(
                                new_guid,
                                ItemBin {
                                    item_type: item_type as u32,
                                    item_id: *id,
                                    guid: new_guid,
                                    detail: Some(item_bin::Detail::Material(MaterialBin {
                                        count: num.unwrap_or(1),
                                        delete_bin: None,
                                    })),
                                },
                            );
                            change_map.insert(new_guid, num.unwrap_or(1));
                        } else {
                            let material_guid = guid.unwrap();
                            let Some(ref mut material_bin) = item_bin.get_mut_item(&material_guid)
                            else {
                                continue;
                            };
                            let Some(item_bin::Detail::Material(ref mut detail)) =
                                material_bin.detail
                            else {
                                continue;
                            };
                            detail.count += num.unwrap_or(1);
                            change_map.insert(material_guid, num.unwrap_or(1));
                        }
                        store_item_change_events
                            .write(StoreItemChangeEvent(*player_uid, change_map));
                    }
                    ItemType::RELIQUARY => {
                        let Some(_reliquary_data) =
                            reliquary_excel_config_collection_clone.get(&id)
                        else {
                            continue;
                        };
                        let Some(_reliquary_main_prop) =
                            reliquary_main_prop_config_collection_clone
                                .get(&main_prop_id.unwrap_or(0))
                        else {
                            gm_notify_events.write(ConsoleChatNotifyEvent(
                                *player_uid,
                                format!("main_prop_id not found:{}", main_prop_id.unwrap_or(0)),
                            ));
                            continue;
                        };
                        if !append_prop_id_list
                            .keys()
                            .all(|k| reliquary_affix_excel_config_collection_clone.contains_key(k))
                        {
                            gm_notify_events.write(ConsoleChatNotifyEvent(
                                *player_uid,
                                format!("append_prop_id not found"),
                            ));
                            continue;
                        }
                        item_bin.add_item(
                            new_guid,
                            ItemBin {
                                item_type: item_type as u32,
                                item_id: *id,
                                guid: new_guid,
                                detail: Some(item_bin::Detail::Equip(EquipBin {
                                    is_locked: false,
                                    detail: Some(equip_bin::Detail::Reliquary(ReliquaryBin {
                                        level: level.unwrap_or(1),
                                        exp: 0,
                                        main_prop_id: main_prop_id.unwrap_or(0),
                                        append_prop_id_list: expand_map_to_vec(append_prop_id_list),
                                    })),
                                })),
                            },
                        );
                        change_map.insert(new_guid, 1);
                        store_item_change_events
                            .write(StoreItemChangeEvent(*player_uid, change_map));
                    }
                    ItemType::WEAPON => {}
                    ItemType::DISPLAY => {}
                    ItemType::FURNITURE => {}
                }
            }
        }
    }
}

pub fn update_player_store(
    mut events: MessageReader<StoreItemChangeEvent>,
    message_output: Res<MessageOutput>,
    players: Res<Players>,
) {
    let mut merged: HashMap<u32, HashMap<u64, u32>> = HashMap::new();

    for StoreItemChangeEvent(uid, map) in events.read() {
        let entry = merged.entry(*uid).or_insert_with(HashMap::new);

        for (item_id, count) in map {
            *entry.entry(*item_id).or_insert(0) += *count;
        }
    }

    for (uid, final_map) in merged {
        let Some(player_info) = players.get(uid) else {
            continue;
        };
        let Some(ref item_bin) = player_info.item_bin else {
            continue;
        };

        message_output.send(
            uid,
            "StoreItemChangeNotify",
            StoreItemChangeNotify {
                store_type: StoreType::StorePack.into(),
                item_list: item_bin
                    .iter()
                    .filter(|(guid, _)| final_map.contains_key(guid))
                    .filter_map(|(_, item)| item.to_normal_proto())
                    .collect(),
                reason: 0,
            },
        );
        message_output.send(
            uid,
            "ItemAddHintNotify",
            ItemAddHintNotify {
                item_list: item_bin
                    .iter()
                    .filter(|(guid, _)| {
                        final_map.contains_key(guid)
                            && final_map.get(guid).copied().unwrap_or(0) > 0
                    })
                    .map(|(guid, item)| ItemHint {
                        count: final_map.get(guid).copied().unwrap_or(0),
                        item_id: item.item_id,
                        is_new: false,
                        guid: *guid,
                    })
                    .collect(),
                ..Default::default()
            },
        );
    }
}

fn expand_map_to_vec(map: &HashMap<u32, u32>) -> Vec<u32> {
    let mut v = Vec::new();
    for (k, count) in map {
        for _ in 0..u32::max(*count, 1) {
            v.push(*k);
        }
    }
    v
}

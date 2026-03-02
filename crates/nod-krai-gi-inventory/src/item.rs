use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_ecs::prelude::Commands;
use common::gm_util::ItemAction;
use nod_krai_gi_data::excel::common::ItemType;
use nod_krai_gi_data::excel::{
    material_excel_config_collection, reliquary_affix_excel_config_collection,
    reliquary_excel_config_collection, reliquary_main_prop_excel_config_collection,
    weapon_excel_config_collection, ReliquaryAffixExcelConfig,
};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{EntityCounter, Visible};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_event::command::{CommandItemEvent, ConsoleChatNotifyEvent};
use nod_krai_gi_event::inventory::{ItemAddEvent, ItemDropEvent, StoreItemChangeEvent};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    equip, item, scene_gadget_info, Equip, Item, ItemAddHintNotify, ItemHint, Material, Reliquary,
    StoreItemChangeNotify, StoreType, TrifleGadget, Weapon,
};
use nod_krai_gi_proto::server_only::{
    equip_bin, item_bin, EquipBin, ItemBin, MaterialBin, ReliquaryBin, VectorBin, WeaponBin,
};
use rand::prelude::IteratorRandom;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

pub fn item_command_handler(
    mut events: MessageReader<CommandItemEvent>,
    mut item_add_events: MessageWriter<ItemAddEvent>,
    mut item_drop_events: MessageWriter<ItemDropEvent>,
) {
    for CommandItemEvent(player_uid, action) in events.read() {
        match action {
            ItemAction::Add {
                id,
                num,
                level,
                refinement,
                main_prop_id,
                append_prop_id_list,
            } => {
                item_add_events.write(ItemAddEvent(
                    *player_uid,
                    vec![(
                        *id,
                        *num,
                        *level,
                        *refinement,
                        *main_prop_id,
                        append_prop_id_list.clone(),
                    )],
                ));
            }
            ItemAction::Drop { id, pos } => {
                item_drop_events.write(ItemDropEvent(*player_uid, *pos, vec![*id]));
            }
        }
    }
}

pub fn item_add_handler(
    mut events: MessageReader<ItemAddEvent>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut store_item_change_events: MessageWriter<StoreItemChangeEvent>,
    mut players: ResMut<Players>,
) {
    let mut rng = SmallRng::from_entropy();

    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());

    let reliquary_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_excel_config_collection::get());

    let material_excel_config_collection_clone =
        std::sync::Arc::clone(material_excel_config_collection::get());

    let reliquary_main_prop_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_main_prop_excel_config_collection::get());

    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());

    for ItemAddEvent(player_uid, item_list) in events.read() {
        let Some(player_info) = players.get_mut(*player_uid) else {
            continue;
        };

        let new_guid = player_info.next_guid();
        let mut change_map: HashMap<u64, u32> = HashMap::new();
        let Some(ref mut player_item_bin) = player_info.item_bin else {
            continue;
        };

        for (id, num, level, refinement, main_prop_id, append_prop_id_list) in item_list.iter() {
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
                    let Some(material_config) = material_excel_config_collection_clone.get(&id)
                    else {
                        continue;
                    };
                    if material_config.use_on_gain {
                        continue;
                    }
                    let guid = player_item_bin.has_material(*id);
                    if guid.is_none() {
                        player_item_bin.add_item(
                            new_guid,
                            ItemBin {
                                item_type: item_type as u32,
                                item_id: *id,
                                guid: new_guid,
                                owner_guid: 0,
                                detail: Some(item_bin::Detail::Material(MaterialBin {
                                    count: num.unwrap_or(1),
                                    delete_bin: None,
                                })),
                            },
                        );
                        change_map.insert(new_guid, num.unwrap_or(1));
                    } else {
                        let material_guid = guid.unwrap();
                        let Some(ref mut material_bin) =
                            player_item_bin.get_mut_item(&material_guid)
                        else {
                            continue;
                        };
                        let Some(item_bin::Detail::Material(ref mut detail)) = material_bin.detail
                        else {
                            continue;
                        };
                        detail.count += num.unwrap_or(1);
                        change_map.insert(material_guid, num.unwrap_or(1));
                    }
                }
                ItemType::RELIQUARY => {
                    let final_key = match main_prop_id {
                        Some(id) => id,
                        None => {
                            match reliquary_main_prop_excel_config_collection_clone
                                .keys()
                                .choose(&mut rng)
                            {
                                Some(k) => k,
                                None => continue,
                            }
                        }
                    };

                    let Some(reliquary_main_prop_config) =
                        reliquary_main_prop_excel_config_collection_clone.get(&final_key)
                    else {
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            format!("main_prop_id not found:{}", final_key),
                        ));
                        continue;
                    };

                    let main_prop = reliquary_main_prop_config.prop_type;

                    if !append_prop_id_list
                        .keys()
                        .all(|k| reliquary_affix_excel_config_collection_clone.contains_key(k))
                    {
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            "append_prop_id not found".to_string(),
                        ));
                        continue;
                    }

                    let mut append_prop_id_list = expand_map_to_vec(append_prop_id_list);
                    if append_prop_id_list.is_empty() {
                        append_prop_id_list = pick_four_affix_ids(
                            &reliquary_affix_excel_config_collection_clone,
                            main_prop,
                            &mut rng,
                        );
                    }

                    player_item_bin.add_item(
                        new_guid,
                        ItemBin {
                            item_type: item_type as u32,
                            item_id: *id,
                            guid: new_guid,
                            owner_guid: 0,
                            detail: Some(item_bin::Detail::Equip(EquipBin {
                                is_locked: false,
                                detail: Some(equip_bin::Detail::Reliquary(ReliquaryBin {
                                    main_prop_id: *final_key,
                                    append_prop_id_list,
                                    level: level.unwrap_or_default().clamp(0, 20) + 1,
                                    ..Default::default()
                                })),
                            })),
                        },
                    );
                    change_map.insert(new_guid, 1);
                }
                ItemType::WEAPON => {
                    let Some(weapon_config) = weapon_excel_config_collection_clone.get(&id) else {
                        continue;
                    };
                    let level = level.unwrap_or(1).clamp(1, 100);
                    let mut affix_map = HashMap::new();
                    weapon_config.skill_affix.iter().for_each(|affix| {
                        affix_map.insert(*affix, refinement.unwrap_or_default().clamp(1, 5) - 1);
                    });
                    player_item_bin.add_item(
                        new_guid,
                        ItemBin {
                            item_type: item_type as u32,
                            item_id: *id,
                            guid: new_guid,
                            owner_guid: 0,
                            detail: Some(item_bin::Detail::Equip(EquipBin {
                                is_locked: false,
                                detail: Some(equip_bin::Detail::Weapon(WeaponBin {
                                    level,
                                    promote_level: get_min_promote_level(level),
                                    affix_map,
                                    ..Default::default()
                                })),
                            })),
                        },
                    );
                    change_map.insert(new_guid, 1);
                }
                ItemType::DISPLAY => {}
                ItemType::FURNITURE => {}
            }
        }

        store_item_change_events.write(StoreItemChangeEvent(*player_uid, change_map));
    }
}

pub fn item_drop_handler(
    mut events: MessageReader<ItemDropEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut players: ResMut<Players>,
) {
    let mut rng = SmallRng::from_entropy();

    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());

    let reliquary_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_excel_config_collection::get());

    let material_excel_config_collection_clone =
        std::sync::Arc::clone(material_excel_config_collection::get());

    let reliquary_main_prop_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_main_prop_excel_config_collection::get());

    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());

    for ItemDropEvent(player_uid, pos, item_list) in events.read() {
        let Some(player_info) = players.get_mut(*player_uid) else {
            continue;
        };

        let new_guid = player_info.next_guid();

        for item_id in item_list.iter() {
            let mut item_type = ItemType::NONE;
            let mut gadget_id = 0;
            let mut item = None;
            if weapon_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::WEAPON;
            } else if reliquary_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::RELIQUARY;
            } else if material_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::MATERIAL;
            } else {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("unknown id:{}", item_id),
                ));
            }

            match item_type {
                ItemType::NONE => {}
                ItemType::VIRTUAL => {}
                ItemType::MATERIAL => {
                    let Some(material_config) = material_excel_config_collection_clone.get(item_id)
                    else {
                        continue;
                    };
                    if material_config.use_on_gain {
                        continue;
                    }
                    gadget_id = material_config.gadget_id;
                    item = Some(Item {
                        item_id: *item_id,
                        guid: new_guid,
                        detail: Some(item::Detail::Material(Material {
                            delete_info: None,
                            count: 1,
                        })),
                    });
                }
                ItemType::RELIQUARY => {
                    let Some(reliquary_config) =
                        reliquary_excel_config_collection_clone.get(item_id)
                    else {
                        continue;
                    };

                    let final_key = match reliquary_main_prop_excel_config_collection_clone
                        .keys()
                        .choose(&mut rng)
                    {
                        Some(k) => k,
                        None => continue,
                    };

                    let Some(reliquary_main_prop_config) =
                        reliquary_main_prop_excel_config_collection_clone.get(&final_key)
                    else {
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            format!("main_prop_id not found:{}", final_key),
                        ));
                        continue;
                    };

                    let main_prop = reliquary_main_prop_config.prop_type;

                    let append_prop_id_list = pick_four_affix_ids(
                        &reliquary_affix_excel_config_collection_clone,
                        main_prop,
                        &mut rng,
                    );

                    gadget_id = reliquary_config.gadget_id;

                    item = Some(Item {
                        item_id: *item_id,
                        guid: new_guid,
                        detail: Some(item::Detail::Equip(Equip {
                            is_locked: false,
                            detail: Some(equip::Detail::Reliquary(Reliquary {
                                main_prop_id: *final_key,
                                append_prop_id_list,
                                level: 1,
                                ..Default::default()
                            })),
                        })),
                    });
                }
                ItemType::WEAPON => {
                    let Some(weapon_config) = weapon_excel_config_collection_clone.get(item_id)
                    else {
                        continue;
                    };
                    let level = 1;
                    let mut affix_map = HashMap::new();
                    weapon_config.skill_affix.iter().for_each(|affix| {
                        affix_map.insert(*affix, 0);
                    });

                    gadget_id = weapon_config.gadget_id;

                    item = Some(Item {
                        item_id: *item_id,
                        guid: new_guid,
                        detail: Some(item::Detail::Equip(Equip {
                            is_locked: false,
                            detail: Some(equip::Detail::Weapon(Weapon {
                                level,
                                promote_level: get_min_promote_level(level),
                                affix_map,
                                ..Default::default()
                            })),
                        })),
                    });
                }
                ItemType::DISPLAY => {}
                ItemType::FURNITURE => {}
            }

            match item {
                None => {}
                Some(item) => {
                    let born_pos = match pos {
                        None => match &player_info.scene_bin {
                            None => (0.0, 0.0, 0.0),
                            Some(player_scene_bin) => (
                                player_scene_bin.my_cur_scene_pos.unwrap_or_default().x,
                                player_scene_bin.my_cur_scene_pos.unwrap_or_default().y + 0.5,
                                player_scene_bin.my_cur_scene_pos.unwrap_or_default().z,
                            ),
                        },
                        Some(pos) => *pos,
                    };
                    let born_pos = random_offset_vec3(born_pos, &mut rng);
                    let Some(gadget_entity) = spawn_gadget_entity(
                        &mut commands,
                        &mut entity_counter,
                        born_pos.into(),
                        VectorBin::default(),
                        gadget_id,
                        1,
                        0,
                        true,
                        Some(scene_gadget_info::Content::TrifleGadget(TrifleGadget {
                            item: Some(item),
                            ..Default::default()
                        })),
                    ) else {
                        continue;
                    };

                    commands.entity(gadget_entity).insert(Visible);
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
        let Some(ref player_item_bin) = player_info.item_bin else {
            continue;
        };

        message_output.send(
            uid,
            "StoreItemChangeNotify",
            StoreItemChangeNotify {
                store_type: StoreType::StorePack.into(),
                item_list: player_item_bin
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
                item_list: player_item_bin
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
        for _ in 0..(*count).clamp(0, 5) {
            v.push(*k);
        }
    }
    v
}

pub fn get_min_promote_level(level: u32) -> u32 {
    if level > 80 {
        6
    } else if level > 70 {
        5
    } else if level > 60 {
        4
    } else if level > 50 {
        3
    } else if level > 40 {
        2
    } else if level > 20 {
        1
    } else {
        0
    }
}

pub fn pick_four_affix_ids(
    map: &std::sync::Arc<HashMap<u32, ReliquaryAffixExcelConfig>>,
    main_prop_type: FightPropType,
    rng: &mut SmallRng,
) -> Vec<u32> {
    let mut buckets: HashMap<FightPropType, u32> = HashMap::new();
    let mut counts: HashMap<FightPropType, usize> = HashMap::new();

    for (id, cfg) in map.iter() {
        if cfg.prop_type == main_prop_type {
            continue;
        }

        let pt = cfg.prop_type;

        let counter = counts.entry(pt).or_insert(0);
        *counter += 1;

        if rng.gen_ratio(1, *counter as u32) {
            buckets.insert(pt, *id);
        }
    }

    buckets
        .values()
        .choose_multiple(rng, 4)
        .into_iter()
        .cloned()
        .collect()
}

pub fn random_offset_vec3((x, y, z): (f32, f32, f32), rng: &mut SmallRng) -> (f32, f32, f32) {
    let dx = rng.gen_range(-1.0..1.0);
    let dy = rng.gen_range(-1.0..1.0);
    let dz = rng.gen_range(-1.0..1.0);

    (x + dx, y + dy, z + dz)
}

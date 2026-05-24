use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_ecs::prelude::Commands;
use common::gm_util::{Command, ItemAction};
use nod_krai_gi_data::excel::common::ItemType;
use nod_krai_gi_data::excel::{
    material_excel_config_collection, reliquary_affix_excel_config_collection,
    reliquary_excel_config_collection, reliquary_level_excel_config_collection,
    reliquary_main_prop_excel_config_collection, weapon_excel_config_collection,
    weapon_level_excel_config_collection, ReliquaryAffixExcelConfig,
};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::quest::quest_config::{QuestCond, QuestContent};
use nod_krai_gi_entity::common::{EntityCounter, Visible};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_event::command::{ConsoleChatNotifyEvent, GmCommandEvent};
use nod_krai_gi_event::inventory::{ItemAddEvent, ItemDropEvent, StoreItemChangeEvent};
use nod_krai_gi_event::quest::{QuestAcceptCondEvent, QuestContentProgressEvent};
use nod_krai_gi_event::scene::{WorldOwnerUID, WorldVersionConfig};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    equip, item, scene_gadget_info, Equip, Item, ItemAddHintNotify, ItemHint, Material, Reliquary,
    StoreItemChangeNotify, StoreItemDelNotify, StoreType, TrifleGadgetInfo, Weapon,
};
use nod_krai_gi_proto::server_only::{
    equip_bin, item_bin, EquipBin, ItemBin, ReliquaryBin, VectorBin, WeaponBin,
};
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

pub fn item_command_handler(
    mut events: MessageReader<GmCommandEvent>,
    mut item_add_events: MessageWriter<ItemAddEvent>,
    mut item_drop_events: MessageWriter<ItemDropEvent>,
) {
    for GmCommandEvent(player_uid, command) in events.read() {
        let Command::Item(action) = command else {
            continue;
        };
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
            ItemAction::Drop { id } => {
                item_drop_events.write(ItemDropEvent(*player_uid, None, vec![(*id, 1)]));
            }
            _ => {}
        }
    }
}

pub fn item_add_handler(
    mut events: MessageReader<ItemAddEvent>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut store_item_change_events: MessageWriter<StoreItemChangeEvent>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
    mut quest_accept_events: MessageWriter<QuestAcceptCondEvent>,
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
        let mut change_map: HashMap<u64, i32> = HashMap::new();
        let Some(ref mut player_item_bin) = player_info.item_bin else {
            continue;
        };

        for (item_id, num, level, refinement, main_prop_id, append_prop_id_list) in item_list.iter()
        {
            let mut item_type = ItemType::NONE;
            if weapon_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::WEAPON;
            } else if reliquary_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::RELIQUARY;
            } else if material_excel_config_collection_clone.contains_key(item_id) {
                match material_excel_config_collection_clone.get(&item_id) {
                    None => {}
                    Some(material_config) => {
                        item_type = material_config.item_type;
                    }
                }
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
                    let Some(material_config) =
                        material_excel_config_collection_clone.get(&item_id)
                    else {
                        continue;
                    };
                    if material_config.use_on_gain {
                        continue;
                    }
                    let (material_guid, change_num) = player_item_bin.add_or_update_material(
                        new_guid,
                        *item_id,
                        item_type as u32,
                        num.unwrap_or(1) as i32,
                    );
                    change_map.insert(material_guid, change_num);
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

                    let Some(_reliquary_main_prop_config) =
                        reliquary_main_prop_excel_config_collection_clone.get(&final_key)
                    else {
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            format!("main_prop_id not found:{}", final_key),
                        ));
                        continue;
                    };

                    let Some(reliquary_config) =
                        reliquary_excel_config_collection_clone.get(item_id)
                    else {
                        continue;
                    };

                    let rank_level = reliquary_config.rank_level;
                    let level = level.unwrap_or_default().clamp(0, 20) + 1;

                    let mut total_exp: u32 = 0;
                    let level_config_map =
                        std::sync::Arc::clone(reliquary_level_excel_config_collection::get());
                    for lvl in 1..level {
                        let level_key = (rank_level << 8) + lvl;
                        if let Some(level_config) = level_config_map.get(&level_key) {
                            total_exp += level_config.exp;
                        }
                    }

                    let main_prop_type = reliquary_main_prop_excel_config_collection_clone
                        .get(final_key)
                        .map(|cfg| cfg.prop_type)
                        .unwrap_or_default();

                    let mut append_prop_id_list = expand_map_to_vec(append_prop_id_list);
                    if append_prop_id_list.is_empty() {
                        append_prop_id_list = pick_four_affix_ids(
                            &reliquary_affix_excel_config_collection_clone,
                            main_prop_type,
                            &mut rng,
                        );
                    }

                    player_item_bin.add_item(
                        new_guid,
                        ItemBin {
                            item_type: item_type as u32,
                            item_id: *item_id,
                            guid: new_guid,
                            owner_guid: 0,
                            detail: Some(item_bin::Detail::Equip(EquipBin {
                                is_locked: false,
                                detail: Some(equip_bin::Detail::Reliquary(ReliquaryBin {
                                    main_prop_id: *final_key,
                                    append_prop_id_list,
                                    level,
                                    exp: total_exp,
                                })),
                            })),
                        },
                    );
                    change_map.insert(new_guid, 1 as i32);
                }
                ItemType::WEAPON => {
                    let Some(weapon_config) = weapon_excel_config_collection_clone.get(&item_id)
                    else {
                        continue;
                    };
                    let level = level.unwrap_or(1).clamp(1, 100);
                    let rank_level = weapon_config.rank_level;

                    let mut total_exp: u32 = 0;
                    let level_config_map =
                        std::sync::Arc::clone(weapon_level_excel_config_collection::get());
                    for lvl in 1..level {
                        if let Some(level_config) = level_config_map.get(&lvl) {
                            let rank_index = (rank_level as usize).saturating_sub(1);
                            if rank_index < level_config.required_exps.len() {
                                total_exp += level_config.required_exps[rank_index];
                            }
                        }
                    }

                    let mut affix_map = HashMap::new();
                    weapon_config.skill_affix.iter().for_each(|affix| {
                        affix_map.insert(*affix, refinement.unwrap_or_default().clamp(1, 5) - 1);
                    });
                    player_item_bin.add_item(
                        new_guid,
                        ItemBin {
                            item_type: item_type as u32,
                            item_id: *item_id,
                            guid: new_guid,
                            owner_guid: 0,
                            detail: Some(item_bin::Detail::Equip(EquipBin {
                                is_locked: false,
                                detail: Some(equip_bin::Detail::Weapon(WeaponBin {
                                    level,
                                    promote_level: get_min_promote_level(level),
                                    affix_map,
                                    exp: total_exp,
                                })),
                            })),
                        },
                    );
                    change_map.insert(new_guid, 1 as i32);
                }
                ItemType::DISPLAY => {}
                ItemType::FURNITURE => {}
            }
        }

        store_item_change_events.write(StoreItemChangeEvent(*player_uid, change_map));

        for (item_id, _, _, _, _, _) in item_list.iter() {
            quest_content_events.write(QuestContentProgressEvent {
                player_uid: *player_uid,
                content_type: QuestContent::ObtainItem,
                param: *item_id,
                param2: 0,
                param3: 0,
                add_progress: 1,
            });
            quest_content_events.write(QuestContentProgressEvent {
                player_uid: *player_uid,
                content_type: QuestContent::ObtainVariousItem,
                param: *item_id,
                param2: 0,
                param3: 0,
                add_progress: 1,
            });
            quest_accept_events.write(QuestAcceptCondEvent {
                player_uid: *player_uid,
                cond_type: QuestCond::PackHaveItem,
                param: *item_id,
            });
        }
    }
}

pub fn item_drop_handler(
    mut events: MessageReader<ItemDropEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    players: Res<Players>,
    world_owner_uid: Res<WorldOwnerUID>,
    world_version_config: Res<WorldVersionConfig>,
) {
    let mut rng = SmallRng::from_entropy();

    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());

    let reliquary_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_excel_config_collection::get());

    let material_excel_config_collection_clone =
        std::sync::Arc::clone(material_excel_config_collection::get());

    for ItemDropEvent(player_uid, pos, item_list) in events.read() {
        let mut player_uid = *player_uid;
        if player_uid == 0 {
            player_uid = world_owner_uid.0;
        }

        let Some(player_info) = players.get(player_uid) else {
            continue;
        };

        let born_pos: (f32, f32, f32) = {
            match pos {
                None => match &player_info.scene_bin {
                    None => (0.0, 0.0, 0.0),
                    Some(player_scene_bin) => (
                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().x,
                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().y + 1.5,
                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().z,
                    ),
                },
                Some(pos) => *pos,
            }
        };

        for (item_id, count) in item_list.iter() {
            let mut item_type = ItemType::NONE;
            let mut gadget_id = 0;
            let mut item = None;
            if weapon_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::WEAPON;
            } else if reliquary_excel_config_collection_clone.contains_key(item_id) {
                item_type = ItemType::RELIQUARY;
            } else if material_excel_config_collection_clone.contains_key(item_id) {
                match material_excel_config_collection_clone.get(&item_id) {
                    None => {}
                    Some(material_config) => {
                        item_type = material_config.item_type;
                    }
                }
            } else {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    player_uid,
                    format!("unknown id:{}", item_id),
                ));
            }

            match item_type {
                ItemType::NONE => {}
                ItemType::VIRTUAL | ItemType::MATERIAL => {
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
                        guid: 0,
                        detail: Some(item::Detail::Material(Material {
                            delete_info: None,
                            count: *count,
                        })),
                    });
                }
                ItemType::RELIQUARY => {
                    let Some(reliquary_config) =
                        reliquary_excel_config_collection_clone.get(item_id)
                    else {
                        continue;
                    };

                    gadget_id = reliquary_config.gadget_id;

                    item = Some(Item {
                        item_id: *item_id,
                        guid: 0,
                        detail: Some(item::Detail::Equip(Equip {
                            is_locked: false,
                            detail: Some(equip::Detail::Reliquary(Reliquary {
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

                    gadget_id = weapon_config.gadget_id;

                    item = Some(Item {
                        item_id: *item_id,
                        guid: 0,
                        detail: Some(item::Detail::Equip(Equip {
                            is_locked: false,
                            detail: Some(equip::Detail::Weapon(Weapon {
                                level: 1,
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
                    let born_pos = random_offset_vec3(born_pos, &mut rng);
                    let Some(gadget_entity) = spawn_gadget_entity(
                        world_version_config.protocol_version.clone(),
                        &mut commands,
                        &mut entity_counter,
                        born_pos.into(),
                        VectorBin::default(),
                        gadget_id,
                        1,
                        true,
                        Some(scene_gadget_info::Content::TrifleGadget(TrifleGadgetInfo {
                            item: Some(item),
                            ..Default::default()
                        })),
                        None,
                        0,
                        0,
                    ) else {
                        continue;
                    };

                    commands.entity(gadget_entity.1).insert(Visible);
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
    let mut merged: HashMap<u32, HashMap<u64, i32>> = HashMap::new();

    for StoreItemChangeEvent(uid, map) in events.read() {
        let entry = merged.entry(*uid).or_insert_with(HashMap::new);

        for (guid, delta) in map {
            *entry.entry(*guid).or_insert(0) += *delta;
        }
    }

    for (uid, final_map) in merged {
        let Some(player_info) = players.get(uid) else {
            continue;
        };
        let Some(ref player_item_bin) = player_info.item_bin else {
            continue;
        };

        let change_guids: Vec<u64> = final_map
            .keys()
            .filter(|guid| player_item_bin.get_item(guid).is_some())
            .cloned()
            .collect();

        let del_guids: Vec<u64> = final_map
            .keys()
            .filter(|guid| player_item_bin.get_item(guid).is_none())
            .cloned()
            .collect();

        if !change_guids.is_empty() {
            message_output.send(
                uid,
                "StoreItemChangeNotify",
                StoreItemChangeNotify {
                    store_type: StoreType::StorePack.into(),
                    item_list: player_item_bin
                        .iter()
                        .filter(|(guid, _)| change_guids.contains(guid))
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
                            change_guids.contains(guid)
                                && final_map.get(guid).copied().unwrap_or(0) > 0
                        })
                        .map(|(guid, item)| ItemHint {
                            count: final_map.get(guid).copied().unwrap_or(0) as u32,
                            item_id: item.item_id,
                            is_new: false,
                            guid: *guid,
                        })
                        .collect(),
                    ..Default::default()
                },
            );
        }

        if !del_guids.is_empty() {
            message_output.send(
                uid,
                "StoreItemDelNotify",
                StoreItemDelNotify {
                    guid_list: del_guids,
                    store_type: StoreType::StorePack.into(),
                },
            );
        }
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

pub fn get_max_level_by_promote_level(promote_level: u32) -> u32 {
    if promote_level == 0 {
        return 20;
    } else if promote_level == 1 {
        return 40;
    } else if promote_level == 2 {
        return 50;
    } else if promote_level == 3 {
        return 60;
    } else if promote_level == 4 {
        return 70;
    } else if promote_level == 5 {
        return 80;
    } else {
        return 100;
    }
}

pub fn pick_four_affix_ids(
    map: &std::sync::Arc<HashMap<u32, ReliquaryAffixExcelConfig>>,
    main_prop_type: FightPropType,
    rng: &mut SmallRng,
) -> Vec<u32> {
    // 1. 按 prop_type 分桶
    let mut buckets: HashMap<FightPropType, Vec<u32>> = HashMap::new();

    for (id, cfg) in map.iter() {
        if cfg.prop_type != main_prop_type {
            buckets.entry(cfg.prop_type).or_default().push(*id);
        }
    }

    // 2. 每个 prop_type 等概率：先随机从每个桶中抽 1 个
    let mut picked_per_type: Vec<u32> = Vec::new();
    for (_pt, ids) in buckets.into_iter() {
        if let Some(chosen) = ids.choose(rng) {
            picked_per_type.push(*chosen);
        }
    }

    // 3. 从所有 prop_type 中随机抽 4 个
    picked_per_type.choose_multiple(rng, 4).cloned().collect()
}

pub fn pick_new_affix_id(
    main_prop_type: FightPropType,
    mut prop_type_list: Vec<FightPropType>,
) -> u32 {
    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());
    let mut rng = SmallRng::from_entropy();
    // 1. 收集所有 prop_type → ids
    let mut buckets: HashMap<FightPropType, Vec<u32>> = HashMap::new();
    for (id, cfg) in reliquary_affix_excel_config_collection_clone.iter() {
        if cfg.prop_type != main_prop_type {
            buckets.entry(cfg.prop_type).or_default().push(*id);
        }
    }

    // 2. 如果 prop_type_list < 4，则补齐
    if prop_type_list.len() < 4 {
        // 所有可选 prop_type
        let mut all_types: Vec<FightPropType> = buckets.keys().cloned().collect();

        // 去掉已有的
        all_types.retain(|pt| !prop_type_list.contains(pt));

        // 随机补齐到 5
        let need = 4 - prop_type_list.len();
        let extra = all_types
            .choose_multiple(&mut rng, need)
            .cloned()
            .collect::<Vec<_>>();

        prop_type_list.extend(extra);
    }

    // 3. 从最终的 prop_type_list 中随机选一个 prop_type
    let chosen_pt = prop_type_list.choose(&mut rng).cloned().unwrap();

    // 4. 在该 prop_type 的桶中随机选一个 affix id
    let ids = buckets.get(&chosen_pt).expect("prop_type must exist");
    *ids.choose(&mut rng).unwrap()
}

pub fn random_offset_vec3((x, y, z): (f32, f32, f32), rng: &mut SmallRng) -> (f32, f32, f32) {
    let dx = rng.gen_range(-0.5..0.5);
    let dy = rng.gen_range(-0.5..0.5);
    let dz = rng.gen_range(-0.5..0.5);

    (x + dx, y + dy, z + dz)
}

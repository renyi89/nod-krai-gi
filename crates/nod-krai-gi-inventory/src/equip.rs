use crate::item::pick_new_affix_id;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::common::{EquipType, ItemType};
use nod_krai_gi_data::excel::{
    reliquary_affix_excel_config_collection, reliquary_excel_config_collection,
    reliquary_level_excel_config_collection, reliquary_main_prop_excel_config_collection,
    weapon_excel_config_collection, weapon_level_excel_config_collection,
};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_event::avatar::AvatarEquipChangeEvent;
use nod_krai_gi_event::inventory::StoreItemChangeEvent;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    ReliquaryUpgradeReq, ReliquaryUpgradeRsp, TakeoffEquipReq, TakeoffEquipRsp, WeaponUpgradeReq,
    WeaponUpgradeRsp, WearEquipReq, WearEquipRsp,
};
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::server_only::{equip_bin, item_bin, PlayerItemCompBin};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument, warn};

#[instrument(skip_all)]
pub fn change_avatar_equip(
    mut events: MessageReader<ClientMessageEvent>,
    mut equip_change_events: MessageWriter<AvatarEquipChangeEvent>,
    mut store_item_change_events: MessageWriter<StoreItemChangeEvent>,
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
                            Some(reliquary_config) => wear_equip_type = reliquary_config.equip_type,
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
            "ReliquaryUpgradeReq" => {
                if let Some(request) = message.decode::<ReliquaryUpgradeReq>() {
                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };

                    let Some(ref mut player_item_bin) = player_info.item_bin else {
                        continue;
                    };

                    let Some(target_item) = player_item_bin
                        .get_item(&request.target_reliquary_guid)
                        .cloned()
                    else {
                        debug!(
                            "reliquary with guid {} doesn't exist",
                            request.target_reliquary_guid
                        );
                        continue;
                    };

                    let Some(item_bin::Detail::Equip(ref target_equip)) = target_item.detail else {
                        debug!("item is not equip");
                        continue;
                    };

                    let Some(equip_bin::Detail::Reliquary(ref target_reliquary)) =
                        target_equip.detail
                    else {
                        debug!("item is not reliquary");
                        continue;
                    };

                    let old_level = target_reliquary.level;
                    let old_append_prop_list = target_reliquary.append_prop_id_list.clone();

                    let reliquary_config_map =
                        std::sync::Arc::clone(reliquary_excel_config_collection::get());
                    let Some(reliquary_config) = reliquary_config_map.get(&target_item.item_id)
                    else {
                        debug!("reliquary config {} doesn't exist", target_item.item_id);
                        continue;
                    };

                    let rank_level = reliquary_config.rank_level;
                    let add_prop_levels = &reliquary_config.add_prop_levels;
                    let max_level = reliquary_config.max_level;

                    let mut total_exp: u32 = 0;
                    let mut change_map: HashMap<u64, i32> = HashMap::new();

                    for food_guid in &request.food_reliquary_guid_list {
                        if let Some(food_item) = player_item_bin.get_item(food_guid) {
                            if let Some(item_bin::Detail::Equip(ref food_equip)) = food_item.detail
                            {
                                if let Some(equip_bin::Detail::Reliquary(ref food_reliquary)) =
                                    food_equip.detail
                                {
                                    let food_config_map = std::sync::Arc::clone(
                                        reliquary_excel_config_collection::get(),
                                    );
                                    if let Some(food_config) =
                                        food_config_map.get(&food_item.item_id)
                                    {
                                        let food_total_exp = get_reliquary_total_exp(
                                            food_reliquary.level,
                                            food_reliquary.exp,
                                            food_config.rank_level,
                                        );
                                        let discounted_exp =
                                            (food_total_exp as f32 * 0.8).ceil() as u32;
                                        total_exp += food_config.base_conv_exp + discounted_exp;
                                    }
                                }
                            }
                        }
                        player_item_bin.remove_item(food_guid);
                        change_map.insert(*food_guid, -1);
                    }

                    for item_param in &request.item_param_list {
                        let mut ra = 1000000;
                        if item_param.item_id == 105002 {
                            ra = 2500;
                        }
                        if item_param.item_id == 105003 {
                            ra = 10000;
                        }
                        total_exp += item_param.count * ra;
                        let consumed =
                            consume_material(player_item_bin, item_param.item_id, item_param.count);
                        for (guid, delta) in consumed {
                            *change_map.entry(guid).or_insert(0) += delta;
                        }
                    }

                    let Some(ref mut item) =
                        player_item_bin.get_mut_item(&request.target_reliquary_guid)
                    else {
                        continue;
                    };

                    let Some(item_bin::Detail::Equip(ref mut equip)) = item.detail else {
                        continue;
                    };

                    let Some(equip_bin::Detail::Reliquary(ref mut reliquary)) = equip.detail else {
                        continue;
                    };

                    let level_config_map =
                        std::sync::Arc::clone(reliquary_level_excel_config_collection::get());

                    debug!("reliquary.exp += {}", total_exp);
                    reliquary.exp += total_exp;

                    let mut prop_type_set: HashSet<FightPropType> = HashSet::new();

                    while reliquary.level < max_level {
                        let level_key = (rank_level << 8) + reliquary.level;
                        let Some(level_config) = level_config_map.get(&level_key) else {
                            break;
                        };

                        if reliquary.exp < level_config.exp {
                            break;
                        }

                        reliquary.exp -= level_config.exp;

                        if add_prop_levels.contains(&(reliquary.level + 1)) {
                            let main_prop_type = get_main_prop_type(reliquary.main_prop_id);
                            prop_type_set.insert(main_prop_type);
                            let append_prop_type_list: Vec<FightPropType> = reliquary
                                .append_prop_id_list
                                .iter()
                                .map(|append_prop_id| get_append_prop_type(*append_prop_id))
                                .collect();

                            let new_affix_id =
                                pick_new_affix_id(main_prop_type, append_prop_type_list);
                            reliquary.append_prop_id_list.push(new_affix_id);
                        }

                        reliquary.level += 1;
                    }

                    if reliquary.level >= max_level {
                        reliquary.exp = 0;
                    }

                    change_map.insert(request.target_reliquary_guid, 0);

                    message_output.send(
                        message.sender_uid(),
                        "ReliquaryUpgradeRsp",
                        ReliquaryUpgradeRsp {
                            retcode: Retcode::RetSucc.into(),
                            target_reliquary_guid: request.target_reliquary_guid,
                            old_level,
                            cur_level: reliquary.level,
                            power_up_rate: 0,
                            old_append_prop_list,
                            cur_append_prop_list: reliquary.append_prop_id_list.clone(),
                        },
                    );

                    store_item_change_events
                        .write(StoreItemChangeEvent(message.sender_uid(), change_map));

                    if target_item.owner_guid != 0 {
                        let Some(ref mut player_avatar_bin) = player_info.avatar_bin else {
                            continue;
                        };

                        let Some(avatar_bin) = player_avatar_bin
                            .avatar_map
                            .get_mut(&target_item.owner_guid)
                        else {
                            debug!("avatar with guid {} doesn't exist", target_item.owner_guid);
                            continue;
                        };

                        let reliquary_excel_config_collection_clone =
                            std::sync::Arc::clone(reliquary_excel_config_collection::get());

                        let equip_type = match reliquary_excel_config_collection_clone
                            .get(&target_item.item_id)
                        {
                            Some(config) => config.equip_type,
                            None => continue,
                        };

                        if equip_type != EquipType::None {
                            avatar_bin.equip_map.insert(equip_type as u32, item.clone());
                            equip_change_events.write(AvatarEquipChangeEvent {
                                player_uid: message.sender_uid(),
                                avatar_guid: target_item.owner_guid,
                                equip_type,
                            });
                        }
                    }
                }
            }
            "WeaponUpgradeReq" => {
                if let Some(request) = message.decode::<WeaponUpgradeReq>() {
                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };

                    let Some(ref mut player_item_bin) = player_info.item_bin else {
                        continue;
                    };

                    let Some(target_item) = player_item_bin
                        .get_item(&request.target_weapon_guid)
                        .cloned()
                    else {
                        debug!(
                            "weapon with guid {} doesn't exist",
                            request.target_weapon_guid
                        );
                        continue;
                    };

                    let Some(item_bin::Detail::Equip(ref target_equip)) = target_item.detail else {
                        debug!("item is not equip");
                        continue;
                    };

                    let Some(equip_bin::Detail::Weapon(ref target_weapon)) = target_equip.detail
                    else {
                        debug!("item is not weapon");
                        continue;
                    };

                    let old_level = target_weapon.level;

                    let weapon_config_map =
                        std::sync::Arc::clone(weapon_excel_config_collection::get());
                    let Some(target_weapon_config) = weapon_config_map.get(&target_item.item_id)
                    else {
                        debug!("weapon config {} doesn't exist", target_item.item_id);
                        continue;
                    };

                    let rank_level = target_weapon_config.rank_level;

                    let mut total_exp: u32 = 0;
                    let mut change_map: HashMap<u64, i32> = HashMap::new();

                    for food_guid in &request.food_weapon_guid_list {
                        if let Some(food_item) = player_item_bin.get_item(food_guid) {
                            if let Some(item_bin::Detail::Equip(ref food_equip)) = food_item.detail
                            {
                                if let Some(equip_bin::Detail::Weapon(ref food_weapon)) =
                                    food_equip.detail
                                {
                                    if let Some(food_config) =
                                        weapon_config_map.get(&food_item.item_id)
                                    {
                                        let food_total_exp = get_weapon_total_exp(
                                            food_config.weapon_base_exp,
                                            food_weapon.level,
                                            food_weapon.exp,
                                            food_config.rank_level,
                                        );
                                        total_exp += food_total_exp;
                                    }
                                }
                            }
                        }
                        player_item_bin.remove_item(food_guid);
                        change_map.insert(*food_guid, -1);
                    }

                    for item_param in &request.item_param_list {
                        let mut ra = 1000000;
                        if item_param.item_id == 104011 {
                            ra = 400;
                        }
                        if item_param.item_id == 104012 {
                            ra = 2000;
                        }
                        if item_param.item_id == 104013 {
                            ra = 10000;
                        }
                        total_exp += item_param.count * ra;
                        let consumed =
                            consume_material(player_item_bin, item_param.item_id, item_param.count);
                        for (guid, delta) in consumed {
                            *change_map.entry(guid).or_insert(0) += delta;
                        }
                    }

                    let Some(ref mut item) =
                        player_item_bin.get_mut_item(&request.target_weapon_guid)
                    else {
                        continue;
                    };

                    let Some(item_bin::Detail::Equip(ref mut equip)) = item.detail else {
                        continue;
                    };

                    let Some(equip_bin::Detail::Weapon(ref mut weapon)) = equip.detail else {
                        continue;
                    };

                    debug!("weapon.exp += {}", total_exp);
                    weapon.exp += total_exp;
                    let max_level =
                        crate::item::get_max_level_by_promote_level(weapon.promote_level);

                    while weapon.level < max_level {
                        let exp_needed = get_weapon_exp_for_level(weapon.level, rank_level);
                        if exp_needed == 0 || weapon.exp < exp_needed {
                            break;
                        }
                        weapon.exp -= exp_needed;
                        weapon.level += 1;
                    }

                    if weapon.level >= max_level {
                        weapon.exp = 0;
                    }

                    change_map.insert(request.target_weapon_guid, 0);

                    message_output.send(
                        message.sender_uid(),
                        "WeaponUpgradeRsp",
                        WeaponUpgradeRsp {
                            retcode: Retcode::RetSucc.into(),
                            target_weapon_guid: request.target_weapon_guid,
                            old_level,
                            cur_level: weapon.level,
                            item_param_list: vec![],
                        },
                    );

                    store_item_change_events
                        .write(StoreItemChangeEvent(message.sender_uid(), change_map));

                    if target_item.owner_guid != 0 {
                        let Some(ref mut player_avatar_bin) = player_info.avatar_bin else {
                            continue;
                        };

                        let Some(avatar_bin) = player_avatar_bin
                            .avatar_map
                            .get_mut(&target_item.owner_guid)
                        else {
                            debug!("avatar with guid {} doesn't exist", target_item.owner_guid);
                            continue;
                        };

                        avatar_bin
                            .equip_map
                            .insert(EquipType::Weapon as u32, item.clone());
                        equip_change_events.write(AvatarEquipChangeEvent {
                            player_uid: message.sender_uid(),
                            avatar_guid: target_item.owner_guid,
                            equip_type: EquipType::Weapon,
                        });
                    }
                }
            }
            &_ => {}
        }
    }
}

fn consume_material(
    item_bin: &mut PlayerItemCompBin,
    item_id: u32,
    count: u32,
) -> HashMap<u64, i32> {
    let mut change_map: HashMap<u64, i32> = HashMap::new();
    let mut remaining = count;
    let mut to_remove = Vec::new();
    let mut partial_guid = None;

    for (&guid, item) in item_bin.iter() {
        if remaining == 0 {
            break;
        }
        if item.item_id != item_id {
            continue;
        }
        if let Some(item_bin::Detail::Material(ref mat)) = item.detail {
            if mat.count <= remaining {
                remaining -= mat.count;
                to_remove.push(guid);
                change_map.insert(guid, -(mat.count as i32));
            } else {
                partial_guid = Some((guid, mat.count - remaining));
                change_map.insert(guid, -(remaining as i32));
                remaining = 0;
            }
        }
    }

    if let Some((guid, new_count)) = partial_guid {
        if let Some(ref mut mat_item) = item_bin.get_mut_item(&guid) {
            if let Some(item_bin::Detail::Material(ref mut m)) = mat_item.detail {
                m.count = new_count;
            }
        }
    }

    for guid in to_remove {
        item_bin.remove_item(&guid);
    }

    change_map
}

fn get_main_prop_type(main_prop_id: u32) -> FightPropType {
    let reliquary_main_prop_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_main_prop_excel_config_collection::get());
    reliquary_main_prop_excel_config_collection_clone
        .get(&main_prop_id)
        .map(|cfg| cfg.prop_type)
        .unwrap_or_default()
}

fn get_append_prop_type(append_prop_id: u32) -> FightPropType {
    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());
    reliquary_affix_excel_config_collection_clone
        .get(&append_prop_id)
        .map(|cfg| cfg.prop_type)
        .unwrap_or_default()
}

fn get_weapon_exp_for_level(level: u32, rank_level: u32) -> u32 {
    let level_config_map = std::sync::Arc::clone(weapon_level_excel_config_collection::get());
    if let Some(level_config) = level_config_map.get(&level) {
        let rank_index = (rank_level as usize).saturating_sub(1);
        if rank_index < level_config.required_exps.len() {
            return level_config.required_exps[rank_index];
        }
    }
    0
}

fn get_weapon_total_exp(
    weapon_base_exp: u32,
    level: u32,
    current_exp: u32,
    rank_level: u32,
) -> u32 {
    let mut total_exp = weapon_base_exp * level;
    total_exp += current_exp;

    let level_config_map = std::sync::Arc::clone(weapon_level_excel_config_collection::get());
    for lvl in 1..level {
        if let Some(level_config) = level_config_map.get(&lvl) {
            let rank_index = (rank_level as usize).saturating_sub(1);
            if rank_index < level_config.required_exps.len() {
                total_exp += level_config.required_exps[rank_index];
            }
        }
    }

    total_exp
}

fn get_reliquary_total_exp(level: u32, current_exp: u32, rank_level: u32) -> u32 {
    let mut total_exp = current_exp;

    let level_config_map = std::sync::Arc::clone(reliquary_level_excel_config_collection::get());
    for lvl in 1..level {
        let level_key = (rank_level << 8) + lvl;
        if let Some(level_config) = level_config_map.get(&level_key) {
            total_exp += level_config.exp;
        }
    }

    total_exp
}

use bevy_ecs::prelude::*;
use common::gm_util::{AvatarAction, Command};
use nod_krai_gi_data::excel::avatar_excel_config_collection;
use nod_krai_gi_event::command::*;
use nod_krai_gi_persistence::Players;

use crate::util::add_avatar_and_weapon;

pub fn avatar_command_handler(
    mut events: MessageReader<GmCommandEvent>,
    mut players: ResMut<Players>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
) {
    for GmCommandEvent(player_uid, command) in events.read() {
        let Command::Avatar(action) = command else {
            continue;
        };
        match action {
            AvatarAction::Add { id } => {
                let Some(avatar_config) = avatar_excel_config_collection::get().get(id) else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("avatar {} not found", id),
                    ));
                    continue;
                };

                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };

                let already_has = player_info
                    .avatar_bin
                    .as_ref()
                    .map(|ab| ab.avatar_map.values().any(|a| a.avatar_id == *id))
                    .unwrap_or(false);

                if already_has {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("avatar {} already exists", id),
                    ));
                    continue;
                }

                add_avatar_and_weapon(player_info, avatar_config);

                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("added avatar {}", id),
                ));
            }
            AvatarAction::Remove { id } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let guid_to_remove = avatar_bin
                    .avatar_map
                    .iter()
                    .find(|(_, av)| av.avatar_id == *id)
                    .map(|(guid, _)| *guid);

                if let Some(guid) = guid_to_remove {
                    avatar_bin.avatar_map.remove(&guid);

                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("removed avatar {}", id),
                    ));
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("avatar {} not found", id),
                    ));
                }
            }
            AvatarAction::Level { level } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    avatar.level = *level;

                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set current avatar level to {}", level),
                    ));
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        "no current avatar".to_string(),
                    ));
                }
            }
            AvatarAction::Break { break_level } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    avatar.promote_level = *break_level;
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set current avatar promote_level to {}", break_level),
                    ));
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        "no current avatar".to_string(),
                    ));
                }
            }
            AvatarAction::AddTalent { talent_id } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    if let Some(depot) = avatar.depot_map.get_mut(&avatar.skill_depot_id) {
                        depot.talent_id_list.push(*talent_id);
                    }

                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("added talent {} to current avatar", talent_id),
                    ));
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        "no current avatar".to_string(),
                    ));
                }
            }
            AvatarAction::Skill { skill_id, level } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    let _old_level = avatar
                        .depot_map
                        .get(&avatar.skill_depot_id)
                        .and_then(|d| d.skill_level_map.get(skill_id))
                        .copied()
                        .unwrap_or(0);

                    if let Some(depot) = avatar.depot_map.get_mut(&avatar.skill_depot_id) {
                        depot.skill_level_map.insert(*skill_id, *level);
                    }

                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set skill {} to level {}", skill_id, level),
                    ));
                } else {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        "no current avatar".to_string(),
                    ));
                }
            }
            AvatarAction::Rename { id, name: _ } => {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("rename avatar {} (notify only)", id),
                ));
            }
            AvatarAction::Elem { element_type } => {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("element type {} (notify only)", element_type),
                ));
            }
            AvatarAction::FightProp { key, value } => {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("set prop {} value {} to current avatar", key, value),
                ));
            }
        }
    }
}

pub fn buff_command_handler(
    mut events: MessageReader<GmCommandEvent>,
    mut players: ResMut<Players>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
) {
    for GmCommandEvent(player_uid, command) in events.read() {
        let Command::Buff(action) = command else {
            continue;
        };
        match action {
            common::gm_util::BuffAction::Add { id, level } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    let _buff_level = level.unwrap_or(1);
                    avatar
                        .buff_list
                        .push(nod_krai_gi_proto::server_only::AvatarBuffBin {
                            buff_id: *id,
                            buff_type: 0,
                            ..Default::default()
                        });
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("added buff {} level {}", id, level.unwrap_or(1)),
                    ));
                }
            }
            common::gm_util::BuffAction::Clear => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get_mut(&cur_avatar_guid) {
                    avatar.buff_list.clear();
                    avatar.buff_map.clear();
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        "cleared all buffs".to_string(),
                    ));
                }
            }
            common::gm_util::BuffAction::List => {
                let Some(player_info) = players.get(*player_uid) else {
                    continue;
                };
                let Some(ref avatar_bin) = player_info.avatar_bin else {
                    continue;
                };

                let cur_avatar_guid = avatar_bin.cur_avatar_guid;
                if let Some(avatar) = avatar_bin.avatar_map.get(&cur_avatar_guid) {
                    let buff_info: Vec<String> = avatar
                        .buff_list
                        .iter()
                        .map(|b| format!("buff_id:{}", b.buff_id))
                        .chain(
                            avatar
                                .buff_map
                                .iter()
                                .map(|(id, b)| format!("buff_id:{} type:{}", id, b.buff_type)),
                        )
                        .collect();
                    let msg = if buff_info.is_empty() {
                        "no active buffs".to_string()
                    } else {
                        buff_info.join("\n")
                    };
                    gm_notify_events.write(ConsoleChatNotifyEvent(*player_uid, msg));
                }
            }
        }
    }
}

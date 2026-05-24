use bevy_ecs::prelude::*;
use common::gm_util::{Command, WeaponAction};
use nod_krai_gi_data::excel::common::EquipType;
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::inventory::StoreItemChangeEvent;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::server_only::{equip_bin, item_bin};
use std::collections::HashMap;
use tracing::instrument;

#[instrument(skip_all)]
pub fn weapon_command_handler(
    mut events: MessageReader<GmCommandEvent>,
    mut players: ResMut<Players>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut store_item_change_events: MessageWriter<StoreItemChangeEvent>,
) {
    for GmCommandEvent(player_uid, command) in events.read() {
        let Command::Weapon(action) = command else {
            continue;
        };
        let Some(player_info) = players.get_mut(*player_uid) else {
            continue;
        };
        let Some(ref mut avatar_bin) = player_info.avatar_bin else {
            continue;
        };

        let cur_avatar_guid = avatar_bin.cur_avatar_guid;
        let Some(avatar) = avatar_bin.avatar_map.get(&cur_avatar_guid) else {
            gm_notify_events.write(ConsoleChatNotifyEvent(
                *player_uid,
                "no current avatar".to_string(),
            ));
            continue;
        };

        let weapon_equip = avatar.equip_map.get(&(EquipType::Weapon as u32)).cloned();
        let weapon_guid = weapon_equip.as_ref().map(|w| w.guid);

        let Some(weapon_guid) = weapon_guid else {
            gm_notify_events.write(ConsoleChatNotifyEvent(
                *player_uid,
                "current avatar has no weapon".to_string(),
            ));
            continue;
        };

        let weapon_item = player_info
            .item_bin
            .as_ref()
            .and_then(|ib| ib.pack_store.as_ref())
            .and_then(|ps| ps.item_map.get(&weapon_guid))
            .cloned();

        let Some(mut weapon_item) = weapon_item else {
            gm_notify_events.write(ConsoleChatNotifyEvent(
                *player_uid,
                "weapon item not found".to_string(),
            ));
            continue;
        };

        let mut changed = false;
        match action {
            WeaponAction::Level { level } => {
                if let Some(detail) = weapon_item.detail.as_mut() {
                    if let item_bin::Detail::Equip(equip) = detail {
                        if let Some(equip_detail) = equip.detail.as_mut() {
                            if let equip_bin::Detail::Weapon(ref mut weapon) = equip_detail {
                                weapon.level = *level;
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set weapon level to {}", level),
                    ));
                }
            }
            WeaponAction::Break { break_level } => {
                if let Some(detail) = weapon_item.detail.as_mut() {
                    if let item_bin::Detail::Equip(equip) = detail {
                        if let Some(equip_detail) = equip.detail.as_mut() {
                            if let equip_bin::Detail::Weapon(ref mut weapon) = equip_detail {
                                weapon.promote_level = *break_level;
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set weapon promote_level to {}", break_level),
                    ));
                }
            }
            WeaponAction::Promote { promote_level } => {
                if let Some(detail) = weapon_item.detail.as_mut() {
                    if let item_bin::Detail::Equip(equip) = detail {
                        if let Some(equip_detail) = equip.detail.as_mut() {
                            if let equip_bin::Detail::Weapon(ref mut weapon) = equip_detail {
                                weapon.affix_map.clear();
                                weapon.affix_map.insert(0, *promote_level);
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    gm_notify_events.write(ConsoleChatNotifyEvent(
                        *player_uid,
                        format!("set weapon refinement to {}", promote_level),
                    ));
                }
            }
        }

        if changed {
            if let Some(ref mut item_bin) = player_info.item_bin {
                if let Some(ref mut pack_store) = item_bin.pack_store {
                    pack_store.item_map.insert(weapon_guid, weapon_item);
                }
            }

            store_item_change_events.write(StoreItemChangeEvent(
                *player_uid,
                HashMap::from([(weapon_guid, 0)]),
            ));
        }
    }
}

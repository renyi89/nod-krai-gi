use std::collections::HashMap;

use nod_krai_gi_avatar::util::add_avatar_and_weapon;
use nod_krai_gi_data::excel::common::ItemType;
use nod_krai_gi_data::excel::{
    avatar_costume_excel_config_collection, avatar_excel_config_collection,
    avatar_flycloak_excel_config_collection, avatar_trace_effect_excel_config_collection,
    weapon_excel_config_collection, AvatarUseType,
};
use nod_krai_gi_proto::server_only::*;

pub fn create_default_player_information(uid: u32, nick_name: String) -> PlayerDataBin {
    const DEFAULT_TEAM: [u32; 1] = [10000046];
    const DEFAULT_LEVEL: u32 = 60;

    let avatar_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_excel_config_collection::get());

    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());

    let avatar_flycloak_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_flycloak_excel_config_collection::get());

    let avatar_costume_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_costume_excel_config_collection::get());

    let avatar_trace_effect_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_trace_effect_excel_config_collection::get());

    let mut gacha_map = HashMap::new();
    gacha_map.insert(16, GachaBin::default());
    gacha_map.insert(1, GachaBin::default());
    gacha_map.insert(803, GachaBin::default());

    let mut player = PlayerDataBin {
        uid,
        guid_counter: 0,
        basic_bin: Some(PlayerBasicCompBin {
            level: DEFAULT_LEVEL,
            exp: 0,
            nickname: nick_name,
            is_game_time_locked: false,
            ..Default::default()
        }),
        avatar_bin: Some(PlayerAvatarCompBin {
            choose_avatar_guid: 0,
            cur_team_id: 1,
            cur_avatar_guid: 0,
            cur_avatar_guid_list: vec![],
            avatar_map: HashMap::new(),
            team_map: HashMap::new(),
            owned_flycloak_list: avatar_flycloak_excel_config_collection_clone
                .keys()
                .cloned()
                .collect(),
            owned_costume_list: avatar_costume_excel_config_collection_clone
                .keys()
                .cloned()
                .collect(),
            owned_trace_effect_list: avatar_trace_effect_excel_config_collection_clone
                .keys()
                .cloned()
                .collect(),
            ..Default::default()
        }),
        item_bin: Some(PlayerItemCompBin {
            pack_store: Some(ItemStoreBin {
                item_map: HashMap::new(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        scene_bin: Some(PlayerSceneCompBin {
            cur_scene_owner_uid: uid,
            my_prev_scene_id: 3,
            my_cur_scene_id: 3,
            my_cur_scene_pos: Some((2336.789, 249.98996, -751.3081).into()),
            my_cur_scene_rot: Some((0.0, 0.0, 0.0).into()),
            ..Default::default()
        }),
        quest_bin: Some(PlayerQuestCompBin {
            quest_bin: Some(PlayerQuestBin {
                quest_map: HashMap::new(),
            }),
            parent_quest_bin: Some(PlayerParentQuestBin {
                parent_quest_map: HashMap::new(),
            }),
            ..Default::default()
        }),
        gacha_bin: Some(PlayerGachaCompBin { gacha_map }),
        ..Default::default()
    };

    avatar_excel_config_collection_clone
        .values()
        .filter(|avatar| {
            return if avatar.id < 10000002
                || avatar.id >= 11000000
                || (avatar.id <= 10000910 && avatar.id >= 10000900)
                || avatar.id == 10000075
            {
                false
            } else if avatar.initial_weapon == 10009 {
                false
            } else {
                avatar.use_type == Some(AvatarUseType::Formal)
            };
        })
        .for_each(|avatar| add_avatar_and_weapon(&mut player, avatar));

    // Get avatar guids first to avoid borrow conflict
    let avatar_guids: Vec<u64> = DEFAULT_TEAM
        .iter()
        .filter_map(|id| {
            player
                .avatar_bin
                .as_ref()
                .unwrap()
                .avatar_map
                .iter()
                .find(|(_, av)| av.avatar_id == *id)
                .map(|(guid, _)| *guid)
        })
        .collect();

    // Now create the team map
    player.avatar_bin.as_mut().unwrap().team_map.insert(
        1,
        AvatarTeamBin {
            avatar_guid_list: avatar_guids,
            team_name: String::new(),
            ..Default::default()
        },
    );

    // Get team avatar list first
    let team_avatar_list = player
        .avatar_bin
        .as_ref()
        .unwrap()
        .team_map
        .get(&1)
        .unwrap()
        .avatar_guid_list
        .clone();

    // Update cur_avatar_guid_list
    player.avatar_bin.as_mut().unwrap().cur_avatar_guid_list = team_avatar_list;

    // Add bunch of weapons to inventory
    weapon_excel_config_collection_clone
        .values()
        .for_each(|weapon| {
            let guid = player.next_guid();
            player.item_bin.as_mut().unwrap().add_item(
                guid,
                ItemBin {
                    item_type: ItemType::WEAPON as u32,
                    item_id: weapon.id,
                    guid,
                    owner_guid: 0,
                    detail: Some(item_bin::Detail::Equip(EquipBin {
                        is_locked: false,
                        detail: Some(equip_bin::Detail::Weapon(WeaponBin {
                            level: 90,
                            exp: 0,
                            promote_level: 6,
                            affix_map: HashMap::with_capacity(0),
                        })),
                    })),
                },
            );
        });

    // Add default materials
    for (item_id, count) in [(223, 10u32), (224, 10u32), (220026, 1u32)] {
        let guid = player.next_guid();
        player.item_bin.as_mut().unwrap().add_item(
            guid,
            ItemBin {
                item_type: ItemType::MATERIAL as u32,
                item_id,
                guid,
                owner_guid: 0,
                detail: Some(item_bin::Detail::Material(MaterialBin {
                    count,
                    ..Default::default()
                })),
            },
        );
    }

    player
}

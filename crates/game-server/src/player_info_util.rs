use std::collections::HashMap;

use common::time_util;
use nod_krai_gi_data::config::{process_inherent_proud_skills, process_talent_ids};
use nod_krai_gi_data::excel::{
    avatar_costume_excel_config_collection, avatar_excel_config_collection,
    avatar_flycloak_excel_config_collection, avatar_skill_depot_excel_config_collection,
    avatar_skill_excel_config_collection, avatar_talent_excel_config_collection,
    avatar_trace_effect_excel_config_collection, proud_skill_excel_config_collection,
    weapon_excel_config_collection, AvatarExcelConfig, AvatarUseType,
};

use nod_krai_gi_proto::server_only::*;

pub fn create_default_player_information(uid: u32, nick_name: String) -> PlayerDataBin {
    const DEFAULT_TEAM: [u32; 1] = [10000106];
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
            my_cur_scene_id: 3,
            my_prev_pos: Some((2336.789, 249.98996, -751.3081).into()),
            my_prev_rot: Some((0.0, 0.0, 0.0).into()),
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
        ..Default::default()
    };

    avatar_excel_config_collection_clone
        .values()
        .filter(|avatar| avatar.use_type == AvatarUseType::Formal)
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
                    item_type: 0,
                    item_id: weapon.id,
                    guid,
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
    player
}

fn add_avatar_and_weapon(player: &mut PlayerDataBin, avatar: &AvatarExcelConfig) {
    const CHOOSE_AVATAR_ID: u32 = 10000007;
    const DEFAULT_AVATAR_LEVEL: u32 = 100;
    const DEFAULT_AVATAR_PROMOTE_LEVEL: u32 = 6;
    const DEFAULT_CORE_PROUD_SKILL_LEVEL: u32 = 6;
    const DEFAULT_WEAPON_LEVEL: u32 = 100;
    const DEFAULT_WEAPON_PROMOTE_LEVEL: u32 = 6;
    const DEFAULT_FLYCLOAK_ID: u32 = 140001;

    let avatar_guid = player.next_guid();
    let weapon_guid = player.next_guid();

    let mut skill_level_map = HashMap::new();
    let mut inherent_proud_skill_list = Vec::new();

    let avatar_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_excel_config_collection::get());

    let avatar_skill_depot_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_skill_depot_excel_config_collection::get());

    let avatar_skill_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_skill_excel_config_collection::get());

    let avatar_talent_collection_clone =
        std::sync::Arc::clone(avatar_talent_excel_config_collection::get());

    let proud_skill_collection_clone =
        std::sync::Arc::clone(proud_skill_excel_config_collection::get());

    if let Some(skill_depot) =
        avatar_skill_depot_excel_config_collection_clone.get(&avatar.skill_depot_id)
    {
        skill_depot
            .skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_depot
            .sub_skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_level_map.insert(skill_depot.energy_skill, 14);

        skill_depot
            .inherent_proud_skill_opens
            .iter()
            .filter(|s| {
                s.proud_skill_group_id != 0 && s.need_avatar_promote_level <= DEFAULT_AVATAR_LEVEL
            })
            .for_each(|s| inherent_proud_skill_list.push(s.proud_skill_group_id * 100 + 1));
    }

    let avatar = avatar_excel_config_collection_clone
        .get(&avatar.id)
        .unwrap();

    let skill_depot = avatar_skill_depot_excel_config_collection_clone
        .get(&avatar.skill_depot_id)
        .unwrap();

    let talent_id_list: Vec<u32> =
        if DEFAULT_CORE_PROUD_SKILL_LEVEL as usize > skill_depot.talents.len() {
            skill_depot.talents.clone()
        } else {
            skill_depot.talents[0..DEFAULT_CORE_PROUD_SKILL_LEVEL as usize].to_vec()
        };

    let mut open_configs = Vec::new();
    open_configs.extend(process_talent_ids(
        &talent_id_list,
        &avatar_talent_collection_clone,
    ));
    open_configs.extend(process_inherent_proud_skills(
        &inherent_proud_skill_list,
        &proud_skill_collection_clone,
    ));

    let mut skill_extra_charge_map: HashMap<u32, AvatarSkillBin> = HashMap::new();

    for open_config in &open_configs {
        match nod_krai_gi_data::config::get_avatar_talent_config(&open_config.clone().into()) {
            None => continue,
            Some(talent_action) => {
                for action in talent_action {
                    if let nod_krai_gi_data::config::TalentAction::ModifySkillPoint {
                        skill_id,
                        point_delta,
                    } = action
                    {
                        if let Some(skill_config) =
                            avatar_skill_excel_config_collection_clone.get(&skill_id)
                        {
                            let max_charge_num = skill_config.max_charge_num;
                            let extra_charge = max_charge_num + point_delta;
                            skill_extra_charge_map.insert(
                                *skill_id,
                                AvatarSkillBin {
                                    max_charge_count: extra_charge,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    if avatar.id == CHOOSE_AVATAR_ID {
        player.avatar_bin.as_mut().unwrap().choose_avatar_guid = avatar_guid;
    }

    let mut depot_map: HashMap<u32, AvatarSkillDepotBin> = HashMap::new();
    depot_map.insert(
        avatar.skill_depot_id,
        AvatarSkillDepotBin {
            talent_id_list,
            core_proud_skill_level: DEFAULT_CORE_PROUD_SKILL_LEVEL,
            inherent_proud_skill_list,
            skill_level_map,
        },
    );

    player.avatar_bin.as_mut().unwrap().avatar_map.insert(
        avatar_guid,
        AvatarBin {
            avatar_id: avatar.id,
            level: DEFAULT_AVATAR_LEVEL,
            promote_level: DEFAULT_AVATAR_PROMOTE_LEVEL,
            skill_map: skill_extra_charge_map,
            depot_map: depot_map,
            skill_depot_id: avatar.skill_depot_id,
            born_time: time_util::unix_timestamp() as u32,
            guid: avatar_guid,
            weapon_guid,
            cur_hp: avatar.hp_base,
            wearing_flycloak_id: DEFAULT_FLYCLOAK_ID,
            costume_id: 0,
            trace_effect_id: 0,
            weapon_skin_id: 0,
            ..Default::default()
        },
    );

    player.item_bin.as_mut().unwrap().add_item(
        weapon_guid,
        ItemBin {
            item_type: 0,
            item_id: avatar.initial_weapon,
            guid: weapon_guid,
            detail: Some(item_bin::Detail::Equip(EquipBin {
                is_locked: false,
                detail: Some(equip_bin::Detail::Weapon(WeaponBin {
                    level: DEFAULT_WEAPON_LEVEL,
                    exp: 0,
                    promote_level: DEFAULT_WEAPON_PROMOTE_LEVEL,
                    affix_map: HashMap::with_capacity(0),
                })),
            })),
        },
    );
}

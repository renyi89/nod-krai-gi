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

use nod_krai_gi_persistence::player_information::*;

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
        nick_name,
        guid_counter: 0,
        basic_bin: PlayerBasicCompBin {
            level: DEFAULT_LEVEL,
            exp: 0,
            is_game_time_locked: false,
        },
        avatar_bin: PlayerAvatarCompBin {
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
        },
        item_bin: PlayerItemCompBin {
            pack_store: ItemStoreBin {
                item_map: Default::default(),
            },
        },
        scene_bin: PlayerSceneCompBin {
            my_cur_scene_id: 3,
            my_prev_pos: (2336.789, 249.98996, -751.3081).into(),
            my_prev_rot: (0.0, 0.0, 0.0).into(),
        },
        quest_bin: PlayerQuestCompBin {
            enable: true,
            parent_quest_map: Default::default(),
            quest_map: Default::default(),
        },
    };

    avatar_excel_config_collection_clone
        .values()
        .filter(|avatar| avatar.use_type == AvatarUseType::Formal)
        .for_each(|avatar| add_avatar_and_weapon(&mut player, avatar));

    player.avatar_bin.team_map.insert(
        1,
        AvatarTeamBin {
            avatar_guid_list: DEFAULT_TEAM
                .iter()
                .map(|id| {
                    player
                        .avatar_bin
                        .avatar_map
                        .iter()
                        .find(|(_, av)| av.avatar_id == *id)
                        .map(|(guid, _)| *guid)
                })
                .flatten()
                .collect(),
            team_name: String::new(),
        },
    );

    player.avatar_bin.cur_avatar_guid_list = player
        .avatar_bin
        .team_map
        .get(&1)
        .unwrap()
        .avatar_guid_list
        .clone();

    // Add bunch of weapons to inventory
    weapon_excel_config_collection_clone
        .values()
        .for_each(|weapon| {
            let guid = player.next_guid();
            player.item_bin.add_item(
                guid,
                ItemBin::Weapon {
                    weapon_id: weapon.id,
                    level: 90,
                    exp: 0,
                    promote_level: 6,
                    affix_map: HashMap::with_capacity(0),
                    is_locked: false,
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
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    if avatar.id == CHOOSE_AVATAR_ID {
        player.avatar_bin.choose_avatar_guid = avatar_guid;
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

    player.avatar_bin.avatar_map.insert(
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
        },
    );

    player.item_bin.add_item(
        weapon_guid,
        ItemBin::Weapon {
            weapon_id: avatar.initial_weapon,
            level: DEFAULT_WEAPON_LEVEL,
            exp: 0,
            promote_level: DEFAULT_WEAPON_PROMOTE_LEVEL,
            affix_map: HashMap::with_capacity(0),
            is_locked: false,
        },
    );
}

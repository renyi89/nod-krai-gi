use std::collections::HashMap;

use common::time_util;
use nod_krai_gi_data::config::{process_inherent_proud_skills, process_talent_ids};
use nod_krai_gi_data::excel::common::{EquipType, ItemType};
use nod_krai_gi_data::excel::{
    avatar_excel_config_collection, avatar_skill_depot_excel_config_collection,
    avatar_skill_excel_config_collection, avatar_talent_excel_config_collection,
    proud_skill_excel_config_collection, AvatarExcelConfig,
};
use nod_krai_gi_proto::server_only::*;

pub fn add_avatar_and_weapon(player: &mut PlayerDataBin, avatar: &AvatarExcelConfig) {
    const CHOOSE_AVATAR_ID: u32 = 10000007;
    const DEFAULT_AVATAR_LEVEL: u32 = 100;
    const DEFAULT_AVATAR_PROMOTE_LEVEL: u32 = 6;
    const DEFAULT_CORE_PROUD_SKILL_LEVEL: u32 = 6;
    const DEFAULT_WEAPON_LEVEL: u32 = 100;
    const DEFAULT_WEAPON_PROMOTE_LEVEL: u32 = 6;
    const DEFAULT_FLYCLOAK_ID: u32 = 140001;

    let avatar_guid = player.next_guid();
    let weapon_guid = player.next_guid();

    let Some(ref mut avatar_bin) = player.avatar_bin else {
        return;
    };

    let Some(ref mut item_bin) = player.item_bin else {
        return;
    };

    let mut skill_level_map = HashMap::new();
    let mut inherent_proud_skill_list = Vec::new();

    let avatar_skill_depot_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_skill_depot_excel_config_collection::get());

    let avatar_skill_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_skill_excel_config_collection::get());

    let avatar_talent_collection_clone =
        std::sync::Arc::clone(avatar_talent_excel_config_collection::get());

    let proud_skill_collection_clone =
        std::sync::Arc::clone(proud_skill_excel_config_collection::get());

    if let Some(skill_depot_config) =
        avatar_skill_depot_excel_config_collection_clone.get(&avatar.skill_depot_id)
    {
        skill_depot_config
            .skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_depot_config
            .sub_skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_level_map.insert(skill_depot_config.energy_skill, 14);

        skill_depot_config
            .inherent_proud_skill_opens
            .iter()
            .filter(|s| {
                s.proud_skill_group_id != 0 && s.need_avatar_promote_level <= DEFAULT_AVATAR_LEVEL
            })
            .for_each(|s| inherent_proud_skill_list.push(s.proud_skill_group_id * 100 + 1));
    }

    let Some(avatar_config) = avatar_excel_config_collection::get().get(&avatar.id) else {
        tracing::debug!("avatar config {} doesn't exist", avatar.id);
        return;
    };

    let Some(skill_depot_config) =
        avatar_skill_depot_excel_config_collection_clone.get(&avatar_config.skill_depot_id)
    else {
        tracing::debug!(
            "avatar skill depot config {} doesn't exist",
            avatar_config.skill_depot_id
        );
        return;
    };

    let talent_id_list: Vec<u32> =
        if DEFAULT_CORE_PROUD_SKILL_LEVEL as usize > skill_depot_config.talents.len() {
            skill_depot_config.talents.clone()
        } else {
            skill_depot_config.talents[0..DEFAULT_CORE_PROUD_SKILL_LEVEL as usize].to_vec()
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

    if avatar_config.id == CHOOSE_AVATAR_ID {
        avatar_bin.choose_avatar_guid = avatar_guid;
    }

    let mut depot_map: HashMap<u32, AvatarSkillDepotBin> = HashMap::new();
    depot_map.insert(
        avatar_config.skill_depot_id,
        AvatarSkillDepotBin {
            talent_id_list,
            core_proud_skill_level: DEFAULT_CORE_PROUD_SKILL_LEVEL,
            inherent_proud_skill_list,
            skill_level_map,
        },
    );

    let mut equip_map = HashMap::new();
    let weapon_item_bin = ItemBin {
        item_type: ItemType::WEAPON as u32,
        item_id: avatar_config.initial_weapon,
        guid: weapon_guid,
        owner_guid: avatar_guid,
        detail: Some(item_bin::Detail::Equip(EquipBin {
            is_locked: false,
            detail: Some(equip_bin::Detail::Weapon(WeaponBin {
                level: DEFAULT_WEAPON_LEVEL,
                exp: 0,
                promote_level: DEFAULT_WEAPON_PROMOTE_LEVEL,
                affix_map: HashMap::with_capacity(0),
            })),
        })),
    };

    equip_map.insert(EquipType::Weapon as u32, weapon_item_bin.clone());

    item_bin.add_item(weapon_guid, weapon_item_bin);

    avatar_bin.avatar_map.insert(
        avatar_guid,
        AvatarBin {
            avatar_type: 1,
            avatar_id: avatar_config.id,
            level: DEFAULT_AVATAR_LEVEL,
            promote_level: DEFAULT_AVATAR_PROMOTE_LEVEL,
            skill_map: skill_extra_charge_map,
            depot_map,
            skill_depot_id: avatar_config.skill_depot_id,
            born_time: time_util::unix_timestamp() as u32,
            guid: avatar_guid,
            equip_map,
            cur_hp: avatar_config.hp_base,
            wearing_flycloak_id: DEFAULT_FLYCLOAK_ID,
            costume_id: 0,
            trace_effect_id: 0,
            weapon_skin_id: 0,
            ..Default::default()
        },
    );
}

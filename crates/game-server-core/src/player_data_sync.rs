use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::time_util;
use nod_krai_gi_data::excel;
use nod_krai_gi_data::excel::{FetterDataConfig, FetterDataConfigKeyed};
use nod_krai_gi_entity::common::create_fight_props_with_equip;
use nod_krai_gi_entity::{common::LifeState, int_prop_map};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::*;

pub struct PlayerDataSyncPlugin;

impl Plugin for PlayerDataSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sync_player_data,
                sync_player_store,
                sync_open_state_map,
                sync_avatar_data,
                sync_quest_list,
            )
                .chain(),
        );
    }
}

pub fn sync_player_data(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };
        let Some(ref player_basic_bin) = player_info.basic_bin else {
            continue;
        };
        out.send(
            *uid,
            "PlayerDataNotify",
            PlayerDataNotify {
                nick_name: player_basic_bin.nickname.clone(),
                prop_map: int_prop_map! {
                    PROP_PLAYER_WORLD_LEVEL: 9;
                    PROP_IS_SPRING_AUTO_USE: 1;
                    PROP_SPRING_AUTO_USE_PERCENT: 50;
                    PROP_IS_FLYABLE: 1;
                    PROP_IS_GAME_TIME_LOCKED: player_basic_bin.is_game_time_locked as i64;
                    PROP_IS_TRANSFERABLE: 1;
                    PROP_MAX_STAMINA: 24000;
                    PROP_CUR_PERSIST_STAMINA: 24000;
                    PROP_PLAYER_LEVEL: player_basic_bin.level;
                    PROP_PLAYER_EXP: player_basic_bin.exp;
                    PROP_PLAYER_MP_SETTING_TYPE :1;
                    PROP_IS_MP_MODE_AVAILABLE :1;
                    PROP_PLAYER_RESIN:200;
                    PROP_IS_DIVEABLE :1;
                    PROP_CUR_PHLOGISTON :10000;
                    PROP_PLAYER_HCOIN :991;
                    PROP_PLAYER_SCOIN :992;
                    PROP_PLAYER_MCOIN :993;
                },
                server_time: time_util::unix_timestamp_ms(),
                is_first_login_today: false,
                region_id: 0,
            },
        )
    }
}

pub fn sync_player_store(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };
        let Some(ref player_item_bin) = player_info.item_bin else {
            continue;
        };

        out.send(
            *uid,
            "PlayerStoreNotify",
            PlayerStoreNotify {
                store_type: StoreType::StorePack.into(),
                weight_limit: 30_000,
                item_list: player_item_bin
                    .iter()
                    .filter_map(|(_guid, item)| item.to_normal_proto())
                    .collect(),
            },
        );
    }
}

pub fn sync_avatar_data(players: Res<Players>, out: Res<MessageOutput>) {
    let avatar_excel_config_collection_clone =
        std::sync::Arc::clone(excel::avatar_excel_config_collection::get());

    let fetter_data_entries_clone =
        std::sync::Arc::clone(FetterDataConfig::get_fetter_data_entries());

    for uid in players.keys() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };

        let Some(ref player_avatar_bin) = player_info.avatar_bin else {
            continue;
        };

        out.send(
            *uid,
            "AvatarDataNotify",
            AvatarDataNotify {
                choose_avatar_guid: player_avatar_bin.choose_avatar_guid,
                avatar_list: player_avatar_bin
                    .avatar_map
                    .values()
                    .filter_map(|avatar_bin| {
                        let Some(skill_depot) =
                            avatar_bin.depot_map.get(&avatar_bin.skill_depot_id)
                        else {
                            tracing::debug!(
                                "skill_depot bin {} doesn't exist",
                                avatar_bin.skill_depot_id
                            );
                            return None;
                        };

                        let mut fetter_data_list = vec![];

                        if fetter_data_entries_clone.contains_key(&avatar_bin.avatar_id) {
                            let Some(temp_fetter_data_list) = fetter_data_entries_clone
                                .get(&avatar_bin.avatar_id)
                                .cloned()
                            else {
                                tracing::debug!(
                                    "fetter config {} doesn't exist",
                                    avatar_bin.avatar_id
                                );
                                return None;
                            };
                            fetter_data_list = temp_fetter_data_list;
                        }

                        let Some(avatar_config) =
                            avatar_excel_config_collection_clone.get(&avatar_bin.avatar_id)
                        else {
                            tracing::debug!("avatar config {} doesn't exist", avatar_bin.avatar_id);
                            return None;
                        };

                        Some(AvatarInfo {
                            avatar_type: 1,
                            avatar_id: avatar_bin.avatar_id,
                            guid: avatar_bin.guid,
                            equip_guid_list: avatar_bin
                                .equip_map
                                .iter()
                                .map(|(_, item)| item.guid)
                                .collect(),
                            skill_depot_id: avatar_bin.skill_depot_id,
                            talent_id_list: skill_depot.talent_id_list.clone(),
                            core_proud_skill_level: skill_depot.core_proud_skill_level,
                            born_time: avatar_bin.born_time,
                            life_state: (avatar_bin.cur_hp > 0.0)
                                .then_some(LifeState::Alive)
                                .unwrap_or(LifeState::Dead)
                                as u32,
                            wearing_flycloak_id: avatar_bin.wearing_flycloak_id,
                            costume_id: avatar_bin.costume_id,
                            trace_effect_id: avatar_bin.trace_effect_id,
                            fetter_info: Some(AvatarFetterInfo {
                                fetter_list: fetter_data_list
                                    .into_iter()
                                    .map(|x| FetterData {
                                        fetter_state: 3,
                                        fetter_id: x.fetter_id,
                                        ..Default::default()
                                    })
                                    .collect(),
                                ..Default::default()
                            }),
                            skill_level_map: skill_depot.skill_level_map.clone(),
                            skill_map: avatar_bin
                                .skill_map
                                .iter()
                                .map(|(k, v)| {
                                    (
                                        *k,
                                        AvatarSkillInfo {
                                            max_charge_count: v.max_charge_count,
                                            ..Default::default()
                                        },
                                    )
                                })
                                .collect(),
                            inherent_proud_skill_list: skill_depot
                                .inherent_proud_skill_list
                                .clone(),
                            prop_map: int_prop_map! {
                                PROP_LEVEL: avatar_bin.level;
                                PROP_BREAK_LEVEL: avatar_bin.promote_level;
                            },
                            fight_prop_map: create_fight_props_with_equip(
                                avatar_bin,
                                avatar_config,
                            )
                            .0
                            .iter()
                            .map(|(ty, val)| (*ty as u32, *val))
                            .collect(),
                            ..Default::default()
                        })
                    })
                    .collect(),
                avatar_team_map: player_avatar_bin
                    .team_map
                    .iter()
                    .map(|(idx, team)| {
                        (
                            *idx,
                            AvatarTeam {
                                team_name: team.team_name.clone(),
                                avatar_guid_list: team.avatar_guid_list.clone(),
                            },
                        )
                    })
                    .collect(),
                cur_avatar_team_id: player_avatar_bin.cur_team_id,
                owned_flycloak_list: player_avatar_bin
                    .owned_flycloak_list
                    .iter()
                    .copied()
                    .collect(),
                owned_costume_list: player_avatar_bin
                    .owned_costume_list
                    .iter()
                    .copied()
                    .collect(),
                owned_trace_effect_list: player_avatar_bin
                    .owned_trace_effect_list
                    .iter()
                    .copied()
                    .collect(),
                ..Default::default()
            },
        );
    }
}

pub fn sync_open_state_map(players: Res<Players>, out: Res<MessageOutput>) {
    let open_state_config_collection_clone =
        std::sync::Arc::clone(excel::open_state_config_collection::get());

    for uid in players.keys() {
        out.send(
            *uid,
            "OpenStateUpdateNotify",
            OpenStateUpdateNotify {
                open_state_map: open_state_config_collection_clone
                    .values()
                    .map(|c| (c.id, 1))
                    .collect(),
            },
        );
    }
}

pub fn sync_quest_list(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };

        let Some(ref player_quest_bin) = player_info.quest_bin else {
            continue;
        };
        let Some(ref quest_bin) = player_quest_bin.quest_bin else {
            continue;
        };

        out.send(
            *uid,
            "QuestListNotify",
            QuestListNotify {
                quest_list: quest_bin
                    .quest_map
                    .iter()
                    .map(|(sub_quest_id, quest_item)| Quest {
                        quest_id: *sub_quest_id,
                        parent_quest_id: quest_item.parent_quest_id,
                        state: quest_item.state,
                        start_time: quest_item.start_time,
                        accept_time: quest_item.accept_time,
                        start_game_time: 438,
                        finish_progress_list: quest_item.finish_progress_list.clone(),
                        fail_progress_list: quest_item.fail_progress_list.clone(),
                        ..Default::default()
                    })
                    .collect(),
            },
        );
    }
}

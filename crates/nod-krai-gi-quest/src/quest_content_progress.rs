use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::common::QuestState;
use nod_krai_gi_data::quest;
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::QuestProgressUpdateNotify;

use crate::quest_state::ensure_player_quest_comp;

pub fn quest_content_progress(
    mut events: MessageReader<QuestContentProgressEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut quest_finish_event: MessageWriter<QuestFinishEvent>,
    mut quest_fail_event: MessageWriter<QuestFailEvent>,
) {
    let sub_quest_config_collection_clone = quest::quest_config::get_sub_quest_config_collection();

    for event in events.read() {
        let QuestContentProgressEvent {
            player_uid,
            content_type,
            param,
            param2,
            param3,
            add_progress,
        } = *event;

        let Some(player_info) = players.get_mut(player_uid) else {
            continue;
        };
        let player_quest_bin = ensure_player_quest_comp(player_info);

        let Some(ref mut quest_bin) = player_quest_bin.quest_bin else {
            continue;
        };

        let mut finished_quest_ids = Vec::new();
        let mut failed_quest_ids = Vec::new();

        for (quest_id, quest_item) in quest_bin.quest_map.iter_mut() {
            if quest_item.state != QuestState::Unfinished as u32 {
                continue;
            }

            let Some(sub_quest_config) = sub_quest_config_collection_clone.get(quest_id) else {
                continue;
            };

            // 处理完成条件 - 更新所有匹配的条件（去掉 break）
            let mut finish_cond_matched = false;
            for (idx, cond) in sub_quest_config.finish_cond.iter().enumerate() {
                if cond.r#type != content_type {
                    continue;
                }
                if !is_param_match(&cond.param, param, param2, param3) {
                    continue;
                }

                if idx < quest_item.finish_progress_list.len() {
                    let current = quest_item.finish_progress_list[idx];
                    let max_count = cond.count;
                    quest_item.finish_progress_list[idx] =
                        current.saturating_add(add_progress).min(max_count);
                }

                finish_cond_matched = true;
            }

            // 处理失败条件 - 更新所有匹配的条件（去掉 break）
            let mut fail_cond_matched = false;
            for (idx, cond) in sub_quest_config.fail_cond.iter().enumerate() {
                if cond.r#type != content_type {
                    continue;
                }
                if !is_param_match(&cond.param, param, param2, param3) {
                    continue;
                }

                if idx < quest_item.fail_progress_list.len() {
                    let current = quest_item.fail_progress_list[idx];
                    let max_count = cond.count;
                    quest_item.fail_progress_list[idx] =
                        current.saturating_add(add_progress).min(max_count);
                }

                fail_cond_matched = true;
            }

            // 只在有匹配条件时发送进度更新
            if finish_cond_matched || fail_cond_matched {
                message_output.send(
                    player_uid,
                    "QuestProgressUpdateNotify",
                    QuestProgressUpdateNotify {
                        quest_id: *quest_id,
                        finish_progress_list: quest_item.finish_progress_list.clone(),
                        fail_progress_list: quest_item.fail_progress_list.clone(),
                    },
                );
            }

            // 检查所有完成条件是否满足 - 使用 finish_cond_comb 逻辑组合
            let finish_results: Vec<bool> = sub_quest_config
                .finish_cond
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    i < quest_item.finish_progress_list.len()
                        && quest_item.finish_progress_list[i] >= c.count
                })
                .collect();
            let all_finish_met =
                evaluate_logic_comb(&sub_quest_config.finish_cond_comb, &finish_results);

            if all_finish_met && !sub_quest_config.finish_cond.is_empty() {
                finished_quest_ids.push(*quest_id);
                continue;
            }

            // 检查所有失败条件是否满足 - 使用 fail_cond_comb 逻辑组合
            let fail_results: Vec<bool> = sub_quest_config
                .fail_cond
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    i < quest_item.fail_progress_list.len()
                        && quest_item.fail_progress_list[i] >= c.count
                })
                .collect();
            let all_fail_met = evaluate_logic_comb(&sub_quest_config.fail_cond_comb, &fail_results);

            if all_fail_met && !sub_quest_config.fail_cond.is_empty() {
                failed_quest_ids.push(*quest_id);
            }
        }

        for quest_id in finished_quest_ids {
            quest_finish_event.write(QuestFinishEvent(player_uid, quest_id));
        }

        for quest_id in failed_quest_ids {
            quest_fail_event.write(QuestFailEvent(player_uid, quest_id));
        }
    }
}

pub fn quest_accept_cond(
    mut events: MessageReader<QuestAcceptCondEvent>,
    players: Res<Players>,
    mut quest_accept_event: MessageWriter<QuestAcceptEvent>,
) {
    let sub_quest_config_collection_clone =
        nod_krai_gi_data::quest::quest_config::get_sub_quest_config_collection();

    let quest_config_collection_clone =
        nod_krai_gi_data::quest::quest_config::get_quest_config_collection();

    for event in events.read() {
        let QuestAcceptCondEvent {
            player_uid,
            cond_type,
            param,
        } = *event;

        let Some(player_info) = players.get(player_uid) else {
            continue;
        };

        let existing_quests: std::collections::HashSet<u32> = if let Some(ref quest_bin) =
            player_info
                .quest_bin
                .as_ref()
                .and_then(|q| q.quest_bin.as_ref())
        {
            quest_bin.quest_map.keys().copied().collect()
        } else {
            std::collections::HashSet::new()
        };

        for (_, quest_config) in quest_config_collection_clone.iter() {
            let Some(ref sub_quests) = quest_config.sub_quests else {
                continue;
            };
            for sub_quest in sub_quests {
                if existing_quests.contains(&sub_quest.sub_id) {
                    continue;
                }

                let Some(sub_quest_config) =
                    sub_quest_config_collection_clone.get(&sub_quest.sub_id)
                else {
                    continue;
                };

                let has_matching_cond = sub_quest_config.accept_cond.iter().any(|cond| {
                    cond.r#type == cond_type
                        && (cond.param.is_empty() || cond.param[0] == param || cond.param[0] == 0)
                });

                if !has_matching_cond {
                    continue;
                }

                let all_accept_conds_met = evaluate_accept_cond(
                    &sub_quest_config.accept_cond,
                    &sub_quest_config.accept_cond_comb,
                    &existing_quests,
                    player_info,
                );

                if all_accept_conds_met {
                    quest_accept_event.write(QuestAcceptEvent(player_uid, sub_quest.sub_id));
                }
            }
        }
    }
}

fn evaluate_accept_cond(
    accept_cond: &[nod_krai_gi_data::quest::quest_config::QuestAcceptCondition],
    logic_type: &nod_krai_gi_data::quest::quest_config::LogicType,
    existing_quests: &std::collections::HashSet<u32>,
    player_info: &nod_krai_gi_proto::server_only::PlayerDataBin,
) -> bool {
    if accept_cond.is_empty() {
        return true;
    }

    let mut results = Vec::with_capacity(accept_cond.len());

    for cond in accept_cond {
        let met =
            match cond.r#type {
                nod_krai_gi_data::quest::quest_config::QuestCond::StateEqual => {
                    if cond.param.len() >= 2 {
                        let quest_id = cond.param[0];
                        let expected_state = cond.param[1];
                        if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin
                                .quest_map
                                .get(&quest_id)
                                .map(|q| q.state == expected_state)
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::StateNotEqual => {
                    if cond.param.len() >= 2 {
                        let quest_id = cond.param[0];
                        let unwanted_state = cond.param[1];
                        if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin
                                .quest_map
                                .get(&quest_id)
                                .map(|q| q.state != unwanted_state)
                                .unwrap_or(true)
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::PackHaveItem => true,
                nod_krai_gi_data::quest::quest_config::QuestCond::PlayerLevelEqualGreater => {
                    if let Some(ref basic_bin) = player_info.basic_bin {
                        if !cond.param.is_empty() {
                            basic_bin.level >= cond.param[0]
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::CompleteTalk => true,
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestNotReceive => {
                    if !cond.param.is_empty() {
                        !existing_quests.contains(&cond.param[0])
                    } else {
                        true
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::IsWorldOwner => true,
                nod_krai_gi_data::quest::quest_config::QuestCond::OpenStateEqual => true,
                nod_krai_gi_data::quest::quest_config::QuestCond::LuaNotify => true,
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestVarEqual => {
                    if cond.param.len() >= 2 {
                        let var_idx = cond.param[0] as usize;
                        let expected = cond.param[1] as i32;
                        // Look up quest var from parent quest
                        if let Some(ref parent_quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.parent_quest_bin.as_ref())
                        {
                            // Check all parent quests for the matching var
                            parent_quest_bin.parent_quest_map.values().any(|pq| {
                                pq.quest_var.get(var_idx).copied().unwrap_or(0) == expected
                            })
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestVarGreater => {
                    if cond.param.len() >= 2 {
                        let var_idx = cond.param[0] as usize;
                        let threshold = cond.param[1] as i32;
                        if let Some(ref parent_quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.parent_quest_bin.as_ref())
                        {
                            parent_quest_bin.parent_quest_map.values().any(|pq| {
                                pq.quest_var.get(var_idx).copied().unwrap_or(0) > threshold
                            })
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestVarLess => {
                    if cond.param.len() >= 2 {
                        let var_idx = cond.param[0] as usize;
                        let threshold = cond.param[1] as i32;
                        if let Some(ref parent_quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.parent_quest_bin.as_ref())
                        {
                            parent_quest_bin.parent_quest_map.values().any(|pq| {
                                pq.quest_var.get(var_idx).copied().unwrap_or(0) < threshold
                            })
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestGlobalVarEqual => {
                    if cond.param.len() >= 2 {
                        let key = cond.param[0];
                        let expected = cond.param[1] as i32;
                        player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| {
                                q.quest_global_var_list
                                    .iter()
                                    .find(|v| v.key == key)
                                    .map(|v| v.value == expected)
                            })
                            .unwrap_or(false)
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestGlobalVarGreater => {
                    if cond.param.len() >= 2 {
                        let key = cond.param[0];
                        let threshold = cond.param[1] as i32;
                        player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| {
                                q.quest_global_var_list
                                    .iter()
                                    .find(|v| v.key == key)
                                    .map(|v| v.value > threshold)
                            })
                            .unwrap_or(false)
                    } else {
                        false
                    }
                }
                nod_krai_gi_data::quest::quest_config::QuestCond::QuestGlobalVarLess => {
                    if cond.param.len() >= 2 {
                        let key = cond.param[0];
                        let threshold = cond.param[1] as i32;
                        player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| {
                                q.quest_global_var_list
                                    .iter()
                                    .find(|v| v.key == key)
                                    .map(|v| v.value < threshold)
                            })
                            .unwrap_or(false)
                    } else {
                        false
                    }
                }
                _ => true,
            };

        results.push(met);
    }

    evaluate_logic_comb(logic_type, &results)
}

pub fn evaluate_logic_comb(
    logic_type: &nod_krai_gi_data::quest::quest_config::LogicType,
    results: &[bool],
) -> bool {
    match logic_type {
        nod_krai_gi_data::quest::quest_config::LogicType::LogicNone
        | nod_krai_gi_data::quest::quest_config::LogicType::LogicAnd => results.iter().all(|&r| r),
        nod_krai_gi_data::quest::quest_config::LogicType::LogicOr => results.iter().any(|&r| r),
        nod_krai_gi_data::quest::quest_config::LogicType::LogicNot => !results.iter().any(|&r| r),
        nod_krai_gi_data::quest::quest_config::LogicType::LogicAAndEtcor => {
            // 第一个条件必须为 true，其余条件中至少一个为 true
            if results.is_empty() {
                return true;
            }
            results[0] && results.iter().skip(1).any(|&r| r)
        }
        nod_krai_gi_data::quest::quest_config::LogicType::LogicAAndBAndEtcor => {
            // 前两个条件必须为 true，其余条件中至少一个为 true
            if results.len() < 2 {
                return results.iter().all(|&r| r);
            }
            results[0] && results[1] && results.iter().skip(2).any(|&r| r)
        }
        nod_krai_gi_data::quest::quest_config::LogicType::LogicAOrEtcand => {
            // 第一个条件必须为 true，其余条件中至少一个为 false
            if results.is_empty() {
                return true;
            }
            results[0] && results.iter().skip(1).any(|&r| !r)
        }
        nod_krai_gi_data::quest::quest_config::LogicType::LogicAOrBOrEtcand => {
            // 前两个条件中至少一个为 true，其余条件中至少一个为 false
            if results.len() < 2 {
                return results.iter().any(|&r| r);
            }
            (results[0] || results[1]) && results.iter().skip(2).any(|&r| !r)
        }
        nod_krai_gi_data::quest::quest_config::LogicType::LogicAAndBOrEtcand => {
            // 前两个条件都必须为 true，其余条件中至少一个为 false
            if results.len() < 2 {
                return results.iter().all(|&r| r);
            }
            results[0] && results[1] && results.iter().skip(2).any(|&r| !r)
        }
        nod_krai_gi_data::quest::quest_config::LogicType::QuestHidden => {
            // QUEST_HIDDEN 默认返回 false
            false
        }
    }
}

/// 检查参数是否匹配
/// C++ 参考: Quest::isParamMatch
/// 支持多参数匹配：
/// - param 为空：匹配任何参数
/// - param[0] == 0：匹配任何参数
/// - param[0] != 0：必须匹配 param
/// - param[1] != 0：必须匹配 param2
/// - param[2] != 0：必须匹配 param3
pub fn is_param_match(cond_param: &[u32], param: u32, param2: u32, param3: u32) -> bool {
    if cond_param.is_empty() {
        return true;
    }
    if cond_param[0] != 0 && cond_param[0] != param {
        return false;
    }
    if cond_param.len() > 1 && cond_param[1] != 0 && cond_param[1] != param2 {
        return false;
    }
    if cond_param.len() > 2 && cond_param[2] != 0 && cond_param[2] != param3 {
        return false;
    }
    true
}

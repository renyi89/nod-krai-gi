use bevy_ecs::prelude::{MessageReader, MessageWriter, Res, ResMut};
use common::string_util::InternString;
use nod_krai_gi_data::quest;
use nod_krai_gi_data::quest::quest_config::{QuestContent, QuestExec};
use nod_krai_gi_event::lua::{LuaTriggerEvent, SpawnGroupEntityEvent};
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::QuestUpdateQuestVarNotify;
use nod_krai_gi_proto::server_only::QuestGlobalVarBin;

use crate::quest_state::ensure_player_quest_comp;

fn param_u32(params: &[InternString], idx: usize) -> u32 {
    params
        .get(idx)
        .and_then(|s| s.as_str().parse::<u32>().ok())
        .unwrap_or(0)
}

fn param_i32(params: &[InternString], idx: usize) -> i32 {
    params
        .get(idx)
        .and_then(|s| s.as_str().parse::<i32>().ok())
        .unwrap_or(0)
}

fn param_str(params: &[InternString], idx: usize) -> &str {
    params.get(idx).map(|s| s.as_str()).unwrap_or("")
}

pub fn quest_exec(
    mut events: MessageReader<QuestExecEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
    mut quest_accept_events: MessageWriter<QuestAcceptEvent>,
    mut spawn_group_entity_events: MessageWriter<SpawnGroupEntityEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
) {
    let sub_quest_config_collection = quest::quest_config::get_sub_quest_config_collection();

    for event in events.read() {
        let Some(sub_quest_config) = sub_quest_config_collection.get(&event.quest_id) else {
            continue;
        };

        let exec_list = match event.exec_type {
            1 => &sub_quest_config.accept_exec,
            2 => &sub_quest_config.finish_exec,
            3 => &sub_quest_config.fail_exec,
            _ => continue,
        };

        if exec_list.is_empty() {
            continue;
        }

        let Some(player_info) = players.get_mut(event.player_uid) else {
            continue;
        };

        for exec_param in exec_list {
            let params = &exec_param.param;

            match exec_param.r#type {
                QuestExec::RefreshGroupSuite => {
                    for entry in param_str(params, 1).split(';') {
                        let parts: Vec<&str> = entry.split(',').collect();
                        if parts.len() >= 2 {
                            let group_id: u32 = parts[0].parse().unwrap_or(0);
                            let suite_id: u32 = parts[1].parse().unwrap_or(0);
                            if group_id != 0 {
                                spawn_group_entity_events.write(SpawnGroupEntityEvent {
                                    scene_id: 0,
                                    block_id: 0,
                                    group_id,
                                    refresh_suite_id: suite_id,
                                });
                            }
                        }
                    }
                    tracing::debug!(
                        "[QuestExec] RefreshGroupSuite quest={} str_param={}",
                        event.quest_id,
                        param_str(params, 1)
                    );
                }
                QuestExec::SetQuestVar => {
                    let index = param_u32(params, 0) as usize;
                    let value = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&event.parent_quest_id)
                        {
                            while parent_quest.quest_var.len() <= index {
                                parent_quest.quest_var.push(0);
                            }
                            parent_quest.quest_var[index] = value;
                            message_output.send(
                                event.player_uid,
                                "QuestUpdateQuestVarNotify",
                                QuestUpdateQuestVarNotify {
                                    parent_quest_id: event.parent_quest_id,
                                    quest_var: parent_quest.quest_var.clone(),
                                    ..Default::default()
                                },
                            );
                            tracing::debug!(
                                "[QuestExec] SetQuestVar parent={} idx={} val={}",
                                event.parent_quest_id,
                                index,
                                value
                            );
                        }
                    }
                }
                QuestExec::IncQuestVar => {
                    let index = param_u32(params, 0) as usize;
                    let delta = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&event.parent_quest_id)
                        {
                            while parent_quest.quest_var.len() <= index {
                                parent_quest.quest_var.push(0);
                            }
                            parent_quest.quest_var[index] += delta;
                            message_output.send(
                                event.player_uid,
                                "QuestUpdateQuestVarNotify",
                                QuestUpdateQuestVarNotify {
                                    parent_quest_id: event.parent_quest_id,
                                    quest_var: parent_quest.quest_var.clone(),
                                    ..Default::default()
                                },
                            );
                            tracing::debug!(
                                "[QuestExec] IncQuestVar parent={} idx={} delta={}",
                                event.parent_quest_id,
                                index,
                                delta
                            );
                        }
                    }
                }
                QuestExec::DecQuestVar => {
                    let index = param_u32(params, 0) as usize;
                    let delta = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&event.parent_quest_id)
                        {
                            while parent_quest.quest_var.len() <= index {
                                parent_quest.quest_var.push(0);
                            }
                            parent_quest.quest_var[index] -= delta;
                            message_output.send(
                                event.player_uid,
                                "QuestUpdateQuestVarNotify",
                                QuestUpdateQuestVarNotify {
                                    parent_quest_id: event.parent_quest_id,
                                    quest_var: parent_quest.quest_var.clone(),
                                    ..Default::default()
                                },
                            );
                            tracing::debug!(
                                "[QuestExec] DecQuestVar parent={} idx={} delta={}",
                                event.parent_quest_id,
                                index,
                                delta
                            );
                        }
                    }
                }
                QuestExec::SetQuestGlobalVar => {
                    let key = param_u32(params, 0);
                    let value = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(existing) =
                        player_quest_bin.quest_global_var_list.iter_mut().find(|v| v.key == key)
                    {
                        existing.value = value;
                    } else {
                        player_quest_bin
                            .quest_global_var_list
                            .push(QuestGlobalVarBin { key, value });
                    }
                    tracing::debug!("[QuestExec] SetQuestGlobalVar key={} val={}", key, value);
                }
                QuestExec::IncQuestGlobalVar => {
                    let key = param_u32(params, 0);
                    let delta = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(existing) =
                        player_quest_bin.quest_global_var_list.iter_mut().find(|v| v.key == key)
                    {
                        existing.value += delta;
                    } else {
                        player_quest_bin
                            .quest_global_var_list
                            .push(QuestGlobalVarBin { key, value: delta });
                    }
                    tracing::debug!("[QuestExec] IncQuestGlobalVar key={} delta={}", key, delta);
                }
                QuestExec::DecQuestGlobalVar => {
                    let key = param_u32(params, 0);
                    let delta = param_i32(params, 1);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(existing) =
                        player_quest_bin.quest_global_var_list.iter_mut().find(|v| v.key == key)
                    {
                        existing.value -= delta;
                    } else {
                        player_quest_bin
                            .quest_global_var_list
                            .push(QuestGlobalVarBin { key, value: -delta });
                    }
                    tracing::debug!("[QuestExec] DecQuestGlobalVar key={} delta={}", key, delta);
                }
                QuestExec::AddQuestProgress => {
                    let param_str_val = param_str(params, 0);
                    let param = param_str_val.parse::<u32>().unwrap_or(0);
                    quest_content_events.write(QuestContentProgressEvent {
                        player_uid: event.player_uid,
                        content_type: QuestContent::AddQuestProgress,
                        param,
                        param2: 0,
                        param3: 0,
                        add_progress: param_u32(params, 1).max(1),
                    });
                    tracing::debug!(
                        "[QuestExec] AddQuestProgress param_str={} parsed={}",
                        param_str_val,
                        param
                    );
                }
                QuestExec::RollbackQuest => {
                    let target_quest_id = param_u32(params, 0);
                    quest_accept_events.write(QuestAcceptEvent(event.player_uid, target_quest_id));
                    tracing::debug!("[QuestExec] RollbackQuest target={}", target_quest_id);
                }
                QuestExec::RollbackParentQuest => {
                    let target_quest_id = param_u32(params, 0);
                    quest_accept_events.write(QuestAcceptEvent(event.player_uid, target_quest_id));
                    tracing::debug!(
                        "[QuestExec] RollbackParentQuest target={}",
                        target_quest_id
                    );
                }
                QuestExec::NotifyGroupLua => {
                    let scene_id = param_u32(params, 0);
                    let group_id = param_u32(params, 1);
                    if group_id != 0 {
                        use nod_krai_gi_data::scene::{EventType, LuaEvt};
                        lua_trigger_events.write(LuaTriggerEvent {
                            group_id,
                            event_type: EventType::EventQuestFinish,
                            evt: LuaEvt {
                                param1: 0,
                                param2: 1,
                                param3: 0,
                                source_eid: 0,
                                target_eid: 0,
                            },
                        });
                    }
                    tracing::debug!(
                        "[QuestExec] NotifyGroupLua quest={} scene={} group={}",
                        event.quest_id,
                        scene_id,
                        group_id
                    );
                }
                QuestExec::DelPackItem => {
                    tracing::debug!(
                        "[QuestExec] DelPackItem item={} count={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::DelPackItemBatch => {
                    let raw = param_str(params, 0);
                    for entry in raw.split(';') {
                        let parts: Vec<&str> = entry.split(',').collect();
                        if parts.len() >= 2 {
                            let item_id: u32 = parts[0].parse().unwrap_or(0);
                            let count: u32 = parts[1].parse().unwrap_or(0);
                            tracing::debug!(
                                "[QuestExec] DelPackItemBatch item={} count={}",
                                item_id,
                                count
                            );
                        }
                    }
                }
                QuestExec::SetOpenState => {
                    tracing::debug!(
                        "[QuestExec] SetOpenState key={} value={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::UnlockPoint => {
                    tracing::debug!(
                        "[QuestExec] UnlockPoint scene={} point={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::UnlockArea => {
                    tracing::debug!(
                        "[QuestExec] UnlockArea scene={} area={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::RefreshGroupMonster => {
                    tracing::debug!(
                        "[QuestExec] RefreshGroupMonster group={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::GrantTrialAvatar => {
                    tracing::debug!(
                        "[QuestExec] GrantTrialAvatar avatarId={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::RemoveTrialAvatar => {
                    tracing::debug!(
                        "[QuestExec] RemoveTrialAvatar avatarId={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::AddCurAvatarEnergy => {
                    tracing::debug!("[QuestExec] AddCurAvatarEnergy");
                }
                QuestExec::SetIsFlyable => {
                    tracing::debug!(
                        "[QuestExec] SetIsFlyable value={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::SetIsGameTimeLocked => {
                    tracing::debug!(
                        "[QuestExec] SetIsGameTimeLocked value={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::ChangeAvatarElement => {
                    tracing::debug!(
                        "[QuestExec] ChangeAvatarElement avatar={} element={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::RandomQuestVar => {
                    let index = param_u32(params, 0) as usize;
                    let min_val = param_i32(params, 1);
                    let max_val = param_i32(params, 2);
                    let player_quest_bin = ensure_player_quest_comp(player_info);
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&event.parent_quest_id)
                        {
                            while parent_quest.quest_var.len() <= index {
                                parent_quest.quest_var.push(0);
                            }
                            if max_val > min_val {
                                use std::time::SystemTime;
                                let seed = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .subsec_nanos();
                                let value = min_val + ((seed as i32) % (max_val - min_val));
                                parent_quest.quest_var[index] = value;
                            }
                            message_output.send(
                                event.player_uid,
                                "QuestUpdateQuestVarNotify",
                                QuestUpdateQuestVarNotify {
                                    parent_quest_id: event.parent_quest_id,
                                    quest_var: parent_quest.quest_var.clone(),
                                    ..Default::default()
                                },
                            );
                            tracing::debug!(
                                "[QuestExec] RandomQuestVar parent={} idx={} range=[{},{}]",
                                event.parent_quest_id,
                                index,
                                min_val,
                                max_val
                            );
                        }
                    }
                }
                QuestExec::InitTimeVar => {
                    tracing::debug!(
                        "[QuestExec] InitTimeVar index={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::ClearTimeVar => {
                    tracing::debug!(
                        "[QuestExec] ClearTimeVar index={}",
                        param_u32(params, 0)
                    );
                }
                QuestExec::AddSceneTag => {
                    tracing::debug!(
                        "[QuestExec] AddSceneTag scene={} tag={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                QuestExec::DelSceneTag => {
                    tracing::debug!(
                        "[QuestExec] DelSceneTag scene={} tag={}",
                        param_u32(params, 0),
                        param_u32(params, 1)
                    );
                }
                _ => {
                    tracing::debug!(
                        "[QuestExec] Unhandled exec type {:?} quest={}",
                        exec_param.r#type,
                        event.quest_id
                    );
                }
            }
        }
    }
}

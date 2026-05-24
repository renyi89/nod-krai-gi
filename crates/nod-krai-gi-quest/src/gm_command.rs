use bevy_ecs::prelude::*;
use common::gm_util::{Command, QuestAction};
use common::time_util::unix_timestamp;
use nod_krai_gi_data::excel::common::QuestState;
use nod_krai_gi_data::quest;
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{QuestDelNotify, QuestListUpdateNotify};
use nod_krai_gi_proto::server_only::PlayerParentQuestBin;

use crate::quest_state::{ensure_player_quest_comp, update_child_quest_state_in_parent};

pub fn gm_command_handler(
    mut events: MessageReader<GmCommandEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut quest_accept_event: MessageWriter<QuestAcceptEvent>,
    mut quest_finish_event: MessageWriter<QuestFinishEvent>,
) {
    let sub_quest_config_collection_clone = quest::quest_config::get_sub_quest_config_collection();
    for GmCommandEvent(player_uid, command) in events.read() {
        let Command::Quest(action) = command else {
            continue;
        };
        let mut result = String::new();
        match action {
            QuestAction::Accept { id } => match sub_quest_config_collection_clone.get(id) {
                None => {
                    result = "unknown quest id".to_string();
                }
                Some(_) => {
                    // 检查任务是否已存在
                    let already_exists = if let Some(player_info) = players.get(*player_uid) {
                        if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin.quest_map.contains_key(id)
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if already_exists {
                        result = format!("quest {} already exists", id);
                    } else {
                        result = format!("accept quest {}", id);
                        quest_accept_event.write(QuestAcceptEvent(*player_uid, *id));
                    }
                }
            },
            QuestAction::Finish { id } => match sub_quest_config_collection_clone.get(id) {
                None => {
                    result = "unknown quest id".to_string();
                }
                Some(_) => {
                    // 检查任务是否存在
                    let exists = if let Some(player_info) = players.get(*player_uid) {
                        if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin.quest_map.contains_key(id)
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if !exists {
                        result = format!("quest {} not found", id);
                    } else {
                        result = format!("finish quest {}", id);
                        quest_finish_event.write(QuestFinishEvent(*player_uid, *id));
                    }
                }
            },
            QuestAction::Cancel { id } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                let mut parent_quest_id = 0u32;
                let mut removed = false;
                if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                    if let Some(quest_item) = quest_bin.quest_map.remove(id) {
                        parent_quest_id = quest_item.parent_quest_id;
                        removed = true;
                    }
                }

                if removed {
                    result = format!("cancelled quest {}", id);
                    message_output.send(
                        *player_uid,
                        "QuestDelNotify",
                        QuestDelNotify { quest_id: *id },
                    );
                    // 从父任务的 child_quest_state_list 中移除
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&parent_quest_id)
                        {
                            parent_quest
                                .child_quest_state_list
                                .retain(|pair| pair.key != *id);
                        }
                    }
                } else {
                    result = format!("quest {} not found", id);
                }
            }
            QuestAction::Clear { id } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                    if let Some(clear_id) = id {
                        let mut parent_quest_id = 0u32;
                        let mut removed = false;
                        if let Some(quest_item) = quest_bin.quest_map.remove(clear_id) {
                            parent_quest_id = quest_item.parent_quest_id;
                            removed = true;
                        }

                        if removed {
                            result = format!("cleared quest {}", clear_id);
                            message_output.send(
                                *player_uid,
                                "QuestDelNotify",
                                QuestDelNotify {
                                    quest_id: *clear_id,
                                },
                            );
                            // 从父任务的 child_quest_state_list 中移除
                            if let Some(ref mut parent_quest_bin) =
                                player_quest_bin.parent_quest_bin
                            {
                                if let Some(parent_quest) =
                                    parent_quest_bin.parent_quest_map.get_mut(&parent_quest_id)
                                {
                                    parent_quest
                                        .child_quest_state_list
                                        .retain(|pair| pair.key != *clear_id);
                                }
                            }
                        } else {
                            result = format!("quest {} not found", clear_id);
                        }
                    } else {
                        let count = quest_bin.quest_map.len();
                        quest_bin.quest_map.clear();
                        if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                            parent_quest_bin.parent_quest_map.clear();
                        }
                        result = format!("cleared all {} quests", count);
                    }
                } else if let Some(clear_id) = id {
                    result = format!("quest {} not found", clear_id);
                } else {
                    result = "no quests to clear".to_string();
                }
            }
            QuestAction::State { id, state } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                let mut parent_quest_id = 0u32;
                let mut found = false;
                if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                    if let Some(quest_item) = quest_bin.quest_map.get_mut(id) {
                        let current_state = QuestState::from(quest_item.state);
                        let target_state = QuestState::from(*state);
                        if crate::quest_state::can_transition_to(current_state, target_state) {
                            quest_item.state = *state;
                            parent_quest_id = quest_item.parent_quest_id;
                            found = true;
                        } else {
                            result = format!(
                                "cannot transition quest {} from {:?} to {:?}",
                                id, current_state, target_state
                            );
                        }
                    }
                }

                if found {
                    result = format!("set quest {} state to {}", id, state);
                    let client_quest = {
                        let Some(ref quest_bin) = player_quest_bin.quest_bin else {
                            continue;
                        };
                        let Some(quest_item) = quest_bin.quest_map.get(id) else {
                            continue;
                        };
                        quest_item.to_normal_proto()
                    };
                    message_output.send(
                        *player_uid,
                        "QuestListUpdateNotify",
                        QuestListUpdateNotify {
                            quest_list: vec![client_quest],
                        },
                    );
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&parent_quest_id)
                        {
                            update_child_quest_state_in_parent(parent_quest, *id, *state);
                        }
                    }
                } else if result.is_empty() {
                    result = format!("quest {} not found", id);
                }
            }
            QuestAction::Restart { id } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                let mut parent_quest_id = 0u32;
                let mut found = false;
                if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                    if let Some(quest_item) = quest_bin.quest_map.get_mut(id) {
                        let current_state = QuestState::from(quest_item.state);
                        if crate::quest_state::can_transition_to(
                            current_state,
                            QuestState::Unfinished,
                        ) {
                            quest_item.state = QuestState::Unfinished as u32;
                            quest_item.start_time = unix_timestamp() as u32;
                            quest_item.finish_progress_list.fill(0);
                            quest_item.fail_progress_list.fill(0);
                            parent_quest_id = quest_item.parent_quest_id;
                            found = true;
                        } else {
                            result =
                                format!("cannot restart quest {} from {:?}", id, current_state);
                        }
                    }
                }

                if found {
                    result = format!("restarted quest {}", id);
                    // 更新父任务中的子任务状态
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        if let Some(parent_quest) =
                            parent_quest_bin.parent_quest_map.get_mut(&parent_quest_id)
                        {
                            update_child_quest_state_in_parent(
                                parent_quest,
                                *id,
                                QuestState::Unfinished as u32,
                            );
                        }
                    }
                    let client_quest = {
                        let Some(ref quest_bin) = player_quest_bin.quest_bin else {
                            continue;
                        };
                        let Some(quest_item) = quest_bin.quest_map.get(id) else {
                            continue;
                        };
                        quest_item.to_normal_proto()
                    };
                    message_output.send(
                        *player_uid,
                        "QuestListUpdateNotify",
                        QuestListUpdateNotify {
                            quest_list: vec![client_quest],
                        },
                    );
                } else if result.is_empty() {
                    result = format!("quest {} not found", id);
                }
            }
            QuestAction::RestartAll => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                    let now = unix_timestamp() as u32;
                    let mut restarted_count = 0;
                    for quest_item in quest_bin.quest_map.values_mut() {
                        let current_state = QuestState::from(quest_item.state);
                        if crate::quest_state::can_transition_to(
                            current_state,
                            QuestState::Unfinished,
                        ) {
                            quest_item.state = QuestState::Unfinished as u32;
                            quest_item.start_time = now;
                            quest_item.start_game_time = now;
                            quest_item.finish_progress_list.fill(0);
                            quest_item.fail_progress_list.fill(0);
                            restarted_count += 1;
                        }
                    }

                    // 同步更新父任务中的子任务状态
                    if let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin {
                        for parent_quest in parent_quest_bin.parent_quest_map.values_mut() {
                            for pair in parent_quest.child_quest_state_list.iter_mut() {
                                if let Some(quest_item) = quest_bin.quest_map.get(&pair.key) {
                                    pair.value = quest_item.state;
                                }
                            }
                        }
                    }

                    let quest_list: Vec<_> = quest_bin
                        .quest_map
                        .values()
                        .map(|quest_item| quest_item.to_normal_proto())
                        .collect();
                    message_output.send(
                        *player_uid,
                        "QuestListUpdateNotify",
                        QuestListUpdateNotify { quest_list },
                    );
                    result = format!("restarted {} quests", restarted_count);
                } else {
                    result = "no quests to restart".to_string();
                }
            }
            QuestAction::Var {
                parent_id,
                index,
                value,
            } => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                let player_quest_bin = ensure_player_quest_comp(player_info);

                let parent_quest_bin =
                    player_quest_bin
                        .parent_quest_bin
                        .get_or_insert_with(|| PlayerParentQuestBin {
                            parent_quest_map: Default::default(),
                        });

                if let Some(parent_quest) = parent_quest_bin.parent_quest_map.get_mut(parent_id) {
                    let idx = index.unwrap_or(0) as usize;
                    let val = value.unwrap_or(0);
                    while parent_quest.quest_var.len() <= idx {
                        parent_quest.quest_var.push(0);
                    }
                    parent_quest.quest_var[idx] = val as i32;
                    result = format!(
                        "set quest var parent_id:{} index:{} = {}",
                        parent_id, idx, val
                    );
                    message_output.send(
                        *player_uid,
                        "QuestUpdateQuestVarNotify",
                        nod_krai_gi_proto::normal::QuestUpdateQuestVarNotify {
                            parent_quest_id: *parent_id,
                            quest_var: parent_quest.quest_var.clone(),
                            ..Default::default()
                        },
                    );
                } else {
                    result = format!("parent quest {} not found", parent_id);
                }
            }
        }
        gm_notify_events.write(ConsoleChatNotifyEvent(
            *player_uid,
            format!("result:{}", result),
        ));
    }
}

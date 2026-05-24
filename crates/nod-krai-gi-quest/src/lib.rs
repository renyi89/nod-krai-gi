pub mod gm_command;
pub mod handle_npc;
pub mod handle_quest;
pub mod quest_content_progress;
pub mod quest_exec;
pub mod quest_state;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::common::QuestState;
use nod_krai_gi_data::quest;
use nod_krai_gi_data::quest::quest_config::{self, QuestCond, QuestContent};
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{FinishedParentQuestUpdateNotify, Quest, QuestListUpdateNotify};

use crate::quest_content_progress::evaluate_logic_comb;
use crate::quest_state::{
    check_parent_quest_finished, create_quest_bin, ensure_parent_quest, ensure_player_quest_comp,
    update_child_quest_state_in_parent,
};

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_npc::handle_npc)
            .add_systems(Update, handle_quest::handle_quest_request)
            .add_systems(Update, gm_command::gm_command_handler)
            .add_systems(Update, quest_accept)
            .add_systems(Update, quest_finish)
            .add_systems(Update, quest_fail)
            .add_systems(Update, quest_exec::quest_exec)
            .add_systems(Update, quest_content_progress::quest_content_progress)
            .add_systems(Update, quest_content_progress::quest_accept_cond);
    }
}

pub fn quest_accept(
    mut events: MessageReader<QuestAcceptEvent>,
    message_output: Res<MessageOutput>,
    mut players: ResMut<Players>,
    mut quest_exec_events: MessageWriter<QuestExecEvent>,
) {
    let sub_quest_config_collection = quest::quest_config::get_sub_quest_config_collection();
    for QuestAcceptEvent(player_uid, sub_quest_id) in events.read() {
        let Some(sub_quest_data) = sub_quest_config_collection.get(sub_quest_id) else {
            continue;
        };

        let parent_quest_id;
        let now;
        let finish_cond_len = sub_quest_data.finish_cond.len();
        let fail_cond_len = sub_quest_data.fail_cond.len();

        // Phase 1: Create quest in player data
        {
            let Some(player_info) = players.get_mut(*player_uid) else {
                continue;
            };
            let player_quest_bin = ensure_player_quest_comp(player_info);

            if let Some(ref quest_bin) = player_quest_bin.quest_bin {
                if quest_bin.quest_map.contains_key(sub_quest_id) {
                    continue;
                }
            }

            let new_quest = create_quest_bin(
                *sub_quest_id,
                sub_quest_data.main_id,
                QuestState::Unfinished as u32,
                finish_cond_len,
                fail_cond_len,
            );

            parent_quest_id = new_quest.parent_quest_id;
            now = new_quest.start_game_time;

            if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                quest_bin.quest_map.insert(*sub_quest_id, new_quest);
            }

            let parent_quest = ensure_parent_quest(player_quest_bin, parent_quest_id);
            update_child_quest_state_in_parent(
                parent_quest,
                *sub_quest_id,
                QuestState::Unfinished as u32,
            );
        }

        // Phase 2: Emit exec event
        quest_exec_events.write(QuestExecEvent {
            player_uid: *player_uid,
            quest_id: *sub_quest_id,
            parent_quest_id,
            exec_type: 1,
        });

        // Phase 3: Send notification
        message_output.send(
            *player_uid,
            "QuestListUpdateNotify",
            QuestListUpdateNotify {
                quest_list: vec![Quest {
                    quest_id: *sub_quest_id,
                    parent_quest_id,
                    state: QuestState::Unfinished as u32,
                    start_time: now,
                    accept_time: now,
                    start_game_time: now,
                    finish_progress_list: vec![0u32; finish_cond_len],
                    fail_progress_list: vec![0u32; fail_cond_len],
                    ..Default::default()
                }],
            },
        );
    }
}

pub fn quest_finish(
    mut events: MessageReader<QuestFinishEvent>,
    message_output: Res<MessageOutput>,
    mut players: ResMut<Players>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
    mut quest_accept_events: MessageWriter<QuestAcceptEvent>,
    mut quest_exec_events: MessageWriter<QuestExecEvent>,
) {
    let sub_quest_config_collection = quest::quest_config::get_sub_quest_config_collection();
    for QuestFinishEvent(player_uid, sub_quest_id) in events.read() {
        let Some(sub_quest_data) = sub_quest_config_collection.get(sub_quest_id) else {
            continue;
        };

        let parent_quest_id = sub_quest_data.main_id;
        let parent_just_finished;

        // Phase 1: Modify quest state
        {
            let Some(player_info) = players.get_mut(*player_uid) else {
                continue;
            };
            let player_quest_bin = ensure_player_quest_comp(player_info);

            let can_finish = if let Some(ref quest_bin) = player_quest_bin.quest_bin {
                if let Some(quest_item) = quest_bin.quest_map.get(sub_quest_id) {
                    quest_item.state == QuestState::Unfinished as u32
                } else {
                    true
                }
            } else {
                false
            };

            if !can_finish {
                continue;
            }

            if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                if let Some(quest_item) = quest_bin.quest_map.get_mut(sub_quest_id) {
                    quest_item.state = QuestState::Finished as u32;
                    for (idx, progress) in quest_item.finish_progress_list.iter_mut().enumerate() {
                        if idx < sub_quest_data.finish_cond.len() {
                            *progress = sub_quest_data.finish_cond[idx].count;
                        }
                    }
                } else {
                    let mut new_quest = create_quest_bin(
                        *sub_quest_id,
                        parent_quest_id,
                        QuestState::Finished as u32,
                        sub_quest_data.finish_cond.len(),
                        sub_quest_data.fail_cond.len(),
                    );
                    for (idx, progress) in new_quest.finish_progress_list.iter_mut().enumerate() {
                        if idx < sub_quest_data.finish_cond.len() {
                            *progress = sub_quest_data.finish_cond[idx].count;
                        }
                    }
                    quest_bin.quest_map.insert(*sub_quest_id, new_quest);
                }
            }

            let parent_quest = ensure_parent_quest(player_quest_bin, parent_quest_id);
            update_child_quest_state_in_parent(
                parent_quest,
                *sub_quest_id,
                QuestState::Finished as u32,
            );
            parent_just_finished = check_parent_quest_finished(parent_quest);
        }

        // Phase 2: Emit exec event
        quest_exec_events.write(QuestExecEvent {
            player_uid: *player_uid,
            quest_id: *sub_quest_id,
            parent_quest_id,
            exec_type: 2,
        });

        // Phase 3: Content progress + post quests
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: *player_uid,
            content_type: QuestContent::QuestStateEqual,
            param: *sub_quest_id,
            param2: QuestState::Finished as u32,
            param3: 0,
            add_progress: 1,
        });
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: *player_uid,
            content_type: QuestContent::QuestStateNotEqual,
            param: *sub_quest_id,
            param2: QuestState::Unfinished as u32,
            param3: 0,
            add_progress: 1,
        });

        trigger_post_quests(
            *player_uid,
            *sub_quest_id,
            parent_quest_id,
            &players,
            &mut quest_accept_events,
        );

        // Phase 4: Notifications
        {
            let Some(player_info) = players.get(*player_uid) else {
                continue;
            };

            for item_param in &sub_quest_data.gain_items {
                tracing::debug!(
                    "[Quest] Grant item: quest={} item={} count={}",
                    sub_quest_id,
                    item_param.id,
                    item_param.count
                );
            }

            if let Some(ref quest_bin) = player_info
                .quest_bin
                .as_ref()
                .and_then(|q| q.quest_bin.as_ref())
            {
                if let Some(quest_item) = quest_bin.quest_map.get(sub_quest_id) {
                    message_output.send(
                        *player_uid,
                        "QuestListUpdateNotify",
                        QuestListUpdateNotify {
                            quest_list: vec![quest_item.to_normal_proto()],
                        },
                    );
                }
            }

            if parent_just_finished && sub_quest_data.finish_parent {
                if let Some(ref parent_quest_bin) = player_info
                    .quest_bin
                    .as_ref()
                    .and_then(|q| q.parent_quest_bin.as_ref())
                {
                    if let Some(parent_quest) =
                        parent_quest_bin.parent_quest_map.get(&parent_quest_id)
                    {
                        message_output.send(
                            *player_uid,
                            "FinishedParentQuestUpdateNotify",
                            FinishedParentQuestUpdateNotify {
                                parent_quest_list: vec![parent_quest.to_normal_proto()],
                            },
                        );
                    }
                }
            }
        }
    }
}

pub fn quest_fail(
    mut events: MessageReader<QuestFailEvent>,
    message_output: Res<MessageOutput>,
    mut players: ResMut<Players>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
    mut quest_accept_events: MessageWriter<QuestAcceptEvent>,
    mut quest_exec_events: MessageWriter<QuestExecEvent>,
) {
    let sub_quest_config_collection = quest::quest_config::get_sub_quest_config_collection();
    for QuestFailEvent(player_uid, sub_quest_id) in events.read() {
        let Some(sub_quest_data) = sub_quest_config_collection.get(sub_quest_id) else {
            continue;
        };

        let parent_quest_id = sub_quest_data.main_id;
        let parent_just_failed;

        // Phase 1: Modify quest state
        {
            let Some(player_info) = players.get_mut(*player_uid) else {
                continue;
            };
            let player_quest_bin = ensure_player_quest_comp(player_info);

            let can_fail = if let Some(ref quest_bin) = player_quest_bin.quest_bin {
                if let Some(quest_item) = quest_bin.quest_map.get(sub_quest_id) {
                    quest_item.state == QuestState::Unfinished as u32
                } else {
                    false
                }
            } else {
                false
            };

            if !can_fail {
                continue;
            }

            if let Some(ref mut quest_bin) = player_quest_bin.quest_bin {
                if let Some(quest_item) = quest_bin.quest_map.get_mut(sub_quest_id) {
                    quest_item.state = QuestState::Failed as u32;
                }
            }

            let parent_quest = ensure_parent_quest(player_quest_bin, parent_quest_id);
            update_child_quest_state_in_parent(
                parent_quest,
                *sub_quest_id,
                QuestState::Failed as u32,
            );
            parent_just_failed = crate::quest_state::check_parent_quest_failed(parent_quest);
        }

        // Phase 2: Emit exec event
        quest_exec_events.write(QuestExecEvent {
            player_uid: *player_uid,
            quest_id: *sub_quest_id,
            parent_quest_id,
            exec_type: 3,
        });

        // Phase 3: Content progress + post quests
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: *player_uid,
            content_type: QuestContent::QuestStateEqual,
            param: *sub_quest_id,
            param2: QuestState::Failed as u32,
            param3: 0,
            add_progress: 1,
        });
        quest_content_events.write(QuestContentProgressEvent {
            player_uid: *player_uid,
            content_type: QuestContent::QuestStateNotEqual,
            param: *sub_quest_id,
            param2: QuestState::Unfinished as u32,
            param3: 0,
            add_progress: 1,
        });

        trigger_post_quests(
            *player_uid,
            *sub_quest_id,
            parent_quest_id,
            &players,
            &mut quest_accept_events,
        );

        // Phase 4: Notifications
        {
            let Some(player_info) = players.get(*player_uid) else {
                continue;
            };

            if let Some(ref quest_bin) = player_info
                .quest_bin
                .as_ref()
                .and_then(|q| q.quest_bin.as_ref())
            {
                if let Some(quest_item) = quest_bin.quest_map.get(sub_quest_id) {
                    message_output.send(
                        *player_uid,
                        "QuestListUpdateNotify",
                        QuestListUpdateNotify {
                            quest_list: vec![quest_item.to_normal_proto()],
                        },
                    );
                }
            }

            if parent_just_failed && sub_quest_data.fail_parent {
                tracing::debug!("[Quest] Parent quest failed: parent={}", parent_quest_id);
            }
        }
    }
}

fn trigger_post_quests(
    player_uid: u32,
    finished_quest_id: u32,
    parent_quest_id: u32,
    players: &Players,
    quest_accept_event: &mut MessageWriter<QuestAcceptEvent>,
) {
    let quest_config_collection = quest_config::get_quest_config_collection();
    let sub_quest_config_collection = quest::quest_config::get_sub_quest_config_collection();

    let Some(player_info) = players.get(player_uid) else {
        return;
    };
    let Some(quest_config) = quest_config_collection.get(&parent_quest_id) else {
        return;
    };
    let Some(ref sub_quests) = quest_config.sub_quests else {
        return;
    };

    let existing_quests: std::collections::HashSet<u32> = player_info
        .quest_bin
        .as_ref()
        .and_then(|q| q.quest_bin.as_ref())
        .map(|q| q.quest_map.keys().copied().collect())
        .unwrap_or_default();

    for sub_quest in sub_quests {
        let sub_id = sub_quest.sub_id;
        if existing_quests.contains(&sub_id) {
            continue;
        }

        let Some(_sub_quest_config) = sub_quest_config_collection.get(&sub_id) else {
            continue;
        };

        let results: Vec<bool> = sub_quest
            .accept_cond
            .iter()
            .map(|cond| match cond.r#type {
                QuestCond::StateEqual => {
                    if cond.param.len() >= 2 {
                        let cond_quest_id = cond.param[0];
                        let expected_state = cond.param[1];
                        if cond_quest_id == finished_quest_id {
                            expected_state == QuestState::Finished as u32
                        } else if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin
                                .quest_map
                                .get(&cond_quest_id)
                                .map(|q| q.state == expected_state)
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                QuestCond::StateNotEqual => {
                    if cond.param.len() >= 2 {
                        let cond_quest_id = cond.param[0];
                        let unwanted_state = cond.param[1];
                        if cond_quest_id == finished_quest_id {
                            unwanted_state != QuestState::Finished as u32
                        } else if let Some(ref quest_bin) = player_info
                            .quest_bin
                            .as_ref()
                            .and_then(|q| q.quest_bin.as_ref())
                        {
                            quest_bin
                                .quest_map
                                .get(&cond_quest_id)
                                .map(|q| q.state != unwanted_state)
                                .unwrap_or(true)
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }
                QuestCond::QuestNotReceive => {
                    if !cond.param.is_empty() {
                        !existing_quests.contains(&cond.param[0])
                    } else {
                        true
                    }
                }
                QuestCond::PlayerLevelEqualGreater => {
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
                _ => true,
            })
            .collect();

        if evaluate_logic_comb(&sub_quest.accept_cond_comb, &results) {
            quest_accept_event.write(QuestAcceptEvent(player_uid, sub_id));
        }
    }
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::gm_util::QuestAction;
use common::time_util::unix_timestamp;
use nod_krai_gi_data::{excel, quest};
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::player_information::QuestItem;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::normal::{    AnecdoteAreaInfo, AnecdoteBriefInfo, AnecdoteConflictInfoReq, AnecdoteConflictInfoRsp,
    AnecdoteDataNotify, AnecdoteFinishNotify, AnecdoteFinishReq, AnecdoteFinishRsp, AnecdoteInfo,
    AnecdoteWishInfo, NpcTalkReq, NpcTalkRsp, Quest, QuestListUpdateNotify,
};
use std::default::Default;
use std::sync::Arc;

#[repr(u32)]
pub enum QuestState {
    QuestStateNone = 0,
    QuestStateUnstarted = 1,
    QuestStateUnfinished = 2,
    QuestStateFinished = 3,
    QuestStateFailed = 4,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_npc)
            .add_systems(Update, gm_command_handler)
            .add_systems(Update, quest_begin)
            .add_systems(Update, quest_finish)
            .add_systems(Update, quest_list_update);
    }
}

pub fn handle_npc(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        match message.message_name() {
            "NpcTalkReq" => {
                if let Some(req) = message.decode::<NpcTalkReq>() {
                    message_output.send(
                        message.sender_uid(),
                        "NpcTalkRsp",
                        NpcTalkRsp {
                            retcode: Retcode::RetSucc.into(),
                            cur_talk_id: req.talk_id,
                            npc_entity_id: req.npc_entity_id,
                            entity_id: req.entity_id,
                        },
                    );
                }
            }
            "AnecdoteDataReq" => {
                let anecdote_excel_config_collection_clone =
                    std::sync::Arc::clone(excel::anecdote_excel_config_collection::get());
                message_output.send(
                    message.sender_uid(),
                    "AnecdoteDataNotify",
                    AnecdoteDataNotify {
                        anecdote_area_info: Some(AnecdoteAreaInfo {
                            unlock_area_list: vec![1, 2, 3, 4, 5, 6, 7],
                        }),
                        anecdote_wish_info: Some(AnecdoteWishInfo {
                            cur_wished_npc_id: 0,
                            last_wish_time: 1757531084,
                        }),
                        anecdote_info_list: anecdote_excel_config_collection_clone
                            .iter()
                            .map(|(id, _)| AnecdoteInfo {
                                finish_count: 0,
                                state: 2,
                                id: *id,
                            })
                            .collect(),
                        ..Default::default()
                    },
                );
            }
            "AnecdoteConflictInfoReq" => {
                if let Some(req) = message.decode::<AnecdoteConflictInfoReq>() {
                    message_output.send(
                        message.sender_uid(),
                        "AnecdoteConflictInfoRsp",
                        AnecdoteConflictInfoRsp {
                            npc_id: req.npc_id,
                            ..Default::default()
                        },
                    );
                }
            }
            "AnecdoteFinishReq" => {
                if let Some(req) = message.decode::<AnecdoteFinishReq>() {
                    message_output.send(
                        message.sender_uid(),
                        "AnecdoteFinishNotify",
                        AnecdoteFinishNotify {
                            anecdode_brief_info_list: vec![AnecdoteBriefInfo {
                                is_new_anecdote: true,
                                anecdote_id: req.anecdote_id,
                            }],
                        },
                    );
                    message_output.send(
                        message.sender_uid(),
                        "AnecdoteFinishRsp",
                        AnecdoteFinishRsp {
                            anecdote_id: req.anecdote_id,
                            ..Default::default()
                        },
                    );
                }
            }
            &_ => {}
        }
    }
}

pub fn gm_command_handler(
    mut events: MessageReader<CommandQuestEvent>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut quest_begin_event: MessageWriter<QuestBeginEvent>,
    mut quest_finish_event: MessageWriter<QuestFinishEvent>,
) {
    let sub_quest_config_collection_clone = Arc::clone(
        quest::quest_config::SUB_QUEST_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );
    for CommandQuestEvent(player_uid, action) in events.read() {
        let result;
        match action {
            QuestAction::Begin { id } => match sub_quest_config_collection_clone.get(id) {
                None => {
                    result = "unknown quest id".to_string();
                }
                Some(_) => {
                    result = format!("begin quest {}", id);
                    quest_begin_event.write(QuestBeginEvent(*player_uid, *id));
                }
            },
            QuestAction::Finish { id } => match sub_quest_config_collection_clone.get(id) {
                None => {
                    result = "unknown quest id".to_string();
                }
                Some(_) => {
                    result = format!("finish quest {}", id);
                    quest_finish_event.write(QuestFinishEvent(*player_uid, *id));
                }
            },
        }
        gm_notify_events.write(ConsoleChatNotifyEvent(
            *player_uid,
            format!("result:{}", result),
        ));
    }
}

pub fn quest_begin(
    mut events: MessageReader<QuestBeginEvent>,
    mut quest_list_update_event: MessageWriter<QuestListUpdateEvent>,
    mut players: ResMut<Players>,
) {
    let sub_quest_config_collection_clone = Arc::clone(
        quest::quest_config::SUB_QUEST_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );
    for QuestBeginEvent(player_uid, sub_quest_id) in events.read() {
        match sub_quest_config_collection_clone.get(sub_quest_id) {
            None => {
                continue;
            }
            Some(sub_quest_data) => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                player_info.quest_bin.quest_map.insert(
                    *sub_quest_id,
                    QuestItem {
                        parent_quest_id: sub_quest_data.main_id,
                        state: QuestState::QuestStateUnfinished as u32,
                        start_time: unix_timestamp() as u32,
                        accept_time: unix_timestamp() as u32,
                        finish_time: 0,
                        finish_progress_list: {
                            if sub_quest_data.finish_cond.is_empty() {
                                vec![0u32; 10]
                            } else {
                                vec![0u32; sub_quest_data.finish_cond.len()]
                            }
                        },
                        fail_progress_list: vec![0u32; sub_quest_data.fail_cond.len()],
                    },
                );

                quest_list_update_event.write(QuestListUpdateEvent(*player_uid, *sub_quest_id));
            }
        }
    }
}

pub fn quest_finish(
    mut events: MessageReader<QuestFinishEvent>,
    mut quest_list_update_event: MessageWriter<QuestListUpdateEvent>,
    mut players: ResMut<Players>,
) {
    let sub_quest_config_collection_clone = Arc::clone(
        quest::quest_config::SUB_QUEST_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );
    for QuestFinishEvent(player_uid, sub_quest_id) in events.read() {
        match sub_quest_config_collection_clone.get(sub_quest_id) {
            None => {
                continue;
            }
            Some(sub_quest_data) => {
                let Some(player_info) = players.get_mut(*player_uid) else {
                    continue;
                };
                player_info.quest_bin.quest_map.insert(
                    *sub_quest_id,
                    QuestItem {
                        parent_quest_id: sub_quest_data.main_id,
                        state: QuestState::QuestStateFinished as u32,
                        start_time: unix_timestamp() as u32,
                        accept_time: unix_timestamp() as u32,
                        finish_time: 0,
                        finish_progress_list: {
                            if sub_quest_data.finish_cond.is_empty() {
                                vec![0u32; 10]
                            } else {
                                vec![0u32; sub_quest_data.finish_cond.len()]
                            }
                        },
                        fail_progress_list: vec![0u32; sub_quest_data.fail_cond.len()],
                    },
                );

                quest_list_update_event.write(QuestListUpdateEvent(*player_uid, *sub_quest_id));
            }
        }
    }
}
pub fn quest_list_update(
    mut events: MessageReader<QuestListUpdateEvent>,
    message_output: Res<MessageOutput>,
    players: Res<Players>,
) {
    for QuestListUpdateEvent(player_uid, sub_quest_id) in events.read() {
        let Some(player_info) = players.get(*player_uid) else {
            continue;
        };
        match player_info
            .quest_bin
            .quest_map
            .get(sub_quest_id)
        {
            None => {
                continue;
            }
            Some(quest_item) => {
                message_output.send(
                    *player_uid,
                    "QuestListUpdateNotify",
                    QuestListUpdateNotify {
                        quest_list: vec![Quest {
                            quest_id: *sub_quest_id,
                            parent_quest_id: quest_item.parent_quest_id,
                            state: quest_item.state,
                            start_time: quest_item.start_time,
                            accept_time: quest_item.accept_time,
                            start_game_time: 438,
                            finish_progress_list: quest_item.finish_progress_list.clone(),
                            fail_progress_list: quest_item.fail_progress_list.clone(),
                            ..Default::default()
                        }],
                    },
                );
            }
        }
    }
}

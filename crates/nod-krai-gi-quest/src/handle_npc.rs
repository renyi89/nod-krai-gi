use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel;
use nod_krai_gi_data::quest::quest_config::QuestContent;
use nod_krai_gi_event::quest::QuestContentProgressEvent;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::{
    AnecdoteAreaInfo, AnecdoteConflictInfoReq, AnecdoteConflictInfoRsp, AnecdoteFinishInfo,
    AnecdoteFinishNotify, AnecdoteFinishReq, AnecdoteFinishRsp, AnecdoteGetDataRsp, AnecdoteInfo,
    AnecdoteWishInfo, NpcTalkReq, NpcTalkRsp,
};
use nod_krai_gi_proto::retcode::Retcode;

pub fn handle_npc(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "NpcTalkReq" => {
                if let Some(req) = message.decode::<NpcTalkReq>() {
                    let uid = message.sender_uid();
                    message_output.send(
                        uid,
                        "NpcTalkRsp",
                        NpcTalkRsp {
                            retcode: Retcode::RetSucc.into(),
                            cur_talk_id: req.talk_id,
                            npc_entity_id: req.npc_entity_id,
                            entity_id: req.entity_id,
                        },
                    );

                    quest_content_events.write(QuestContentProgressEvent {
                        player_uid: uid,
                        content_type: QuestContent::CompleteTalk,
                        param: req.talk_id,
                        param2: req.npc_entity_id,
                        param3: 0,
                        add_progress: 1,
                    });

                    quest_content_events.write(QuestContentProgressEvent {
                        player_uid: uid,
                        content_type: QuestContent::CompleteAnyTalk,
                        param: req.talk_id,
                        param2: req.npc_entity_id,
                        param3: 0,
                        add_progress: 1,
                    });
                }
            }
            "AnecdoteGetDataReq" => {
                let anecdote_excel_config_collection_clone =
                    std::sync::Arc::clone(excel::anecdote_excel_config_collection::get());
                message_output.send(
                    message.sender_uid(),
                    "AnecdoteGetDataRsp",
                    AnecdoteGetDataRsp {
                        anecdote_area_info: Some(AnecdoteAreaInfo {
                            unlock_area_list: vec![1, 2, 3, 4, 5, 6, 7],
                        }),
                        anecdote_wish_info: Some(AnecdoteWishInfo {
                            cur_wished_npc_id: 0,
                            last_wish_time: 0,
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
                            anecdode_finish_info_list: vec![AnecdoteFinishInfo {
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

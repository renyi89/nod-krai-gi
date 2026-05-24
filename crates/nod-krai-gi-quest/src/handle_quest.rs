use bevy_ecs::prelude::*;
use nod_krai_gi_data::quest::quest_config::QuestContent;
use nod_krai_gi_event::quest::*;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    AddQuestContentProgressReq, AddQuestContentProgressRsp, QuestUpdateQuestVarNotify,
    QuestUpdateQuestVarReq, QuestUpdateQuestVarRsp,
};
use nod_krai_gi_proto::retcode::Retcode;

use crate::quest_state::ensure_player_quest_comp;

pub fn handle_quest_request(
    mut events: MessageReader<ClientMessageEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut quest_content_progress_events: MessageWriter<QuestContentProgressEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "AddQuestContentProgressReq" => {
                if let Some(req) = message.decode::<AddQuestContentProgressReq>() {
                    let uid = message.sender_uid();

                    if players.get_mut(uid).is_none() {
                        message_output.send(
                            uid,
                            "AddQuestContentProgressRsp",
                            AddQuestContentProgressRsp {
                                content_type: req.content_type,
                                retcode: Retcode::RetFail.into(),
                            },
                        );
                        continue;
                    }

                    // 写入 QuestContentProgressEvent，由 quest_content_progress 系统统一处理进度更新
                    quest_content_progress_events.write(QuestContentProgressEvent {
                        player_uid: uid,
                        content_type: QuestContent::from_u32(req.content_type)
                            .unwrap_or(QuestContent::None),
                        param: req.param,
                        param2: 0,
                        param3: 0,
                        add_progress: req.add_progress,
                    });

                    message_output.send(
                        uid,
                        "AddQuestContentProgressRsp",
                        AddQuestContentProgressRsp {
                            content_type: req.content_type,
                            retcode: Retcode::RetSucc.into(),
                        },
                    );
                }
            }
            "QuestUpdateQuestVarReq" => {
                if let Some(req) = message.decode::<QuestUpdateQuestVarReq>() {
                    let uid = message.sender_uid();

                    let Some(player_info) = players.get_mut(uid) else {
                        message_output.send(
                            uid,
                            "QuestUpdateQuestVarRsp",
                            QuestUpdateQuestVarRsp {
                                parent_quest_id: req.parent_quest_id,
                                quest_id: req.quest_id,
                                retcode: Retcode::RetFail.into(),
                                ..Default::default()
                            },
                        );
                        continue;
                    };
                    let player_quest_bin = ensure_player_quest_comp(player_info);

                    let Some(ref mut parent_quest_bin) = player_quest_bin.parent_quest_bin else {
                        message_output.send(
                            uid,
                            "QuestUpdateQuestVarRsp",
                            QuestUpdateQuestVarRsp {
                                parent_quest_id: req.parent_quest_id,
                                quest_id: req.quest_id,
                                retcode: Retcode::RetFail.into(),
                                ..Default::default()
                            },
                        );
                        continue;
                    };

                    if let Some(parent_quest) = parent_quest_bin
                        .parent_quest_map
                        .get_mut(&req.parent_quest_id)
                    {
                        for var_op in &req.quest_var_op_list {
                            let idx = var_op.index as usize;
                            while parent_quest.quest_var.len() <= idx {
                                parent_quest.quest_var.push(0);
                            }
                            if var_op.is_add {
                                parent_quest.quest_var[idx] += var_op.value;
                            } else {
                                parent_quest.quest_var[idx] = var_op.value;
                            }
                        }

                        message_output.send(
                            uid,
                            "QuestUpdateQuestVarNotify",
                            QuestUpdateQuestVarNotify {
                                parent_quest_id: req.parent_quest_id,
                                quest_var: parent_quest.quest_var.clone(),
                                ..Default::default()
                            },
                        );

                        message_output.send(
                            uid,
                            "QuestUpdateQuestVarRsp",
                            QuestUpdateQuestVarRsp {
                                parent_quest_id: req.parent_quest_id,
                                quest_id: req.quest_id,
                                retcode: Retcode::RetSucc.into(),
                                ..Default::default()
                            },
                        );
                    } else {
                        message_output.send(
                            uid,
                            "QuestUpdateQuestVarRsp",
                            QuestUpdateQuestVarRsp {
                                parent_quest_id: req.parent_quest_id,
                                quest_id: req.quest_id,
                                retcode: Retcode::RetFail.into(),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            &_ => {}
        }
    }
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::{AnecdoteInfo, AnecdoteAreaInfo, AnecdoteDataNotify, NpcTalkReq, NpcTalkRsp};

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_npc);
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
                let avatarnecdote_excel_config_collection_clone =
                    std::sync::Arc::clone(excel::anecdote_excel_config_collection::get());
                message_output.send(
                    message.sender_uid(),
                    "AnecdoteDataNotify",
                    AnecdoteDataNotify {
                        retcode: Retcode::RetSucc.into(),
                        anecdote_area_info: Some(AnecdoteAreaInfo {
                            unlock_area_list: vec![1, 2, 3, 4, 5, 6, 7],
                        }),
                        anecdote_info_list: avatarnecdote_excel_config_collection_clone
                            .iter()
                            .map(|(id, _)| AnecdoteInfo {
                                finish_count: 1,
                                state: 2,
                                id: *id,
                            })
                            .collect(),
                        ..Default::default()
                    },
                );
            }
            &_ => {}
        }
    }
}

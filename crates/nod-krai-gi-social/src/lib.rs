use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::time_util::unix_timestamp;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::retcode::Retcode;
use nod_krai_gi_proto::{
    ChatInfo, FriendBrief, GetPlayerFriendListRsp, PrivateChatNotify, PrivateChatReq,
    ProfilePicture,
};

pub struct SocialPlugin;

impl Plugin for SocialPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ConsoleChatEvent>()
            .add_systems(Update, handle_chat);
    }
}

#[derive(Message)]
pub struct ConsoleChatEvent(pub u32, pub String);

pub fn handle_chat(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut console_chat_event: MessageWriter<ConsoleChatEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "GetPlayerFriendListReq" => {
                message_output.send(
                    message.sender_uid(),
                    "GetPlayerFriendListRsp",
                    GetPlayerFriendListRsp {
                        ask_friend_list: vec![],
                        friend_list: vec![FriendBrief {
                            uid: 123,
                            nickname: "Console".to_string(),
                            level: 60,
                            world_level: 9,
                            signature: "这是签名 this's signature".to_string(),
                            name_card_id: 210248,
                            profile_picture: Some(ProfilePicture {
                                profile_picture_id: 10100,
                                profile_frame_id: 100014,
                                ..Default::default()
                            }),
                            is_game_source: true,
                            online_state: 1,
                            platform_type: 3,
                            ..Default::default()
                        }],
                        retcode: Retcode::RetSucc.into(),
                    },
                );
            }
            "PrivateChatReq" => {
                if let Some(req) = message.decode::<PrivateChatReq>() {
                    if req.target_uid == 123 {
                        let recv_text;
                        match req.content {
                            Some(nod_krai_gi_proto::private_chat_req::Content::Text(text)) => {
                                recv_text = text.clone();
                            }
                            _ => {
                                continue;
                            }
                        }
                        console_chat_event
                            .write(ConsoleChatEvent(message.sender_uid(), recv_text.clone()));
                        message_output.send(
                            message.sender_uid(),
                            "PrivateChatNotify",
                            PrivateChatNotify {
                                chat_info: Some(ChatInfo {
                                    time: unix_timestamp() as u32,
                                    to_uid: message.sender_uid(),
                                    uid: req.target_uid,
                                    content: Some(nod_krai_gi_proto::chat_info::Content::Text(
                                        format!("console: {}", recv_text),
                                    )),
                                    ..Default::default()
                                }),
                            },
                        );
                    }
                    message_output.send_none(message.sender_uid(), "PrivateChatRsp");
                }
            }
            &_ => {}
        }
    }
}

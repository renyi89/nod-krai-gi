use crate::common::PlayerSceneStates;
use bevy_ecs::prelude::*;
use nod_krai_gi_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    common::Visible,
};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_proto::dy_parser::replace_out_u32;
use nod_krai_gi_proto::{retcode::Retcode, EnterSceneDoneRsp};

#[derive(Message)]
pub struct EnterSceneDoneEvent(pub u32);

pub fn on_enter_scene_done(
    mut commands: Commands,
    mut reader: MessageReader<EnterSceneDoneEvent>,
    avatars: Query<(Entity, AvatarQueryReadOnly), With<CurrentPlayerAvatarMarker>>,
) {
    for event in reader.read() {
        let uid = event.0;

        let (cur_player_avatar, _) = avatars
            .iter()
            .find(|(_, data)| data.owner_player_uid.0 == uid)
            .unwrap();

        commands.entity(cur_player_avatar).insert(Visible);
    }
}

pub fn enter_scene_done_send_rsp(
    mut enter_scene_done_events: MessageReader<EnterSceneDoneEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    message_output: Res<MessageOutput>,
) {
    for event in enter_scene_done_events.read() {
        let uid = event.0;

        let version = get_player_version!(&uid);
        let protocol_version = version.as_str();

        message_output.send(
            uid,
            "EnterSceneDoneRsp",
            EnterSceneDoneRsp {
                retcode: Retcode::RetSucc.into(),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "EnterSceneDoneRsp.enter_scene_token",
                    player_scene_states.get(&uid).unwrap().enter_scene_token(),
                ),
            },
        );
    }
}

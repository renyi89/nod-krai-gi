use crate::common::PlayerSceneStates;
use bevy_ecs::prelude::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::replace_out_u32;

#[derive(Message)]
pub struct PostEnterSceneEvent(pub u32);

pub fn on_post_enter_scene(
    mut reader: MessageReader<PostEnterSceneEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    mut players: ResMut<Players>,
    out: Res<MessageOutput>,
) {
    for PostEnterSceneEvent(uid) in reader.read() {
        let player_info =  players.get_mut(*uid);
        player_info.cache.is_tp = false;

        let version = get_player_version!(uid);
        let protocol_version = version.as_str();

        out.send(
            *uid,
            "PostEnterSceneRsp",
            nod_krai_gi_proto::PostEnterSceneRsp {
                retcode: nod_krai_gi_proto::retcode::Retcode::RetSucc.into(),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "PostEnterSceneRsp.enter_scene_token",
                    player_scene_states.get(uid).unwrap().enter_scene_token(),
                ),
            },
        );
    }
}

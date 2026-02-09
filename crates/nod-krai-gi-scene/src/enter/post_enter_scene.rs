use crate::common::PlayerSceneStates;
use bevy_ecs::prelude::*;
use common::game_server_config::cache_set_is_tp;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::dy_parser::replace_out_u32;

pub fn on_post_enter_scene(
    mut reader: MessageReader<PostEnterSceneEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    out: Res<MessageOutput>,
) {
    for PostEnterSceneEvent(uid) in reader.read() {
        cache_set_is_tp(*uid, false);

        let version = get_player_version!(uid);
        let protocol_version = version.as_str();

        let Some(player_scene_state) = player_scene_states.get(&uid) else {
            continue;
        };

        out.send(
            *uid,
            "PostEnterSceneRsp",
            nod_krai_gi_proto::normal::PostEnterSceneRsp {
                retcode: nod_krai_gi_proto::retcode::Retcode::RetSucc.into(),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "PostEnterSceneRsp.enter_scene_token",
                    player_scene_state.enter_scene_token(),
                ),
            },
        );
    }
}

use crate::{common::PlayerSceneStates, player_join_team::PlayerJoinTeamEvent};
use bevy_ecs::prelude::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::replace_out_u32;
use nod_krai_gi_proto::{retcode::Retcode, SceneInitFinishRsp};

#[derive(Message)]
pub struct SceneInitFinishEvent(pub u32);

pub fn on_scene_init_finish(
    mut reader: MessageReader<SceneInitFinishEvent>,
    mut players: ResMut<Players>,
    mut join_team_events: MessageWriter<PlayerJoinTeamEvent>,
) {
    for event in reader.read() {
        let uid = event.0;
        let player_info = players.get_mut(uid);

        if player_info.avatar_module.cur_avatar_guid_list.is_empty() {
            player_info.avatar_module.cur_avatar_guid_list = player_info
                .avatar_module
                .team_map
                .get(&player_info.avatar_module.cur_avatar_team_id)
                .unwrap()
                .avatar_guid_list
                .clone();
        }

        let appear_avatar_guid = {
            if !player_info
                .avatar_module
                .cur_avatar_guid_list
                .contains(&player_info.avatar_module.cur_avatar_guid)
            {
                player_info.avatar_module.cur_avatar_guid = player_info
                    .avatar_module
                    .cur_avatar_guid_list
                    .first()
                    .copied()
                    .unwrap();
            }
            player_info.avatar_module.cur_avatar_guid
        };

        join_team_events.write(PlayerJoinTeamEvent {
            player_uid: uid,
            avatar_guid_list: player_info.avatar_module.cur_avatar_guid_list.clone(),
            appear_avatar_guid,
        });
    }
}

pub fn scene_init_finish_send_rsp(
    mut scene_init_finish_events: MessageReader<SceneInitFinishEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    message_output: Res<MessageOutput>,
    mut lua_shell_events: MessageWriter<nod_krai_gi_luashell::LuaShellEvent>,
) {
    for event in scene_init_finish_events.read() {
        let uid = event.0;
        let version = get_player_version!(&uid);
        let protocol_version = version.as_str();

        message_output.send(
            uid,
            "SceneInitFinishRsp",
            SceneInitFinishRsp {
                retcode: Retcode::RetSucc.into(),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "SceneInitFinishRsp.enter_scene_token",
                    player_scene_states.get(&uid).unwrap().enter_scene_token(),
                ),
            },
        );

        lua_shell_events.write(nod_krai_gi_luashell::LuaShellEvent());
    }
}

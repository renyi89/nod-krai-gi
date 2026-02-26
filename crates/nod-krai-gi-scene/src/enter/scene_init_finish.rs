use crate::common::PlayerSceneStates;
use bevy_ecs::prelude::*;
use common::language::Language;
use common::player_cache::{cache_get_is_notify, cache_get_language, cache_set_is_notify};
use nod_krai_gi_event::combat::PlayerMoveEvent;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::replace_out_u32;
use nod_krai_gi_proto::normal::{AntiAddictNotify, SceneInitFinishRsp};
use nod_krai_gi_proto::retcode::Retcode;

static MSG_CHS: &[u8] = &[
    0x6E, 0x6F, 0x64, 0x2D, 0x6B, 0x72, 0x61, 0x69, 0x2D, 0x67, 0x69, 0x20, 0xE6, 0x98, 0xAF, 0xE5,
    0x85, 0x8D, 0xE8, 0xB4, 0xB9, 0xE5, 0xBC, 0x80, 0xE6, 0xBA, 0x90, 0xE8, 0xBD, 0xAF, 0xE4, 0xBB,
    0xB6, 0xEF, 0xBC, 0x8C, 0xE9, 0x81, 0xB5, 0xE5, 0xBE, 0xAA, 0x20, 0x41, 0x47, 0x50, 0x4C, 0x2D,
    0x33, 0x2E, 0x30, 0x20, 0x6C, 0x69, 0x63, 0x65, 0x6E, 0x73, 0x65, 0xE3, 0x80, 0x82, 0xE5, 0xA6,
    0x82, 0xE6, 0x9E, 0x9C, 0xE4, 0xBD, 0xA0, 0xE6, 0x98, 0xAF, 0xE4, 0xBB, 0x98, 0xE8, 0xB4, 0xB9,
    0xE8, 0xB4, 0xAD, 0xE4, 0xB9, 0xB0, 0xE7, 0x9A, 0x84, 0xEF, 0xBC, 0x8C, 0xE9, 0x82, 0xA3, 0xE4,
    0xBD, 0xA0, 0xE5, 0xB7, 0xB2, 0xE7, 0xBB, 0x8F, 0xE8, 0xA2, 0xAB, 0xE9, 0xAA, 0x97, 0xE4, 0xBA,
    0x86, 0xE3, 0x80, 0x82,
];

static MSG_EN: &[u8] = &[
    0x6E, 0x6F, 0x64, 0x2D, 0x6B, 0x72, 0x61, 0x69, 0x2D, 0x67, 0x69, 0x20, 0x69, 0x73, 0x20, 0x46,
    0x52, 0x45, 0x45, 0x20, 0x73, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2E, 0x20, 0x49, 0x66,
    0x20, 0x79, 0x6F, 0x75, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x70, 0x61, 0x69, 0x64, 0x20, 0x66,
    0x6F, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x2C, 0x20, 0x79, 0x6F, 0x75, 0x20, 0x6D, 0x61, 0x79,
    0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x73, 0x63, 0x61, 0x6D, 0x6D,
    0x65, 0x64, 0x2E,
];

pub fn on_scene_init_finish(
    mut reader: MessageReader<SceneInitFinishEvent>,
    mut players: ResMut<Players>,
    mut move_events: MessageWriter<PlayerMoveEvent>,
    mut join_team_events: MessageWriter<PlayerJoinTeamEvent>,
) {
    for event in reader.read() {
        let uid = event.0;
        let Some(player_info) = players.get_mut(uid) else {
            continue;
        };
        if player_info.dungeon_bin.is_some() {
            if let Some(ref player_scene_bin) = player_info.scene_bin {
                if let Some(ref pos) = player_scene_bin.my_cur_scene_pos {
                    move_events.write(PlayerMoveEvent(
                        uid,
                        player_scene_bin.my_cur_scene_id,
                        (pos.x, pos.y, pos.z),
                        true,
                    ));
                }
            }
        };
        let Some(ref mut player_avatar_bin) = player_info.avatar_bin else {
            continue;
        };

        let Some(team_info) = player_avatar_bin
            .team_map
            .get(&player_avatar_bin.cur_team_id)
        else {
            tracing::debug!("team_info {} doesn't exist", player_avatar_bin.cur_team_id);
            continue;
        };

        if player_avatar_bin.cur_avatar_guid_list.is_empty() {
            player_avatar_bin.cur_avatar_guid_list = team_info.avatar_guid_list.clone();
        }

        if player_avatar_bin.cur_avatar_guid_list.is_empty() {
            tracing::debug!("cur_avatar_guid_list is_empty");
            continue;
        }

        let appear_avatar_guid = {
            if !player_avatar_bin
                .cur_avatar_guid_list
                .contains(&player_avatar_bin.cur_avatar_guid)
            {
                player_avatar_bin.cur_avatar_guid = player_avatar_bin
                    .cur_avatar_guid_list
                    .first()
                    .copied()
                    .unwrap_or_default();
            }
            player_avatar_bin.cur_avatar_guid
        };

        join_team_events.write(PlayerJoinTeamEvent {
            player_uid: uid,
            avatar_guid_list: player_avatar_bin.cur_avatar_guid_list.clone(),
            appear_avatar_guid,
        });
    }
}

pub fn scene_init_finish_send_rsp(
    mut scene_init_finish_events: MessageReader<SceneInitFinishEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    message_output: Res<MessageOutput>,
    mut lua_shell_events: MessageWriter<nod_krai_gi_event::luashell::LuaShellEvent>,
) {
    for event in scene_init_finish_events.read() {
        let uid = event.0;
        let version = get_player_version!(&uid);
        let protocol_version = version.as_str();

        let Some(player_scene_state) = player_scene_states.get(&uid) else {
            continue;
        };

        message_output.send(
            uid,
            "SceneInitFinishRsp",
            SceneInitFinishRsp {
                retcode: Retcode::RetSucc.into(),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "SceneInitFinishRsp.enter_scene_token",
                    player_scene_state.enter_scene_token(),
                ),
            },
        );
        if !cache_get_is_notify(uid).unwrap_or_default() {
            cache_set_is_notify(uid, true);
            let language = cache_get_language(uid).unwrap_or(Language::Chs);
            let msg;
            if language == Language::Chs || language == Language::Cht {
                msg = String::from_utf8(MSG_CHS.to_vec()).unwrap();
            } else {
                msg = String::from_utf8(MSG_EN.to_vec()).unwrap();
            }
            message_output.send(
                uid,
                "AntiAddictNotify",
                AntiAddictNotify {
                    msg,
                    msg_type: 1,
                    ..Default::default()
                },
            );
        }

        lua_shell_events.write(nod_krai_gi_event::luashell::LuaShellEvent());
    }
}

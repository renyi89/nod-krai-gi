use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use common::game_server_config::{cache_get_client_time, cache_get_is_pause, cache_set_is_pause};
use common::time_util;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::{
    retcode::Retcode, ClientSetGameTimeReq, ClientSetGameTimeRsp, PlayerGameTimeNotify,
    PlayerSetPauseReq, PlayerSetPauseRsp, PlayerTimeNotify, ServerTimeNotify,
};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneTime::default())
            .add_systems(Startup, init_scene_time)
            .add_systems(PreUpdate, set_pause)
            .add_systems(PreUpdate, client_set_game_time)
            .add_systems(First, sync_scene_time_on_scene_init_finish)
            // .add_systems(First, sync_scene_time_on_enter_scene_done)
        ;
    }
}

#[derive(Resource, Default)]
pub struct SceneTime {
    pub scene_time: u64,
    pub game_time: u32,
}

pub fn init_scene_time(mut time: ResMut<SceneTime>) {
    time.game_time = (7.31 * 60.0) as u32;
}

pub fn set_pause(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        match message.message_name() {
            "PlayerSetPauseReq" => {
                if let Some(request) = message.decode::<PlayerSetPauseReq>() {
                    let uid = message.sender_uid();
                    cache_set_is_pause(uid, request.is_paused);
                    message_output.send(uid, "PlayerSetPauseRsp", PlayerSetPauseRsp { retcode: 0 });
                }
            }
            &_ => {}
        }
    }
}

pub fn client_set_game_time(
    mut events: MessageReader<ClientMessageEvent>,
    players: Res<Players>,
    mut time: ResMut<SceneTime>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        match message.message_name() {
            "ClientSetGameTimeReq" => {
                if let Some(request) = message.decode::<ClientSetGameTimeReq>() {
                    let uid = message.sender_uid();
                    let Some(player_info) = players.get(uid) else {
                        continue;
                    };
                    let mut rsp = ClientSetGameTimeRsp::default();

                    if player_info.basic_module.is_game_time_locked {
                        tracing::debug!("game time is locked, uid: {uid}");
                        rsp.retcode = Retcode::RetPlayerTimeLocked.into();
                    } else {
                        tracing::debug!("set game time to {}, uid: {uid}", request.game_time);

                        rsp.game_time = request.game_time;
                        rsp.client_game_time = request.client_game_time;
                        time.game_time = request.game_time;

                        message_output.send_to_all(
                            "PlayerGameTimeNotify",
                            PlayerGameTimeNotify {
                                uid,
                                is_home: false,
                                game_time: time.game_time,
                            },
                        );
                    }

                    message_output.send(uid, "ClientSetGameTimeRsp", rsp);
                }
            }
            &_ => {}
        }
    }
}

pub fn sync_scene_time_on_scene_init_finish(
    mut events: MessageReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    players: Res<Players>,
    time: Res<SceneTime>,
) {
    use nod_krai_gi_proto::{PlayerGameTimeNotify, SceneTimeNotify};

    for SceneInitFinishEvent(uid) in events.read() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };

        message_output.send(
            *uid,
            "ServerTimeNotify",
            ServerTimeNotify {
                server_time: time_util::unix_timestamp_ms(),
            },
        );

        message_output.send(
            *uid,
            "SceneTimeNotify",
            SceneTimeNotify {
                is_paused: cache_get_is_pause(*uid).unwrap_or_default(),
                scene_id: player_info.world_position.scene_id,
                scene_time: time.scene_time,
            },
        );

        message_output.send(
            *uid,
            "PlayerGameTimeNotify",
            PlayerGameTimeNotify {
                is_home: false,
                uid: *uid,
                game_time: time.game_time,
            },
        )
    }
}

pub fn sync_scene_time_on_enter_scene_done(
    mut events: MessageReader<EnterSceneDoneEvent>,
    message_output: Res<MessageOutput>,
) {
    for EnterSceneDoneEvent(uid) in events.read() {
        message_output.send(
            *uid,
            "PlayerTimeNotify",
            PlayerTimeNotify {
                is_paused: cache_get_is_pause(*uid).unwrap_or_default(),
                server_time: time_util::unix_timestamp_ms(),
                player_time: cache_get_client_time(*uid).unwrap_or_default() as u64,
            },
        );
    }
}

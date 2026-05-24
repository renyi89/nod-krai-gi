use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::gm_util::{parse_command, Command, GachaAction, TpAction};
use common::player_cache::cache_get_is_tp;
use common::time_util::unix_timestamp;
use nod_krai_gi_entity::common::{EntityCounter, Visible};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_entity::monster::spawn_monster_entity;
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{ChatInfo, PrivateChatNotify, SceneAreaWeatherNotify};
use nod_krai_gi_proto::server_only::{GachaBin, VectorBin};
use rand::RngCore;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_command_handler)
            .add_systems(Update, gm_command_handler)
            .add_systems(Update, gm_talk_notify);
    }
}

pub fn debug_command_handler(
    mut events: MessageReader<DebugCommandEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    players: Res<Players>,
    mut jump_events: MessageWriter<ScenePlayerJumpEvent>,
    world_version_config: Res<WorldVersionConfig>,
) {
    for command in events.read() {
        tracing::debug!(
            "executor_uid: {}, kind: {:?}",
            command.executor_uid,
            command.kind
        );

        let Some(player_info) = players.get(command.executor_uid) else {
            continue;
        };

        match command.kind {
            CommandKind::QuickSpawnMonster {
                monster_id,
                position,
            } => {
                let monster_id = monster_id.unwrap_or_else(|| {
                    [20010101, 20010302, 20010502, 20010803, 20011002]
                        [rand::thread_rng().next_u32() as usize % 5]
                });

                let Some(monster_entity) = spawn_monster_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    {
                        let y = if let Some(ref player_scene_bin) = player_info.scene_bin {
                            player_scene_bin.my_cur_scene_pos.unwrap_or_default().y
                        } else {
                            0.0
                        };
                        (position.0, y + 4.0, position.1)
                    }
                    .into(),
                    VectorBin::default(),
                    monster_id,
                    90,
                    0,
                    0,
                    None,
                    0,
                ) else {
                    continue;
                };
                commands.entity(monster_entity.1).insert(Visible);
            }
            CommandKind::QuickSpawnGadget {
                gadget_id,
                position,
            } => {
                let gadget_id = gadget_id.unwrap_or_else(|| {
                    [70801015, 70801016, 70801017, 70801018, 70801019, 70801020]
                        [rand::thread_rng().next_u32() as usize % 5]
                });

                let Some(gadget_entity) = spawn_gadget_entity(
                    world_version_config.protocol_version.clone(),
                    &mut commands,
                    &mut entity_counter,
                    {
                        let y = if let Some(ref player_scene_bin) = player_info.scene_bin {
                            player_scene_bin.my_cur_scene_pos.unwrap_or_default().y
                        } else {
                            0.0
                        };
                        (position.0, y, position.1)
                    }
                    .into(),
                    VectorBin::default(),
                    gadget_id,
                    90,
                    true,
                    None,
                    None,
                    0,
                    0,
                ) else {
                    continue;
                };

                commands.entity(gadget_entity.1).insert(Visible);
            }
            CommandKind::QuickTravel { scene_id, position } => match scene_id {
                None => {}
                Some(scene_id) => {
                    jump_events.write(ScenePlayerJumpEvent(
                        command.executor_uid,
                        scene_id,
                        EnterReason::TransPoint,
                        (position.0, position.1.unwrap_or(2600.0), position.2),
                    ));
                }
            },
        }
    }
}

pub fn gm_command_handler(
    mut events: MessageReader<ConsoleChatReqEvent>,
    mut players: ResMut<Players>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut gm_command_events: MessageWriter<GmCommandEvent>,
    message_output: Res<MessageOutput>,
    mut tp_events: MessageWriter<ScenePlayerJumpEvent>,
    mut enter_dungeon_events: MessageWriter<ScenePlayerEnterDungeonEvent>,
) {
    for ConsoleChatReqEvent(player_uid, console_content) in events.read() {
        let Some(player_info) = players.get_mut(*player_uid) else {
            continue;
        };
        if cache_get_is_tp(*player_uid).unwrap_or(true) {
            continue;
        }
        let result = parse_command(console_content);
        match result {
            Ok(gm) => {
                tracing::debug!("gm_command_handler result: {:?}", gm);
                match &gm {
                    Command::Weather(id) => {
                        message_output.send(
                            *player_uid,
                            "SceneAreaWeatherNotify",
                            SceneAreaWeatherNotify {
                                weather_area_id: *id,
                                ..Default::default()
                            },
                        );
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            format!("set weather to {}", id),
                        ));
                    }
                    Command::Climate(id) => {
                        message_output.send(
                            *player_uid,
                            "SceneAreaWeatherNotify",
                            SceneAreaWeatherNotify {
                                climate_type: *id,
                                ..Default::default()
                            },
                        );
                        gm_notify_events.write(ConsoleChatNotifyEvent(
                            *player_uid,
                            format!("set climate to {}", id),
                        ));
                    }
                    Command::Gacha(action) => match action {
                        GachaAction::Add { id } => {
                            if let Some(ref mut player_gacha_bin) = player_info.gacha_bin {
                                player_gacha_bin.gacha_map.insert(*id, GachaBin::default());
                            } else {
                                tracing::debug!("gacha_bin is None");
                            }
                        }
                        GachaAction::Clear { .. } => {
                            if let Some(ref mut player_gacha_bin) = player_info.gacha_bin {
                                player_gacha_bin.gacha_map.clear();
                            } else {
                                tracing::debug!("gacha_bin is None");
                            }
                        }
                    },
                    Command::Tp(action) => match action {
                        TpAction::A { id, x, y, z } => {
                            tp_events.write(ScenePlayerJumpEvent(
                                *player_uid,
                                *id,
                                EnterReason::Gm,
                                (
                                    x.unwrap_or_default(),
                                    y.unwrap_or_default(),
                                    z.unwrap_or_default(),
                                ),
                            ));
                        }
                        TpAction::R { id, x, y, z } => {
                            if let Some(ref player_scene_bin) = player_info.scene_bin {
                                tp_events.write(ScenePlayerJumpEvent(
                                    *player_uid,
                                    *id,
                                    EnterReason::Gm,
                                    (
                                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().x
                                            + x.unwrap_or_default(),
                                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().y
                                            + y.unwrap_or_default(),
                                        player_scene_bin.my_cur_scene_pos.unwrap_or_default().z
                                            + z.unwrap_or_default(),
                                    ),
                                ));
                            }
                        }
                    },
                    Command::Dun(dungeon_id) => match dungeon_id {
                        Some(id) => {
                            enter_dungeon_events
                                .write(ScenePlayerEnterDungeonEvent(*player_uid, *id));
                            gm_notify_events.write(ConsoleChatNotifyEvent(
                                *player_uid,
                                format!("entering dungeon {}", id),
                            ));
                        }
                        None => {
                            gm_notify_events.write(ConsoleChatNotifyEvent(
                                *player_uid,
                                "exit dungeon (notify only)".to_string(),
                            ));
                        }
                    },
                    Command::Pos => {
                        if let Some(ref scene_bin) = player_info.scene_bin {
                            let pos = scene_bin.my_cur_scene_pos.unwrap_or_default();
                            let rot = scene_bin.my_cur_scene_rot.unwrap_or_default();
                            gm_notify_events.write(ConsoleChatNotifyEvent(
                                *player_uid,
                                format!(
                                    "pos: ({:.2}, {:.2}, {:.2}) rot: ({:.2}, {:.2}, {:.2}) scene_id: {}",
                                    pos.x, pos.y, pos.z,
                                    rot.x, rot.y, rot.z,
                                    scene_bin.my_cur_scene_id
                                ),
                            ));
                        }
                    }
                    Command::SendPacket(key) => match key.as_str() {
                        "cmd_id_list" => {
                            match common::string_util::read_utf8_no_bom(
                                "./assets/custom/test_packet.json",
                            ) {
                                Ok(content) => match serde_json::from_str(&*content) {
                                    Ok(value) => {
                                        let value: serde_json::Value = value;
                                        if let Some(cmd_id_list) = value.get("cmd_id_list") {
                                            if let Some(arr) = cmd_id_list.as_array() {
                                                for cmd_id in arr {
                                                    if let Some(cmd_id) = cmd_id.as_u64() {
                                                        message_output.send_none_with_cmd_id(
                                                            *player_uid,
                                                            cmd_id as u16,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {}
                                },
                                Err(_) => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {
                        gm_command_events.write(GmCommandEvent(*player_uid, gm));
                    }
                }
            }
            Err(error) => {
                gm_notify_events.write(ConsoleChatNotifyEvent(
                    *player_uid,
                    format!("error:{}", error),
                ));
            }
        }
    }
}

pub fn gm_talk_notify(
    mut events: MessageReader<ConsoleChatNotifyEvent>,
    message_output: Res<MessageOutput>,
) {
    for ConsoleChatNotifyEvent(player_uid, content) in events.read() {
        message_output.send(
            *player_uid,
            "PrivateChatNotify",
            PrivateChatNotify {
                chat_info: Some(ChatInfo {
                    time: unix_timestamp() as u32,
                    to_uid: *player_uid,
                    uid: 123,
                    content: Some(nod_krai_gi_proto::normal::chat_info::Content::Text(
                        content.clone(),
                    )),
                    ..Default::default()
                }),
            },
        );
    }
}

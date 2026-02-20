use bevy_ecs::prelude::*;
use std::sync::Arc;

use nod_krai_gi_event::command::*;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    mark_map_req::Operation, MapMarkPointType, MarkMapReq, PersonalSceneJumpReq,
    PersonalSceneJumpRsp, PlayerEnterDungeonReq, PlayerEnterDungeonRsp, SceneTransToPointReq,
    SceneTransToPointRsp, Vector,
};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn message_on_map(
    mut events: MessageReader<ClientMessageEvent>,
    mut debug_events: MessageWriter<DebugCommandEvent>,
    mut jump_events: MessageWriter<ScenePlayerJumpEvent>,
    mut jump_point_events: MessageWriter<ScenePlayerJumpByPointEvent>,
    mut enter_dungeon_events: MessageWriter<ScenePlayerEnterDungeonEvent>,
    message_output: Res<MessageOutput>,
    players: Res<Players>,
) {
    for message in events.read() {
        match message.message_name() {
            "MarkMapReq" => {
                if let Some(request) = message.decode::<MarkMapReq>() {
                    debug!(
                        "operation: {:?}, mark: {:?}, old: {:?}",
                        request.op, request.mark, request.old
                    );

                    if let (Operation::Add, Some(mark), _) =
                        (request.op(), request.mark, request.old)
                    {
                        match mark.point_type() {
                            MapMarkPointType::Npc => {
                                debug_events.write(DebugCommandEvent {
                                    executor_uid: message.sender_uid(),
                                    kind: CommandKind::QuickSpawnMonster {
                                        monster_id: mark
                                            .name
                                            .split(' ')
                                            .next()
                                            .and_then(|s| s.parse::<u32>().ok()),
                                        position: (
                                            mark.pos.unwrap_or_default().x,
                                            mark.pos.unwrap_or_default().z,
                                        ),
                                    },
                                });
                            }
                            MapMarkPointType::Quest => {
                                debug_events.write(DebugCommandEvent {
                                    executor_uid: message.sender_uid(),
                                    kind: CommandKind::QuickSpawnGadget {
                                        gadget_id: mark
                                            .name
                                            .split(' ')
                                            .next()
                                            .and_then(|s| s.parse::<u32>().ok()),
                                        position: (
                                            mark.pos.unwrap_or_default().x,
                                            mark.pos.unwrap_or_default().z,
                                        ),
                                    },
                                });
                            }
                            MapMarkPointType::Special => {
                                debug_events.write(DebugCommandEvent {
                                    executor_uid: message.sender_uid(),
                                    kind: CommandKind::QuickTravel {
                                        scene_id: Some(mark.scene_id),
                                        position: (
                                            mark.pos.unwrap_or_default().x,
                                            mark.name
                                                .split(' ')
                                                .next()
                                                .and_then(|s| s.parse::<f32>().ok()),
                                            mark.pos.unwrap_or_default().z,
                                        ),
                                    },
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }
            "SceneTransToPointReq" => {
                if let Some(request) = message.decode::<SceneTransToPointReq>() {
                    jump_point_events.write(ScenePlayerJumpByPointEvent(
                        message.sender_uid(),
                        request.scene_id,
                        request.point_id,
                    ));
                    message_output.send(
                        message.sender_uid(),
                        "SceneTransToPointRsp",
                        SceneTransToPointRsp {
                            retcode: 0,
                            scene_id: request.scene_id,
                            point_id: request.point_id,
                        },
                    );
                }
            }
            "PersonalSceneJumpReq" => {
                if let Some(request) = message.decode::<PersonalSceneJumpReq>() {
                    let Some(player_info) = players.get(message.sender_uid()) else {
                        continue;
                    };
                    let Some(ref player_scene_bin) = player_info.scene_bin else {
                        continue;
                    };
                    let scene_point_entry_map_collection_clone = Arc::clone(
                        nod_krai_gi_data::scene::scene_point_config::SCENE_POINT_ENTRY_MAP_COLLECTION
                            .get()
                            .unwrap(),
                    );
                    let Some(scene_point_data) = scene_point_entry_map_collection_clone
                        .get(&((player_scene_bin.my_cur_scene_id << 16) + request.point_id))
                    else {
                        continue;
                    };
                    jump_point_events.write(ScenePlayerJumpByPointEvent(
                        message.sender_uid(),
                        player_scene_bin.my_cur_scene_id,
                        request.point_id,
                    ));
                    message_output.send(
                        message.sender_uid(),
                        "PersonalSceneJumpRsp",
                        PersonalSceneJumpRsp {
                            retcode: 0,
                            dest_scene_id: scene_point_data.tran_scene_id,
                            dest_pos: Some(Vector {
                                x: scene_point_data.pos.x,
                                z: scene_point_data.pos.y,
                                y: scene_point_data.pos.z,
                            }),
                        },
                    );
                }
            }
            "PlayerEnterDungeonReq" => {
                if let Some(request) = message.decode::<PlayerEnterDungeonReq>() {
                    enter_dungeon_events.write(ScenePlayerEnterDungeonEvent(
                        message.sender_uid(),
                        request.dungeon_id,
                    ));
                    message_output.send(
                        message.sender_uid(),
                        "PlayerEnterDungeonRsp",
                        PlayerEnterDungeonRsp {
                            retcode: 0,
                            dungeon_id: request.dungeon_id,
                            point_id: request.point_id,
                        },
                    );
                }
            }
            "PlayerQuitDungeonReq" => {
                let Some(player_info) = players.get(message.sender_uid()) else {
                    continue;
                };

                if let Some(ref player_dungeon_bin) = player_info.dungeon_bin {
                    let Some(quit_pos) = player_dungeon_bin.quit_pos else {
                        continue;
                    };

                    jump_events.write(ScenePlayerJumpEvent(
                        message.sender_uid(),
                        player_dungeon_bin.quit_scene_id,
                        EnterReason::DungeonQuit,
                        (quit_pos.x, quit_pos.y, quit_pos.z),
                    ));

                    message_output.send_none(message.sender_uid(), "PlayerQuitDungeonRsp");
                }
            }
            &_ => {}
        }
    }
}

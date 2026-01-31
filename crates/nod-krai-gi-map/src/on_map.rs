use bevy_ecs::prelude::*;

use nod_krai_gi_command::{CommandKind, DebugCommandEvent};
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::mark_map_req::Operation;
use nod_krai_gi_proto::{MapMarkPointType, MarkMapReq, SceneTransToPointReq, SceneTransToPointRsp};
use nod_krai_gi_scene::ScenePlayerJumpByPointEvent;
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn message_on_map(
    mut events: MessageReader<ClientMessageEvent>,
    mut debug_events: MessageWriter<DebugCommandEvent>,
    mut jump_events: MessageWriter<ScenePlayerJumpByPointEvent>,
    message_output: Res<MessageOutput>,
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
                            },
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
                    jump_events.write(ScenePlayerJumpByPointEvent(
                        message.sender_uid(),
                        request.scene_id,
                        request.point_id,
                    ));
                    message_output.send(
                        message.sender_uid(),
                        "SceneTransToPointRsp",
                        SceneTransToPointRsp {
                            retcode: 0,
                            point_id: request.point_id,
                            scene_id: request.scene_id,
                        },
                    );
                }
            }
            &_ => {}
        }
    }
}

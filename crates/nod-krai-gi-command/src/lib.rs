use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::gm_util::Command;
use common::gm_util::{parse_command, TpAction};
use common::player_cache::cache_get_is_tp;
use common::time_util::unix_timestamp;
use nod_krai_gi_entity::common::{EntityCounter, Visible};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_entity::monster::spawn_monster_entity;
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{ChatInfo, PrivateChatNotify};
use nod_krai_gi_proto::server_only::VectorBin;
use rand::RngCore;
use tracing::{debug, instrument};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_command_handler)
            .add_systems(Update, gm_command_handler)
            .add_systems(Update, gm_talk_notify);
    }
}

#[instrument(skip_all)]
pub fn debug_command_handler(
    mut events: MessageReader<DebugCommandEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    players: Res<Players>,
    mut jump_events: MessageWriter<ScenePlayerJumpEvent>,
) {
    for command in events.read() {
        debug!(
            "executor_uid: {}, kind: {:?}",
            command.executor_uid, command.kind
        );

        let Some(player_info) = players.get(command.executor_uid) else {
            continue;
        };

        match command.kind {
            CommandKind::QuickSpawnMonster {
                monster_id,
                position,
            } => {
                // spawn random slime if not specified
                let monster_id = monster_id.unwrap_or_else(|| {
                    [20010101, 20010302, 20010502, 20010803, 20011002]
                        [rand::thread_rng().next_u32() as usize % 5]
                });

                let Some(monster_entity) = spawn_monster_entity(
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
                ) else {
                    continue;
                };
                commands.entity(monster_entity).insert(Visible);
            }
            CommandKind::QuickSpawnGadget {
                gadget_id,
                position,
            } => {
                // spawn random slime if not specified
                let gadget_id = gadget_id.unwrap_or_else(|| {
                    [70801015, 70801016, 70801017, 70801018, 70801019, 70801020]
                        [rand::thread_rng().next_u32() as usize % 5]
                });

                let Some(gadget_entity) = spawn_gadget_entity(
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
                    gadget_id,
                    90,
                    0,
                    true,
                ) else {
                    continue;
                };

                commands.entity(gadget_entity).insert(Visible);
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

#[instrument(skip_all)]
pub fn gm_command_handler(
    mut events: MessageReader<ConsoleChatReqEvent>,
    players: Res<Players>,
    mut gm_notify_events: MessageWriter<ConsoleChatNotifyEvent>,
    mut tp_events: MessageWriter<ScenePlayerJumpEvent>,
    mut quest_events: MessageWriter<CommandQuestEvent>,
    mut item_events: MessageWriter<CommandItemEvent>,
) {
    for ConsoleChatReqEvent(player_uid, console_content) in events.read() {
        let Some(player_info) = players.get(*player_uid) else {
            continue;
        };
        if cache_get_is_tp(*player_uid).unwrap_or(true) {
            continue;
        }
        let result = parse_command(console_content);
        match result {
            Ok(gm) => {
                debug!("gm_command_handler result: {:?}", gm);
                match gm {
                    Command::Avatar(_) => {}
                    Command::Tp(action) => match action {
                        TpAction::A { id, x, y, z } => {
                            tp_events.write(ScenePlayerJumpEvent(
                                *player_uid,
                                id,
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
                                    id,
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
                    Command::Quest(action) => {
                        quest_events.write(CommandQuestEvent(*player_uid, action));
                    }
                    Command::Item(action) => {
                        item_events.write(CommandItemEvent(*player_uid, action));
                    }
                    Command::Prop(_, _) => {}
                    Command::Dun(_) => {}
                    Command::Pos => {}
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

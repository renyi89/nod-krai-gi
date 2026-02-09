use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::game_server_config::cache_get_is_tp;
use common::gm_util::Command;
use common::gm_util::{parse_command, TpAction};
use common::time_util::unix_timestamp;
use nod_krai_gi_data::excel::{gadget_excel_config_collection, monster_excel_config_collection};
use nod_krai_gi_entity::ability::Ability;
use nod_krai_gi_entity::common::OwnerProtocolEntityID;
use nod_krai_gi_entity::gadget::{GadgetBundle, GadgetID};
use nod_krai_gi_entity::util::{
    create_fight_properties_by_gadget_config, create_fight_properties_by_monster_config,
};
use nod_krai_gi_entity::{
    common::{
        EntityCounter, GlobalAbilityValues, GrowCurveConfigType, InstancedAbilities,
        InstancedModifiers, Level, LifeState, Visible,
    },
    monster::{MonsterBundle, MonsterID},
    transform::Transform,
    util::to_protocol_entity_id,
};
use nod_krai_gi_event::command::*;
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{ChatInfo, PrivateChatNotify, ProtEntityType};
use nod_krai_gi_proto::server_only::Vector;
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

                let monster_excel_config_collection_clone =
                    std::sync::Arc::clone(monster_excel_config_collection::get());

                let Some(config) = monster_excel_config_collection_clone.get(&monster_id) else {
                    debug!("monster config for id {monster_id} not found");
                    continue;
                };

                let level = 90;

                let mut fight_properties = create_fight_properties_by_monster_config(config);
                for grow_curve in config.prop_grow_curves.iter() {
                    fight_properties.apply_grow_curve(
                        level,
                        grow_curve,
                        GrowCurveConfigType::Monster,
                    );
                }
                fight_properties.apply_base_values();

                commands
                    .spawn(MonsterBundle {
                        monster_id: MonsterID(monster_id),
                        entity_id: to_protocol_entity_id(
                            ProtEntityType::ProtEntityMonster,
                            entity_counter.inc(),
                        ),
                        level: Level(level),
                        transform: Transform {
                            // Take Y (height) from player's pos, spawn a bit above
                            position: (
                                position.0,
                                player_info.scene_bin.my_prev_pos.y + 4.0,
                                position.1,
                            )
                                .into(),
                            rotation: Vector::default(),
                        },
                        fight_properties,
                        instanced_abilities: InstancedAbilities::default(),
                        instanced_modifiers: InstancedModifiers::default(),
                        global_ability_values: GlobalAbilityValues::default(),
                        life_state: LifeState::Alive,
                    })
                    .insert(Visible);
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

                let gadget_excel_config_collection_clone =
                    std::sync::Arc::clone(gadget_excel_config_collection::get());

                let Some(config) = gadget_excel_config_collection_clone.get(&gadget_id) else {
                    debug!("gadget config for id {gadget_id} not found");
                    continue;
                };

                let level = 90;

                let mut fight_properties = create_fight_properties_by_gadget_config(config);
                fight_properties.apply_base_values();

                let ability = Ability::new_for_gadget(&config.json_name);

                commands
                    .spawn(GadgetBundle {
                        gadget_id: GadgetID(gadget_id),
                        entity_id: to_protocol_entity_id(
                            ProtEntityType::ProtEntityGadget,
                            entity_counter.inc(),
                        ),
                        owner_entity_id: OwnerProtocolEntityID(None),
                        level: Level(level),
                        transform: Transform {
                            // Take Y (height) from player's pos, spawn a bit above
                            position: (position.0, player_info.scene_bin.my_prev_pos.y, position.1)
                                .into(),
                            rotation: Vector::default(),
                        },
                        fight_properties,
                        ability: ability,
                        instanced_abilities: InstancedAbilities::default(),
                        instanced_modifiers: InstancedModifiers::default(),
                        global_ability_values: GlobalAbilityValues::default(),
                        life_state: LifeState::Alive,
                    })
                    .insert(Visible);
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
                            tp_events.write(ScenePlayerJumpEvent(
                                *player_uid,
                                id,
                                EnterReason::Gm,
                                (
                                    player_info.scene_bin.my_prev_pos.x + x.unwrap_or_default(),
                                    player_info.scene_bin.my_prev_pos.y + y.unwrap_or_default(),
                                    player_info.scene_bin.my_prev_pos.z + z.unwrap_or_default(),
                                ),
                            ));
                        }
                    },
                    Command::Quest(action) => {
                        quest_events.write(CommandQuestEvent(*player_uid, action));
                    }
                    Command::Item(_) => {}
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

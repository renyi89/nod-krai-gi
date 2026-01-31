use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
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
    transform::{Transform, Vector3},
    util::to_protocol_entity_id,
    ProtEntityType,
};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_scene::ScenePlayerJumpEvent;
use rand::RngCore;
use tracing::{debug, instrument};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DebugCommandEvent>()
            .add_message::<GmCommandEvent>()
            .add_systems(Update, debug_command_handler)
            .add_systems(Update, gm_command_handler);
    }
}

#[derive(Message)]
pub struct DebugCommandEvent {
    pub executor_uid: u32,
    pub kind: CommandKind,
}

#[derive(Message)]
pub struct GmCommandEvent {
    pub executor_uid: u32,
    pub kind: CommandKind,
}

#[derive(Debug)]
pub enum CommandKind {
    QuickSpawnMonster {
        monster_id: Option<u32>,
        position: (f32, f32),
    },
    QuickSpawnGadget {
        gadget_id: Option<u32>,
        position: (f32, f32),
    },
    QuickTravel {
        scene_id: Option<u32>,
        position: (f32, Option<f32>, f32),
    },
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

        let player = players.get(command.executor_uid);

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
                                player.world_position.position.1 + 4.0,
                                position.1,
                            )
                                .into(),
                            rotation: Vector3::default(),
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

                let ability = Ability::new_for_gadget(config.json_name.as_str());

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
                            position: (
                                position.0,
                                player.world_position.position.1 + 1.0,
                                position.1,
                            )
                                .into(),
                            rotation: Vector3::default(),
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
            CommandKind::QuickTravel { scene_id, position } => {
                let destination =
                    Vector3::from((position.0, position.1.unwrap_or(2600.0), position.2));
                match scene_id {
                    None => {}
                    Some(scene_id) => {
                        jump_events.write(ScenePlayerJumpEvent(
                            command.executor_uid,
                            scene_id,
                            destination,
                        ));
                    }
                }
            }
        }
    }
}

#[allow(unused)]
#[instrument(skip_all)]
pub fn gm_command_handler() {}

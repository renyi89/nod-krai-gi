use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::*;
use common::player_cache::cache_get_scene_level;
use nod_krai_gi_data::scene::GadgetState;
use nod_krai_gi_entity::common::{
    BlockId, ConfigId, EntityCounter, GroupId, ProtocolEntityID, ToBeRemovedMarker, Visible,
};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_entity::monster::spawn_monster_entity;
use nod_krai_gi_entity::EntityDisappearEvent;
use nod_krai_gi_event::lua::{DespawnGroupEntityEvent, SpawnGroupEntityEvent};
use nod_krai_gi_proto::normal::VisionType;
use nod_krai_gi_proto::server_only::VectorBin;
use nod_krai_gi_scene::common::WorldOwnerUID;

pub fn spawn_group_entity(
    mut spawn_group_entity_event: MessageReader<SpawnGroupEntityEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    world_owner_uid: Res<WorldOwnerUID>,
) {
    let scene_group_collection_clone = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    let gather_excel_config_collection_clone =
        std::sync::Arc::clone(nod_krai_gi_data::excel::gather_excel_config_collection::get());

    for event in spawn_group_entity_event.read() {
        let Some(scene_group_template) = scene_group_collection_clone.get(&event.group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        let show_level =
            cache_get_scene_level(world_owner_uid.0, scene_group_template.base_info.scene_id)
                .unwrap_or(1);

        if scene_group_template.init_config.suite < 1
            || scene_group_template.init_config.suite > scene_group_template.suites.len() as u32
        {
            continue;
        }

        let suite =
            &scene_group_template.suites[(scene_group_template.init_config.suite - 1) as usize];

        for monster in scene_group_template.monsters.iter() {
            if suite.monsters.contains(&monster.config_id) {
                let mut level = monster.level.unwrap_or(103) + 67;
                level += show_level - 1;
                let Some(monster_entity) = spawn_monster_entity(
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: monster.pos.x,
                        y: monster.pos.y,
                        z: monster.pos.z,
                    },
                    VectorBin {
                        x: monster.rot.x,
                        y: monster.rot.y,
                        z: monster.rot.z,
                    },
                    monster.monster_id,
                    level,
                    monster.title_id.unwrap_or(0),
                    monster.special_name_id.unwrap_or(0),
                ) else {
                    continue;
                };
                commands
                    .entity(monster_entity)
                    .insert(BlockId(event.block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(monster.config_id))
                    .insert(Visible);
            }
        }
        for gadget in scene_group_template.gadgets.iter() {
            if suite.gadgets.contains(&gadget.config_id) {
                let mut gadget_id = gadget.gadget_id;
                let mut is_interactive = false;
                if gadget_id == 70500000 && gadget.point_type.is_some() {
                    let Some(gather_config) = gather_excel_config_collection_clone
                        .get(&gadget.point_type.unwrap_or_default())
                    else {
                        tracing::debug!(
                            "gather config {} doesn't exist",
                            gadget.point_type.unwrap_or_default()
                        );
                        continue;
                    };
                    gadget_id = gather_config.gadget_id;
                    is_interactive = true;
                }
                let Some(gadget_entity) = spawn_gadget_entity(
                    &mut commands,
                    &mut entity_counter,
                    VectorBin {
                        x: gadget.pos.x,
                        y: gadget.pos.y,
                        z: gadget.pos.z,
                    },
                    VectorBin {
                        x: gadget.rot.x,
                        y: gadget.rot.y,
                        z: gadget.rot.z,
                    },
                    gadget_id,
                    gadget.level.unwrap_or(90),
                    gadget.state.unwrap_or(GadgetState::Default) as u32,
                    is_interactive,
                ) else {
                    continue;
                };
                commands
                    .entity(gadget_entity)
                    .insert(BlockId(event.block_id))
                    .insert(GroupId(event.group_id))
                    .insert(ConfigId(gadget.config_id))
                    .insert(Visible);
            }
        }
    }
}

pub fn despawn_group_entity(
    mut commands: Commands,
    mut despawn_group_entity_event: MessageReader<DespawnGroupEntityEvent>,
    mut entities: Query<(Entity, &ProtocolEntityID, &GroupId)>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for event in despawn_group_entity_event.read() {
        entities
            .iter_mut()
            .filter(|(_, _, group_id)| group_id.0 == event.group_id)
            .for_each(|(entity, entity_id, _)| {
                disappear_events.write(EntityDisappearEvent(
                    entity_id.0,
                    VisionType::VisionMiss.into(),
                ));
                commands.entity(entity).insert(ToBeRemovedMarker);
            });
    }
}

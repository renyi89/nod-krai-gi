use bevy_ecs::change_detection::ResMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::{Commands, Query};
use nod_krai_gi_entity::common::{
    BlockId, ConfigId, EntityCounter, GroupId, ToBeRemovedMarker, Visible,
};
use nod_krai_gi_entity::gadget::spawn_gadget_entity;
use nod_krai_gi_entity::monster::spawn_monster_entity;
use nod_krai_gi_event::lua::{DespawnGroupEntityEvent, SpawnGroupEntityEvent};
use nod_krai_gi_proto::server_only::VectorBin;
use std::sync::Arc;

pub fn spawn_group_entity(
    mut spawn_group_entity_event: MessageReader<SpawnGroupEntityEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
) {
    let scene_group_collection_clone = Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION
            .get()
            .unwrap(),
    );

    for event in spawn_group_entity_event.read() {
        let Some(scene_group_template) = scene_group_collection_clone.get(&event.group_id) else {
            continue;
        };
        let Some(scene_group_template) = scene_group_template.value() else {
            continue;
        };

        if scene_group_template.init_config.suite < 1
            || scene_group_template.init_config.suite > scene_group_template.suites.len() as u32
        {
            continue;
        }

        let suite =
            &scene_group_template.suites[(scene_group_template.init_config.suite - 1) as usize];

        for monster in scene_group_template.monsters.iter() {
            if suite.monsters.contains(&monster.config_id) {
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
                    monster.level.unwrap_or(90),
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
                    gadget.gadget_id,
                    gadget.level.unwrap_or(90),
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
    mut entities: Query<(Entity, &GroupId)>,
) {
    for event in despawn_group_entity_event.read() {
        entities
            .iter_mut()
            .filter(|(_, group_id)| group_id.0 == event.group_id)
            .for_each(|(entity, _)| {
                commands.entity(entity).insert(ToBeRemovedMarker);
            });
    }
}

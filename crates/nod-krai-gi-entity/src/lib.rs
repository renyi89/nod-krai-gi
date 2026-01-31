use avatar::{AvatarAppearanceChangeEvent, AvatarEquipChangeEvent};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::{
    EntityById, EntityCounter, FightProperties, LifeState, ProtocolEntityID, ToBeRemovedMarker,
};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_message::output::MessageOutput;
use std::collections::HashMap;

pub mod ability;
pub mod avatar;
pub mod client_gadget;
pub mod common;
pub mod fight;
pub mod gadget;
pub mod monster;
pub mod mp_level;
pub mod play_team;
pub mod team;
pub mod transform;
pub mod util;
pub mod weapon;

use crate::common::Visible;
use crate::fight::EntityFightPropChangeReasonNotifyEvent;
use crate::{avatar::CurrentPlayerAvatarMarker, client_gadget::EntitySystemSet};
pub use nod_krai_gi_proto::ProtEntityType;
use nod_krai_gi_proto::{LifeStateChangeNotify, SceneEntityDisappearNotify, VisionType};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityCounter::default())
            .insert_resource(EntityById::default())
            .add_message::<EntityPropertySeparateUpdateEvent>()
            .add_message::<EntityDisappearEvent>()
            .add_message::<AvatarEquipChangeEvent>()
            .add_message::<AvatarAppearanceChangeEvent>()
            .add_message::<EntityFightPropChangeReasonNotifyEvent>()
            .add_systems(Update, update_entity_index)
            .add_systems(Update, update_separate_property_entity)
            .add_systems(Update, avatar::update_avatar_appearance)
            .add_systems(
                Update,
                client_gadget::handle_evt_update_gadget
                    .in_set(EntitySystemSet::HandleEvtGadgetUpdate),
            )
            .add_systems(
                PostUpdate,
                (
                    fight::notify_fight_properties_to_clients,
                    fight::notify_fight_properties_change_reason_to_clients,
                )
                    .chain(),
            )
            .add_systems(
                Last,
                (
                    update_entity_life_state,
                    notify_life_state_change,
                    notify_disappear_entities,
                    remove_marked_entities,
                    avatar::notify_avatar_appearance_change,
                    avatar::notify_appear_avatar_entities
                        .run_if(avatar::run_if_avatar_entities_appeared),
                    avatar::notify_appear_replace_avatar_entities
                        .run_if(avatar::run_if_avatar_entities_appeared),
                    monster::notify_appear_monster_entities
                        .run_if(monster::run_if_monster_entities_appeared),
                    gadget::notify_appear_gadget_entities
                        .run_if(gadget::run_if_gadget_entities_appeared),
                )
                    .chain(),
            );
    }
}

#[derive(Message)]
pub struct EntityPropertySeparateUpdateEvent(pub Entity, pub FightPropType, pub f32);

#[derive(Message)]
pub struct EntityDisappearEvent(pub u32, pub VisionType);

fn update_separate_property_entity(
    mut events: MessageReader<EntityPropertySeparateUpdateEvent>,
    mut entities: Query<&mut FightProperties>,
) {
    for event in events.read() {
        match entities.get_mut(event.0) {
            Ok(mut fight_props) => {
                fight_props.change_property(event.1, event.2);
            }
            Err(_) => {}
        }
    }
}

fn update_entity_life_state(
    mut commands: Commands,
    mut entities: Query<
        (Entity, &ProtocolEntityID, &FightProperties, &mut LifeState),
        Changed<FightProperties>,
    >,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for (entity, id, fight_props, mut life_state) in entities.iter_mut() {
        if fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP) <= 0.0 {
            if id.entity_type() == ProtEntityType::ProtEntityAvatar {
                commands
                    .entity(entity)
                    .remove::<CurrentPlayerAvatarMarker>()
                    .remove::<Visible>();
            } else {
                commands.entity(entity).insert(ToBeRemovedMarker);
            }
            disappear_events.write(EntityDisappearEvent(id.0, VisionType::VisionDie));
            *life_state = LifeState::Dead;
        } else if *life_state == LifeState::Dead {
            *life_state = LifeState::Alive;
        }
    }
}

fn notify_life_state_change(
    entities: Query<(&ProtocolEntityID, &LifeState), Changed<LifeState>>,
    message_output: Res<MessageOutput>,
) {
    entities.iter().for_each(|(id, life_state)| {
        message_output.send_to_all(
            "LifeStateChangeNotify",
            Some(LifeStateChangeNotify {
                entity_id: id.0,
                life_state: *life_state as u32,
                ..Default::default()
            }),
        )
    });
}

fn notify_disappear_entities(
    mut events: MessageReader<EntityDisappearEvent>,
    message_output: Res<MessageOutput>,
) {
    let mut grouped: HashMap<VisionType, Vec<u32>> = HashMap::new();

    for EntityDisappearEvent(id, disappear_type) in events.read() {
        grouped.entry(*disappear_type).or_default().push(*id);
    }

    for (disappear_type, ids) in grouped {
        message_output.send_to_all(
            "SceneEntityDisappearNotify",
            Some(SceneEntityDisappearNotify {
                disappear_type: disappear_type.into(),
                param: 0,
                entity_list: ids,
            }),
        );
    }
}

fn remove_marked_entities(
    mut index: ResMut<EntityById>,
    mut commands: Commands,
    entities: Query<Entity, With<ToBeRemovedMarker>>,
) {
    entities.iter().for_each(|entity| {
        index.0.retain(|_, e| *e != entity);
        commands.entity(entity).despawn()
    });
}

fn update_entity_index(
    mut index: ResMut<EntityById>,
    query: Query<(Entity, &ProtocolEntityID), Changed<ProtocolEntityID>>,
) {
    for (entity, id) in query.iter() {
        index.0.insert(id.0, entity);
    }
}

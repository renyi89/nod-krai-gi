use avatar::{AvatarAppearanceChangeEvent, AvatarEquipChangeEvent};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::{
    EntityById, EntityCounter, FightProperties, LifeState, ProtocolEntityID, ToBeRemovedMarker,
};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_event::entity::{EvtCreateGadgetEvent, EvtDestroyGadgetEvent, GadgetInteractEvent};
use nod_krai_gi_message::event::ClientMessageEvent;
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
use nod_krai_gi_proto::normal::{
    EvtCreateGadgetNotify, EvtDestroyGadgetNotify, GadgetInteractReq, GadgetInteractRsp,
    LifeStateChangeNotify, ProtEntityType, SceneEntityDisappearNotify, VisionType,
};

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
            .add_systems(Update, handle_entity)
            .add_systems(Update, gadget::handle_gadget_interact)
            .add_systems(Update, avatar::update_avatar_appearance)
            .add_systems(
                Update,
                client_gadget::handle_evt_create_gadget
                    .in_set(EntitySystemSet::HandleEvtGadgetUpdate),
            )
            .add_systems(Update, client_gadget::handle_evt_destroy_gadget)
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
    message_output: Res<MessageOutput>,
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
            if *life_state != LifeState::Dead {
                message_output.send_to_all(
                    "LifeStateChangeNotify",
                    Some(LifeStateChangeNotify {
                        entity_id: id.0,
                        life_state: LifeState::Dead as u32,
                        ..Default::default()
                    }),
                )
            }
            *life_state = LifeState::Dead;
        } else if *life_state == LifeState::Dead {
            if *life_state != LifeState::Alive {
                message_output.send_to_all(
                    "LifeStateChangeNotify",
                    Some(LifeStateChangeNotify {
                        entity_id: id.0,
                        life_state: LifeState::Alive as u32,
                        ..Default::default()
                    }),
                )
            }
            *life_state = LifeState::Alive;
        }
    }
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

pub fn handle_entity(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut gadget_interact_events: MessageWriter<GadgetInteractEvent>,
    mut evt_create_gadget_events: MessageWriter<EvtCreateGadgetEvent>,
    mut evt_destroy_gadget_events: MessageWriter<EvtDestroyGadgetEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "EvtCreateGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtCreateGadgetNotify>() {
                    evt_create_gadget_events.write(EvtCreateGadgetEvent(
                        notify.config_id,
                        notify.entity_id,
                        notify.owner_entity_id,
                    ));
                }
            }
            "EvtDestroyGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtDestroyGadgetNotify>() {
                    evt_destroy_gadget_events.write(EvtDestroyGadgetEvent(notify.entity_id));
                }
            }
            "GadgetInteractReq" => {
                if let Some(req) = message.decode::<GadgetInteractReq>() {
                    message_output.send(
                        message.sender_uid(),
                        "GadgetInteractRsp",
                        GadgetInteractRsp {
                            retcode: 0,
                            gadget_id: req.gadget_id,
                            gadget_entity_id: req.gadget_entity_id,
                            op_type: req.op_type,
                            ..Default::default()
                        },
                    );

                    gadget_interact_events.write(GadgetInteractEvent(
                        message.sender_uid(),
                        req.gadget_id,
                        req.gadget_entity_id,
                    ));
                }
            }
            &_ => {}
        }
    }
}

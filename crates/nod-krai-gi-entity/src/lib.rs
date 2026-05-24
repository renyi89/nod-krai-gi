use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::{
    EntityById, EntityCounter, FightProperties, LifeState, ProtocolEntityID, ToBeRemovedMarker,
};
use nod_krai_gi_data::custom::{resolve_drop, CombinedDrop};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::quest::quest_config::QuestContent;
use nod_krai_gi_data::scene::{EventType, LuaEvt};
use nod_krai_gi_event::entity::{
    EntityDisappearEvent, EntityPropertySeparateUpdateEvent, EntityPropertyUpdateEvent,
    GadgetInteractEvent, SetWorktopOptionsEvent,
};
use nod_krai_gi_event::inventory::ItemDropEvent;
use nod_krai_gi_event::lua::{LuaTriggerEvent, MonsterKillEvent};
use nod_krai_gi_event::quest::QuestContentProgressEvent;
use nod_krai_gi_event::scene::{WorldOwnerUID, WorldVersionConfig};
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

use crate::avatar::CurrentPlayerAvatarMarker;
use crate::common::{ChestDropId, ConfigId, DropTag, GroupId, Level, Visible};
use crate::gadget::GadgetID;
use crate::monster::MonsterID;
use crate::transform::Transform;
use nod_krai_gi_data::scene::group_entity_state_cache::get_group_entity_state_cache;
use nod_krai_gi_proto::normal::{
    GadgetInteractReq, GadgetInteractRsp, LifeStateChangeNotify, ProtEntityType,
    SceneEntityDisappearNotify, SceneEntityDrownReq, SelectWorktopOptionReq,
    SelectWorktopOptionRsp, VisionType,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EntitySystemSet {
    HandleEntitySpawn,
    HandleEntityIndexUpdate,
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityCounter::default())
            .insert_resource(EntityById::default())
            .add_message::<SetWorktopOptionsEvent>()
            .add_systems(
                PreUpdate,
                update_entity_index.in_set(EntitySystemSet::HandleEntityIndexUpdate),
            )
            .add_systems(PreUpdate, handle_entity)
            .add_systems(
                PreUpdate,
                client_gadget::handle_evt_update_gadget.in_set(EntitySystemSet::HandleEntitySpawn),
            )
            .add_systems(Update, update_property_entity)
            .add_systems(Update, update_separate_property_entity)
            .add_systems(Update, gadget::handle_gadget_interact)
            .add_systems(Update, gadget::handle_set_worktop_options)
            .add_systems(Update, gadget::handle_gadget_state_change)
            .add_systems(Update, avatar::update_avatar_appearance)
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
                        .in_set(EntitySystemSet::HandleEntitySpawn)
                        .run_if(avatar::run_if_avatar_entities_appeared),
                    avatar::notify_appear_replace_avatar_entities
                        .run_if(avatar::run_if_avatar_entities_appeared),
                    monster::notify_appear_monster_entities
                        .in_set(EntitySystemSet::HandleEntitySpawn)
                        .run_if(monster::run_if_monster_entities_appeared),
                    gadget::notify_appear_gadget_entities
                        .in_set(EntitySystemSet::HandleEntitySpawn)
                        .run_if(gadget::run_if_gadget_entities_appeared),
                )
                    .chain(),
            );
    }
}

fn update_property_entity(
    mut events: MessageReader<EntityPropertyUpdateEvent>,
    mut entities: Query<&mut FightProperties>,
) {
    for event in events.read() {
        match entities.get_mut(event.0) {
            Ok(mut fight_props) => {
                fight_props.set_property(event.1, event.2);
            }
            Err(_) => {}
        }
    }
}

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
        (
            Entity,
            &ProtocolEntityID,
            &FightProperties,
            &mut LifeState,
            &Level,
            &Transform,
            Option<&MonsterID>,
            Option<&GadgetID>,
            Option<&DropTag>,
            Option<&ChestDropId>,
            Option<&GroupId>,
            Option<&ConfigId>,
        ),
        Changed<FightProperties>,
    >,
    mut item_drop_events: MessageWriter<ItemDropEvent>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
    mut monster_kill_events: MessageWriter<MonsterKillEvent>,
    mut quest_content_events: MessageWriter<QuestContentProgressEvent>,
    message_output: Res<MessageOutput>,
    world_owner_uid: Res<WorldOwnerUID>,
    world_version_config: Res<WorldVersionConfig>,
) {
    let gather_excel_config_collection_clone =
        std::sync::Arc::clone(nod_krai_gi_data::excel::gather_excel_config_collection::get());

    let cache = get_group_entity_state_cache();

    for (
        entity,
        id,
        fight_props,
        mut life_state,
        level,
        transform,
        monster_id,
        gadget_id,
        drop_tag,
        chest_drop_id,
        group_id,
        config_id,
    ) in entities.iter_mut()
    {
        let cur_hp = fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP);
        let max_hp = fight_props.get_property(FightPropType::FIGHT_PROP_MAX_HP);

        if cur_hp <= 0.0 {
            if id.entity_type(world_version_config.protocol_version.as_str())
                == ProtEntityType::ProtEntityAvatar
            {
                commands
                    .entity(entity)
                    .remove::<CurrentPlayerAvatarMarker>()
                    .remove::<Visible>();
            } else {
                match gadget_id {
                    None => {}
                    Some(gadget_id) => {
                        match gather_excel_config_collection_clone
                            .iter()
                            .find(|(_, gather_config)| gather_config.gadget_id == gadget_id.0)
                        {
                            None => {}
                            Some((_, gather_config)) => {
                                item_drop_events.write(ItemDropEvent(
                                    0,
                                    Some((
                                        transform.position.x,
                                        transform.position.y + 0.5,
                                        transform.position.z,
                                    )),
                                    vec![(gather_config.item_id, 1)],
                                ));
                            }
                        }
                    }
                }

                let mut drop_id = 0;

                match drop_tag {
                    None => {}
                    Some(drop_tag) => match &drop_tag.0 {
                        None => {}
                        Some(drop_tag) => {
                            match CombinedDrop::get_drop_config(drop_tag.clone(), level.0) {
                                None => {}
                                Some(drop_config) => {
                                    drop_id = drop_config.drop_id;
                                }
                            }
                        }
                    },
                }

                if drop_id == 0 {
                    match chest_drop_id {
                        None => {}
                        Some(chest_drop_id) => {
                            drop_id = chest_drop_id.0;
                        }
                    }
                }

                if drop_id != 0 {
                    tracing::debug!("drop_id is {}", drop_id);
                    let drop_vec = resolve_drop(drop_id, 1);
                    tracing::debug!("drop_vec is {:#?}", drop_vec);
                    if !drop_vec.is_empty() {
                        item_drop_events.write(ItemDropEvent(
                            0,
                            Some((
                                transform.position.x,
                                transform.position.y + 0.5,
                                transform.position.z,
                            )),
                            drop_vec,
                        ));
                    }
                }

                commands.entity(entity).insert(ToBeRemovedMarker);
            }
            disappear_events.write(EntityDisappearEvent(id.0, VisionType::VisionDie));
            if *life_state != LifeState::Dead {
                message_output.send_to_all(
                    "LifeStateChangeNotify",
                    LifeStateChangeNotify {
                        entity_id: id.0,
                        life_state: LifeState::Dead as u32,
                        ..Default::default()
                    },
                )
            }
            *life_state = LifeState::Dead;
        } else if *life_state == LifeState::Dead {
            if *life_state != LifeState::Alive {
                message_output.send_to_all(
                    "LifeStateChangeNotify",
                    LifeStateChangeNotify {
                        entity_id: id.0,
                        life_state: LifeState::Alive as u32,
                        ..Default::default()
                    },
                )
            }
            *life_state = LifeState::Alive;
        }

        if let (Some(group_id), Some(config_id)) = (group_id, config_id) {
            if monster_id.is_some() {
                cache.on_monster_life_state_update(
                    world_owner_uid.0,
                    group_id.0,
                    config_id.0,
                    *life_state as u32,
                    cur_hp,
                    max_hp,
                );

                if cur_hp <= 0.0 {
                    lua_trigger_events.write(LuaTriggerEvent {
                        group_id: group_id.0,
                        event_type: EventType::EventAnyMonsterDie,
                        evt: LuaEvt {
                            param1: config_id.0,
                            param2: 0,
                            param3: 0,
                            source_eid: id.0,
                            target_eid: id.0,
                        },
                    });

                    monster_kill_events.write(MonsterKillEvent {
                        group_id: group_id.0,
                        config_id: config_id.0,
                        monster_id: monster_id.map(|m| m.0).unwrap_or(0),
                    });

                    quest_content_events.write(QuestContentProgressEvent {
                        player_uid: world_owner_uid.0,
                        content_type: QuestContent::KillMonster,
                        param: monster_id.map(|m| m.0).unwrap_or(0),
                        param2: 0,
                        param3: 0,
                        add_progress: 1,
                    });
                    quest_content_events.write(QuestContentProgressEvent {
                        player_uid: world_owner_uid.0,
                        content_type: QuestContent::MonsterDie,
                        param: monster_id.map(|m| m.0).unwrap_or(0),
                        param2: 0,
                        param3: 0,
                        add_progress: 1,
                    });
                }
            } else if gadget_id.is_some() {
                cache.on_gadget_life_state_update(
                    world_owner_uid.0,
                    group_id.0,
                    config_id.0,
                    *life_state as u32,
                    cur_hp,
                    max_hp,
                );

                if cur_hp <= 0.0 {
                    lua_trigger_events.write(LuaTriggerEvent {
                        group_id: group_id.0,
                        event_type: EventType::EventAnyGadgetDie,
                        evt: LuaEvt {
                            param1: config_id.0,
                            param2: 0,
                            param3: 0,
                            source_eid: id.0,
                            target_eid: id.0,
                        },
                    });
                }
            }
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
            SceneEntityDisappearNotify {
                disappear_type: disappear_type.into(),
                param: 0,
                entity_list: ids,
            },
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
    index: Res<EntityById>,
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    entities: Query<(&ProtocolEntityID, Option<&GroupId>, Option<&ConfigId>)>,
    mut gadget_interact_events: MessageWriter<GadgetInteractEvent>,
    mut update_separate_property_entity_events: MessageWriter<EntityPropertySeparateUpdateEvent>,
    mut lua_trigger_events: MessageWriter<LuaTriggerEvent>,
) {
    for message in events.read() {
        match message.message_name() {
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
            "SceneEntityDrownReq" => {
                if let Some(req) = message.decode::<SceneEntityDrownReq>() {
                    match index.0.get(&req.entity_id) {
                        None => {}
                        Some(entity) => {
                            update_separate_property_entity_events.write(
                                EntityPropertySeparateUpdateEvent(
                                    *entity,
                                    FightPropType::FIGHT_PROP_CUR_HP,
                                    -100000000.0,
                                ),
                            );
                        }
                    }
                }
            }
            "SelectWorktopOptionReq" => {
                if let Some(req) = message.decode::<SelectWorktopOptionReq>() {
                    match index.0.get(&req.gadget_entity_id) {
                        None => {}
                        Some(entity) => match entities.get(*entity) {
                            Ok((_, group_id, config_id)) => {
                                let Some(group_id) = group_id else { continue };
                                let Some(config_id) = config_id else { continue };

                                lua_trigger_events.write(LuaTriggerEvent {
                                    group_id: group_id.0,
                                    event_type: EventType::EventSelectOption,
                                    evt: LuaEvt {
                                        param1: config_id.0,
                                        param2: req.option_id,
                                        param3: 0,
                                        source_eid: req.gadget_entity_id,
                                        target_eid: req.gadget_entity_id,
                                    },
                                });

                                message_output.send(
                                    message.sender_uid(),
                                    "SelectWorktopOptionRsp",
                                    SelectWorktopOptionRsp {
                                        gadget_entity_id: req.gadget_entity_id,
                                        option_id: req.option_id,
                                        ..Default::default()
                                    },
                                );
                            }
                            Err(_) => {}
                        },
                    }
                }
            }
            &_ => {}
        }
    }
}

use super::ability::Ability;
use crate::util::{create_fight_properties_by_gadget_config, to_protocol_entity_id};
use crate::{common::*, int_prop_pair, transform::Transform, EntityDisappearEvent};
use bevy_ecs::{prelude::*, query::QueryData};
use nod_krai_gi_data::custom::{resolve_drop, CombinedDrop};
use nod_krai_gi_data::excel::common::EntityType;
use nod_krai_gi_data::excel::gadget_excel_config_collection;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::scene::GadgetState;
use nod_krai_gi_event::entity::{
    GadgetInteractEvent, GadgetStateChangeEvent, SetWorktopOptionsEvent,
};
use nod_krai_gi_event::inventory::{ItemAddEvent, ItemDropEvent};
use nod_krai_gi_event::scene::WorldOwnerUID;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::item::Detail;
use nod_krai_gi_proto::normal::{
    scene_gadget_info::Content, ProtEntityType, VisionType, WorktopInfo,
};
use nod_krai_gi_proto::server_only::VectorBin;
use std::collections::HashMap;

#[derive(Component)]
pub struct GadgetID(pub u32);

#[derive(Component)]
pub struct Interactive(pub bool);

#[derive(Component)]
pub struct GadgetContent(pub Option<Content>);

#[derive(Component)]
pub struct State(pub u32);

#[derive(Bundle)]
pub struct GadgetBundle {
    pub gadget_id: GadgetID,
    pub entity_id: ProtocolEntityID,
    pub owner_entity_id: OwnerProtocolEntityID,
    pub level: Level,
    pub interactive: Interactive,
    pub gadget_content: GadgetContent,
    pub drop_tag: DropTag,
    pub chest_drop_id: ChestDropId,
    pub state: State,
    pub transform: Transform,
    pub fight_properties: FightProperties,
    pub instanced_abilities: InstancedAbilities,
    pub instanced_modifiers: InstancedModifiers,
    pub global_ability_values: GlobalAbilityValues,
    pub life_state: LifeState,
}

#[derive(QueryData)]
pub struct GadgetQueryReadOnly {
    pub gadget_id: &'static GadgetID,
    pub entity_id: &'static ProtocolEntityID,
    pub owner_entity_id: &'static OwnerProtocolEntityID,
    pub level: &'static Level,
    pub interactive: &'static Interactive,
    pub gadget_content: &'static GadgetContent,
    pub drop_tag: &'static DropTag,
    pub chest_drop_id: &'static ChestDropId,
    pub state: &'static State,
    pub transform: &'static Transform,
    pub fight_properties: &'static FightProperties,
    pub instanced_abilities: &'static InstancedAbilities,
    pub instanced_modifiers: &'static InstancedModifiers,
    pub global_ability_values: &'static GlobalAbilityValues,
    pub life_state: &'static LifeState,
}

pub fn notify_appear_gadget_entities(
    gadgets: Query<(GadgetQueryReadOnly, Option<&GroupId>, Option<&ConfigId>), Added<Visible>>,
    message_output: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::normal::*;

    let mut entity_list: Vec<SceneEntityInfo> = vec![];

    gadgets
        .iter()
        .for_each(|(gadget_data, group_id, config_id)| {
            entity_list.push(SceneEntityInfo {
                entity_type: ProtEntityType::ProtEntityGadget.into(),
                entity_id: gadget_data.entity_id.0,
                name: String::new(),
                motion_info: Some(MotionInfo {
                    pos: Some(gadget_data.transform.position.into()),
                    rot: Some(gadget_data.transform.rotation.into()),
                    speed: Some(Vector::default()),
                    ..Default::default()
                }),
                prop_list: vec![int_prop_pair!(PROP_LEVEL, gadget_data.level.0)],
                fight_prop_list: gadget_data
                    .fight_properties
                    .0
                    .iter()
                    .map(|(k, v)| FightPropPair {
                        prop_type: *k as u32,
                        prop_value: *v,
                    })
                    .collect(),
                life_state: *gadget_data.life_state as u32,
                animator_para_list: vec![AnimatorParameterValueInfoPair {
                    name_id: 0,
                    animator_para: Some(AnimatorParameterValueInfo::default()),
                }],
                last_move_scene_time_ms: 0,
                last_move_reliable_seq: 0,
                entity_client_data: Some(EntityClientData::default()),
                entity_environment_info_list: Vec::with_capacity(0),
                entity_authority_info: Some(EntityAuthorityInfo {
                    ability_info: Some(AbilitySyncStateInfo::default()),
                    born_pos: Some(gadget_data.transform.position.into()),
                    client_extra_info: Some(EntityClientExtraInfo {
                        skill_anchor_position: Some(Vector::default()),
                    }),
                    renderer_changed_info: Some(EntityRendererChangedInfo::default()),
                    pose_para_list: Vec::with_capacity(0),
                    ..Default::default()
                }),
                tag_list: Vec::with_capacity(0),
                server_buff_list: Vec::with_capacity(0),
                entity: Some(scene_entity_info::Entity::Gadget(SceneGadgetInfo {
                    gadget_id: gadget_data.gadget_id.0,
                    is_enable_interact: gadget_data.interactive.0,
                    gadget_state: gadget_data.state.0,
                    group_id: group_id.and_then(|t| Some(t.0)).unwrap_or_default(),
                    config_id: config_id.and_then(|t| Some(t.0)).unwrap_or_default(),
                    born_type: {
                        let mut born_type = GadgetBornType::GadgetBornNone;
                        match &gadget_data.gadget_content.0 {
                            None => {}
                            Some(gadget_content) => match gadget_content {
                                Content::NightCrowGadgetInfo(_) => {}
                                Content::DeshretObeliskGadgetInfo(_) => {}
                                Content::AbilityGadget(_) => {}
                                Content::FishPoolInfo(_) => {}
                                Content::FoundationInfo(_) => {}
                                Content::Weather(_) => {}
                                Content::ShellInfo(_) => {}
                                Content::VehicleInfo(_) => {}
                                Content::StatueGadget(_) => {}
                                Content::GeneralReward(_) => {}
                                Content::RoguelikeGadgetInfo(_) => {}
                                Content::CoinCollectOperatorInfo(_) => {}
                                Content::Worktop(_) => {}
                                Content::OfferingInfo(_) => {}
                                Content::TrifleGadget(_) => {
                                    born_type = GadgetBornType::GadgetBornNone;
                                }
                                Content::MpPlayReward(_) => {}
                                Content::GatherGadget(_) => {}
                                Content::ScreenInfo(_) => {}
                                Content::CustomGadgetTreeInfo(_) => {}
                                Content::BlossomChest(_) => {}
                                Content::ClientGadget(_) => {}
                                Content::BossChest(_) => {}
                            },
                        }
                        born_type as i32
                    },
                    content: gadget_data.gadget_content.0.clone(),
                    ..Default::default()
                })),
                ..Default::default()
            });
        });
    message_output.send_to_all(
        "SceneEntityAppearNotify",
        SceneEntityAppearNotify {
            appear_type: VisionType::VisionBorn.into(),
            param: 0,
            entity_list,
        },
    );
}

pub fn run_if_gadget_entities_appeared(
    gadgets: Query<GadgetQueryReadOnly, Added<Visible>>,
) -> bool {
    !gadgets.is_empty()
}

pub fn spawn_gadget_entity(
    protocol_version: String,
    commands: &mut Commands,
    entity_counter: &mut ResMut<EntityCounter>,
    position: VectorBin,
    rotation: VectorBin,
    gadget_id: u32,
    level: u32,
    is_interactive: bool,
    gadget_content: Option<Content>,
    drop_tag: Option<String>,
    chest_drop_id: u32,
    state: u32,
) -> Option<(u32, Entity, f32, f32)> {
    let gadget_excel_config_collection_clone =
        std::sync::Arc::clone(gadget_excel_config_collection::get());

    let Some(config) = gadget_excel_config_collection_clone.get(&gadget_id) else {
        tracing::debug!("gadget config for id {gadget_id} not found");
        return None;
    };

    let mut fight_properties = create_fight_properties_by_gadget_config(config);
    fight_properties.apply_base_values();

    let cur_hp = fight_properties.get_property(FightPropType::FIGHT_PROP_CUR_HP);
    let max_hp = fight_properties.get_property(FightPropType::FIGHT_PROP_MAX_HP);

    let ability = {
        if !config.json_name.is_empty() {
            Ability::new_for_gadget(&config.json_name)
        } else {
            Ability::default()
        }
    };

    let inst = ability.instantiate();

    let protocol_entity_id = to_protocol_entity_id(
        protocol_version.as_str(),
        ProtEntityType::ProtEntityGadget,
        entity_counter.inc(),
    );
    let entity_id = protocol_entity_id.0;

    let mut is_interactive = is_interactive;
    let mut gadget_content = gadget_content;
    let mut state = state;

    if gadget_content.is_none() {
        match config.r#type {
            EntityType::Worktop | EntityType::SealGadget => {
                gadget_content = Some(Content::Worktop(WorktopInfo {
                    option_list: vec![],
                    is_guest_can_operate: true,
                }));
            }
            EntityType::RewardPoint
            | EntityType::RewardStatue
            | EntityType::MpPlayRewardPoint
            | EntityType::GeneralRewardPoint => {
                is_interactive = true;
                state = GadgetState::StatueActive as u32;
            }
            _ => {}
        }
    }

    let gadget_entity = commands.spawn(GadgetBundle {
        gadget_id: GadgetID(gadget_id),
        entity_id: protocol_entity_id,
        owner_entity_id: OwnerProtocolEntityID(None),
        level: Level(level),
        interactive: Interactive(config.is_interactive || is_interactive),
        gadget_content: GadgetContent(gadget_content),
        drop_tag: DropTag(drop_tag),
        chest_drop_id: ChestDropId(chest_drop_id),
        state: State(state),
        transform: Transform { position, rotation },
        fight_properties,
        instanced_abilities: inst,
        instanced_modifiers: InstancedModifiers::default(),
        global_ability_values: GlobalAbilityValues::default(),
        life_state: LifeState::Alive,
    });

    Some((entity_id, gadget_entity.id(), cur_hp, max_hp))
}

pub fn handle_gadget_interact(
    mut events: MessageReader<GadgetInteractEvent>,
    index: Res<EntityById>,
    mut commands: Commands,
    players: Res<Players>,
    gadgets: Query<(
        &Level,
        &Transform,
        Option<&GadgetContent>,
        &DropTag,
        &ChestDropId,
    )>,
    mut item_add_events: MessageWriter<ItemAddEvent>,
    mut item_drop_events: MessageWriter<ItemDropEvent>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    let gadget_excel_config_collection_clone =
        std::sync::Arc::clone(gadget_excel_config_collection::get());

    let env_animal_gather_excel_config_collection_clone = std::sync::Arc::clone(
        nod_krai_gi_data::excel::env_animal_gather_excel_config_collection::get(),
    );

    for GadgetInteractEvent(player_uid, gadget_id, gadget_entity_id) in events.read() {
        match index.0.get(&gadget_entity_id) {
            None => {}
            Some(entity) => match gadgets.get(*entity) {
                Ok((level, transform, gadget_content, drop_tag, chest_drop_id)) => {
                    // animal
                    match env_animal_gather_excel_config_collection_clone.get(gadget_id) {
                        None => {}
                        Some(gather_config) => {
                            for gather_item in &gather_config.gather_item_list {
                                if gather_item.id != 0 {
                                    item_add_events.write(ItemAddEvent(
                                        *player_uid,
                                        vec![(
                                            gather_item.id,
                                            None,
                                            None,
                                            None,
                                            None,
                                            HashMap::new(),
                                        )],
                                    ));
                                }
                            }

                            disappear_events.write(EntityDisappearEvent(
                                *gadget_entity_id,
                                VisionType::VisionMiss.into(),
                            ));

                            commands.entity(*entity).insert(ToBeRemovedMarker);
                            continue;
                        }
                    }

                    // gather
                    match gadget_content {
                        None => {}
                        Some(gadget_content) => match &gadget_content.0 {
                            None => {}
                            Some(gadget_content) => match gadget_content {
                                Content::TrifleGadget(trifle_gadget) => match &trifle_gadget.item {
                                    None => {}
                                    Some(item) => {
                                        let mut count = 1;
                                        match &item.detail {
                                            None => {}
                                            Some(detail) => match detail {
                                                Detail::Material(material) => {
                                                    count = material.count
                                                }
                                                _ => {}
                                            },
                                        }
                                        item_add_events.write(ItemAddEvent(
                                            *player_uid,
                                            vec![(
                                                item.item_id,
                                                Some(count),
                                                None,
                                                None,
                                                None,
                                                HashMap::new(),
                                            )],
                                        ));

                                        disappear_events.write(EntityDisappearEvent(
                                            *gadget_entity_id,
                                            VisionType::VisionGatherEscape.into(),
                                        ));

                                        commands.entity(*entity).insert(ToBeRemovedMarker);
                                        continue;
                                    }
                                },
                                Content::GatherGadget(gather_gadget) => {
                                    item_add_events.write(ItemAddEvent(
                                        *player_uid,
                                        vec![(
                                            gather_gadget.item_id,
                                            None,
                                            None,
                                            None,
                                            None,
                                            HashMap::new(),
                                        )],
                                    ));

                                    disappear_events.write(EntityDisappearEvent(
                                        *gadget_entity_id,
                                        VisionType::VisionGatherEscape.into(),
                                    ));

                                    commands.entity(*entity).insert(ToBeRemovedMarker);
                                    continue;
                                }
                                _ => {}
                            },
                        },
                    }

                    // chest
                    let mut drop_id = 0;
                    match &drop_tag.0 {
                        None => {}
                        Some(drop_tag) => {
                            match CombinedDrop::get_drop_config(drop_tag.clone(), level.0) {
                                None => {}
                                Some(drop_config) => {
                                    drop_id = drop_config.drop_id;
                                }
                            }
                        }
                    }
                    if drop_id == 0 {
                        drop_id = chest_drop_id.0;
                    }
                    if drop_id != 0 {
                        tracing::debug!("drop_id is {}", drop_id);
                        let drop_vec = resolve_drop(drop_id, 1);
                        tracing::debug!("drop_vec is {:#?}", drop_vec);
                        if !drop_vec.is_empty() {
                            item_drop_events.write(ItemDropEvent(
                                *player_uid,
                                Some((
                                    transform.position.x,
                                    transform.position.y + 0.5,
                                    transform.position.z,
                                )),
                                drop_vec,
                            ));
                        }

                        disappear_events.write(EntityDisappearEvent(
                            *gadget_entity_id,
                            VisionType::VisionGatherEscape.into(),
                        ));

                        commands.entity(*entity).insert(ToBeRemovedMarker);
                        continue;
                    }

                    // dungeon
                    match gadget_excel_config_collection_clone.get(gadget_id) {
                        None => {}
                        Some(gadget_config) => {
                            if gadget_config.r#type == EntityType::RewardStatue {
                                let Some(player_info) = players.get(*player_uid) else {
                                    continue;
                                };

                                let Some(ref player_dungeon_bin) = player_info.dungeon_bin else {
                                    continue;
                                };

                                let dungeon_excel_config_collection_clone = std::sync::Arc::clone(
                                    nod_krai_gi_data::excel::dungeon_excel_config_collection::get(),
                                );

                                match dungeon_excel_config_collection_clone
                                    .get(&player_dungeon_bin.cur_dungeon_id)
                                {
                                    None => {}
                                    Some(dungeon_config) => {
                                        if dungeon_config.statue_drop != 0 {
                                            let drop_id = dungeon_config.statue_drop;
                                            tracing::debug!("cur_dungeon_id is {}", player_dungeon_bin.cur_dungeon_id);
                                            tracing::debug!("drop_id is {}", drop_id);
                                            let drop_vec = resolve_drop(drop_id, 1);
                                            tracing::debug!("drop_vec is {:#?}", drop_vec);
                                            if !drop_vec.is_empty() {
                                                item_drop_events.write(ItemDropEvent(
                                                    *player_uid,
                                                    None,
                                                    drop_vec,
                                                ));
                                            }

                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            },
        }
    }
}

pub fn handle_set_worktop_options(
    mut events: MessageReader<SetWorktopOptionsEvent>,
    index: Res<EntityById>,
    mut gadgets: Query<&mut GadgetContent>,
    message_output: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::normal::WorktopOptionNotify;

    for SetWorktopOptionsEvent {
        player_uid,
        group_id,
        config_id,
        gadget_entity_id,
        option_list,
        del_option,
    } in events.read()
    {
        tracing::debug!(
            "SetWorktopOptionsEvent: player_uid={}, group_id={}, config_id={}, gadget_entity_id={}, option_list={:?}, del_option={}",
            player_uid,
            group_id,
            config_id,
            gadget_entity_id,
            option_list,
            del_option
        );

        if let Some(entity) = index.0.get(gadget_entity_id) {
            if let Ok(mut gadget_content) = gadgets.get_mut(*entity) {
                match &mut gadget_content.0 {
                    None => {}
                    Some(ref mut gadget_content) => {
                        match gadget_content {
                            Content::NightCrowGadgetInfo(_) => {}
                            Content::DeshretObeliskGadgetInfo(_) => {}
                            Content::AbilityGadget(_) => {}
                            Content::FishPoolInfo(_) => {}
                            Content::FoundationInfo(_) => {}
                            Content::Weather(_) => {}
                            Content::ShellInfo(_) => {}
                            Content::VehicleInfo(_) => {}
                            Content::StatueGadget(_) => {}
                            Content::GeneralReward(_) => {}
                            Content::RoguelikeGadgetInfo(_) => {}
                            Content::CoinCollectOperatorInfo(_) => {}
                            Content::Worktop(ref mut work_top) => {
                                let mut option_list = option_list.clone();

                                if *del_option != 0 {
                                    let mut temp_list = work_top.option_list.clone();
                                    temp_list.retain(|&x| x != *del_option);
                                    option_list = temp_list
                                }

                                work_top.option_list = option_list.clone();

                                message_output.send(
                                    *player_uid,
                                    "WorktopOptionNotify",
                                    WorktopOptionNotify {
                                        gadget_entity_id: *gadget_entity_id,
                                        option_list,
                                        ..Default::default()
                                    },
                                );
                            }
                            Content::OfferingInfo(_) => {}
                            Content::TrifleGadget(_) => {}
                            Content::MpPlayReward(_) => {}
                            Content::GatherGadget(_) => {}
                            Content::ScreenInfo(_) => {}
                            Content::CustomGadgetTreeInfo(_) => {}
                            Content::BlossomChest(_) => {}
                            Content::ClientGadget(_) => {}
                            Content::BossChest(_) => {}
                        }
                        continue;
                    }
                }
            }
        }
    }
}

pub fn handle_gadget_state_change(
    mut events: MessageReader<GadgetStateChangeEvent>,
    mut gadgets: Query<(
        &ProtocolEntityID,
        &mut State,
        &Interactive,
        Option<&GroupId>,
        Option<&ConfigId>,
    )>,
    message_output: Res<MessageOutput>,
    world_owner_uid: Res<WorldOwnerUID>,
) {
    use nod_krai_gi_data::scene::group_entity_state_cache::get_group_entity_state_cache;
    use nod_krai_gi_proto::normal::GadgetStateNotify;

    let cache = get_group_entity_state_cache();

    for GadgetStateChangeEvent {
        entity,
        state_id,
        previous_state_id,
    } in events.read()
    {
        let Ok((entity_id, mut state, interactive, group_id, config_id)) = gadgets.get_mut(*entity)
        else {
            continue;
        };

        tracing::debug!(
            "handle_update_gadget_state: entity_id={}, state={}, previous_state={:?}",
            entity_id.0,
            state_id,
            previous_state_id
        );

        if let (Some(group_id), Some(config_id)) = (group_id, config_id) {
            cache.on_gadget_state_update(world_owner_uid.0, group_id.0, config_id.0, state.0);
        }

        state.0 = *state_id;

        message_output.send_to_all(
            "GadgetStateNotify",
            GadgetStateNotify {
                gadget_entity_id: entity_id.0,
                gadget_state: state.0,
                is_enable_interact: interactive.0,
            },
        );
    }
}

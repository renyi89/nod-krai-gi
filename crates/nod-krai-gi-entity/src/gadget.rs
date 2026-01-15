use crate::{
    common::*,
    int_prop_pair,
    transform::{Transform, Vector3},
    util::create_fight_properties_by_gadget_config,
};
use bevy_ecs::{prelude::*, query::QueryData};
use nod_krai_gi_data::excel::gadget_excel_config_collection;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_proto::{EvtCreateGadgetNotify, EvtDestroyGadgetNotify};

#[derive(Component)]
pub struct GadgetID(pub u32);

#[derive(Bundle)]
pub struct GadgetBundle {
    pub gadget_id: GadgetID,
    pub entity_id: ProtocolEntityID,
    pub owner_entity_id: OwnerProtocolEntityID,
    pub level: Level,
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
    pub transform: &'static Transform,
    pub fight_properties: &'static FightProperties,
    pub instanced_abilities: &'static InstancedAbilities,
    pub instanced_modifiers: &'static InstancedModifiers,
    pub global_ability_values: &'static GlobalAbilityValues,
    pub life_state: &'static LifeState,
}

pub fn notify_appear_gadget_entities(
    gadgets: Query<GadgetQueryReadOnly, Added<Visible>>,
    out: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::*;

    let gadget_excel_config_collection_clone =
        std::sync::Arc::clone(gadget_excel_config_collection::get());

    let mut entity_list: Vec<SceneEntityInfo> = vec![];

    gadgets.iter().for_each(|gadget_data| {
        match gadget_excel_config_collection_clone.get(&gadget_data.gadget_id.0) {
            None => {}
            Some(gadget_config) => {
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
                        is_enable_interact: gadget_config.is_interactive,
                        ..Default::default()
                    })),
                });
            }
        }
    });
    out.send_to_all(
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

pub fn handle_evt_update_gadget(
    index: Res<EntityById>,
    mut events: MessageReader<ClientMessageEvent>,
    mut commands: Commands,
) {
    for message in events.read() {
        match message.message_name() {
            "EvtCreateGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtCreateGadgetNotify>() {
                    let gadget_id = notify.config_id;

                    let gadget_excel_config_collection_clone =
                        std::sync::Arc::clone(gadget_excel_config_collection::get());

                    let Some(config) = gadget_excel_config_collection_clone.get(&gadget_id) else {
                        tracing::debug!("gadget config for id {gadget_id} not found");
                        continue;
                    };

                    let mut fight_properties = create_fight_properties_by_gadget_config(config);
                    fight_properties.apply_base_values();

                    commands.spawn(GadgetBundle {
                        gadget_id: GadgetID(gadget_id),
                        entity_id: ProtocolEntityID(notify.entity_id),
                        owner_entity_id: OwnerProtocolEntityID(notify.owner_entity_id),
                        level: Level(1),
                        transform: Transform {
                            // Take Y (height) from player's pos, spawn a bit above
                            position: Vector3::default(),
                            rotation: Vector3::default(),
                        },
                        fight_properties,
                        instanced_abilities: InstancedAbilities::default(),
                        instanced_modifiers: InstancedModifiers::default(),
                        global_ability_values: GlobalAbilityValues::default(),
                        life_state: LifeState::Alive,
                    });
                }
            }
            "EvtDestroyGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtDestroyGadgetNotify>() {
                    let entity_id = notify.entity_id;
                    let entity = match index.0.get(&entity_id) {
                        Some(e) => *e,
                        None => continue,
                    };
                    commands.entity(entity).insert(ToBeRemovedMarker);
                }
            }
            &_ => {}
        }
    }
}

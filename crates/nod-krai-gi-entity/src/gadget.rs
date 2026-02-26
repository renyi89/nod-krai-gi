use super::ability::Ability;
use crate::util::{create_fight_properties_by_gadget_config, to_protocol_entity_id};
use crate::{common::*, int_prop_pair, transform::Transform};
use bevy_ecs::{prelude::*, query::QueryData};
use nod_krai_gi_data::excel::gadget_excel_config_collection;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::ProtEntityType;
use nod_krai_gi_proto::server_only::VectorBin;
use tracing::debug;

#[derive(Component)]
pub struct GadgetID(pub u32);

#[derive(Component)]
pub struct Interactive(pub bool);

#[derive(Component)]
pub struct State(pub u32);

#[derive(Bundle)]
pub struct GadgetBundle {
    pub gadget_id: GadgetID,
    pub entity_id: ProtocolEntityID,
    pub owner_entity_id: OwnerProtocolEntityID,
    pub level: Level,
    pub interactive: Interactive,
    pub state: State,
    pub transform: Transform,
    pub fight_properties: FightProperties,
    pub ability: Ability,
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
    pub state: &'static State,
    pub transform: &'static Transform,
    pub fight_properties: &'static FightProperties,
    pub ability: &'static Ability,
    pub instanced_abilities: &'static InstancedAbilities,
    pub instanced_modifiers: &'static InstancedModifiers,
    pub global_ability_values: &'static GlobalAbilityValues,
    pub life_state: &'static LifeState,
}

pub fn notify_appear_gadget_entities(
    gadgets: Query<(GadgetQueryReadOnly, Option<&GroupId>, Option<&ConfigId>), Added<Visible>>,
    out: Res<MessageOutput>,
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
                    ..Default::default()
                })),
                ..Default::default()
            });
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

pub fn spawn_gadget_entity(
    commands: &mut Commands,
    entity_counter: &mut ResMut<EntityCounter>,
    position: VectorBin,
    rotation: VectorBin,
    gadget_id: u32,
    level: u32,
    state: u32,
    is_interactive: bool,
) -> Option<Entity> {
    let gadget_excel_config_collection_clone =
        std::sync::Arc::clone(gadget_excel_config_collection::get());

    let Some(config) = gadget_excel_config_collection_clone.get(&gadget_id) else {
        debug!("gadget config for id {gadget_id} not found");
        return None;
    };

    let mut fight_properties = create_fight_properties_by_gadget_config(config);
    fight_properties.apply_base_values();

    let ability = {
        if !config.json_name.is_empty() {
            Ability::new_for_gadget(&config.json_name)
        } else {
            Ability::default()
        }
    };

    let gadget_entity = commands.spawn(GadgetBundle {
        gadget_id: GadgetID(gadget_id),
        entity_id: to_protocol_entity_id(ProtEntityType::ProtEntityGadget, entity_counter.inc()),
        owner_entity_id: OwnerProtocolEntityID(None),
        level: Level(level),
        interactive: Interactive(config.is_interactive || is_interactive),
        state: State(state),
        transform: Transform { position, rotation },
        fight_properties,
        ability,
        instanced_abilities: InstancedAbilities::default(),
        instanced_modifiers: InstancedModifiers::default(),
        global_ability_values: GlobalAbilityValues::default(),
        life_state: LifeState::Alive,
    });

    Some(gadget_entity.id())
}

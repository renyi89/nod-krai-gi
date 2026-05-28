use crate::gadget::GadgetID;
use crate::util::{create_fight_properties_by_monster_config, to_protocol_entity_id};
use crate::weapon::{AffixMap, WeaponBundle, WeaponID, WeaponPromoteLevel, WeaponQueryReadOnly};
use crate::{common::*, int_prop_pair, transform::Transform};
use bevy_ecs::{prelude::*, query::QueryData};
use nod_krai_gi_data::excel::{gadget_excel_config_collection, monster_excel_config_collection};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::ProtEntityType;
use nod_krai_gi_proto::server_only::VectorBin;
use std::collections::HashMap;
use tracing::debug;

#[derive(Component)]
pub struct MonsterEquipment(pub Vec<Entity>);

#[derive(Component)]
pub struct MonsterID(pub u32);

#[derive(Component)]
pub struct AffixList(pub Vec<u32>);

#[derive(Component)]
pub struct PoseId(pub u32);

#[derive(Component)]
pub struct TitleId(pub u32);

#[derive(Component)]
pub struct SpecialNameId(pub u32);

#[derive(Bundle)]
pub struct MonsterBundle {
    pub monster_id: MonsterID,
    pub entity_id: ProtocolEntityID,
    pub level: Level,
    pub monster_equipment: MonsterEquipment,
    pub affix_list: AffixList,
    pub pose_id: PoseId,
    pub title_id: TitleId,
    pub special_name_id: SpecialNameId,
    pub drop_tag: DropTag,
    pub chest_drop_id: ChestDropId,
    pub transform: Transform,
    pub fight_properties: FightProperties,
    pub instanced_abilities: InstancedAbilities,
    pub instanced_modifiers: InstancedModifiers,
    pub global_ability_values: GlobalAbilityValues,
    pub life_state: LifeState,
}

#[derive(QueryData)]
pub struct MonsterQueryReadOnly {
    pub monster_id: &'static MonsterID,
    pub entity_id: &'static ProtocolEntityID,
    pub level: &'static Level,
    pub monster_equipment: &'static MonsterEquipment,
    pub affix_list: &'static AffixList,
    pub pose_id: &'static PoseId,
    pub title_id: &'static TitleId,
    pub special_name_id: &'static SpecialNameId,
    pub drop_tag: &'static DropTag,
    pub chest_drop_id: &'static ChestDropId,
    pub transform: &'static Transform,
    pub fight_properties: &'static FightProperties,
    pub instanced_abilities: &'static InstancedAbilities,
    pub instanced_modifiers: &'static InstancedModifiers,
    pub global_ability_values: &'static GlobalAbilityValues,
    pub life_state: &'static LifeState,
}

pub fn notify_appear_monster_entities(
    monsters: Query<
        (
            MonsterQueryReadOnly,
            Option<&BlockId>,
            Option<&GroupId>,
            Option<&ConfigId>,
        ),
        Added<Visible>,
    >,
    weapon_query: Query<WeaponQueryReadOnly>,
    message_output: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::normal::*;

    message_output.send_to_all(
        "SceneEntityAppearNotify",
        SceneEntityAppearNotify {
            appear_type: VisionType::VisionBorn.into(),
            param: 0,
            entity_list: monsters
                .iter()
                .map(
                    |(monster_data, block_id, group_id, config_id)| SceneEntityInfo {
                        entity_type: ProtEntityType::ProtEntityMonster.into(),
                        entity_id: monster_data.entity_id.0,
                        name: String::new(),
                        motion_info: Some(MotionInfo {
                            pos: Some(monster_data.transform.position.into()),
                            rot: Some(monster_data.transform.rotation.into()),
                            speed: Some(Vector::default()),
                            ..Default::default()
                        }),
                        prop_list: vec![int_prop_pair!(PROP_LEVEL, monster_data.level.0)],
                        fight_prop_list: monster_data
                            .fight_properties
                            .0
                            .iter()
                            .map(|(k, v)| FightPropPair {
                                prop_type: *k as u32,
                                prop_value: *v,
                            })
                            .collect(),
                        life_state: *monster_data.life_state as u32,
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
                            born_pos: Some(monster_data.transform.position.into()),
                            client_extra_info: Some(EntityClientExtraInfo {
                                skill_anchor_position: Some(Vector::default()),
                            }),
                            renderer_changed_info: Some(EntityRendererChangedInfo::default()),
                            pose_para_list: Vec::with_capacity(0),
                            ..Default::default()
                        }),
                        tag_list: Vec::with_capacity(0),
                        server_buff_list: Vec::with_capacity(0),
                        entity: Some(scene_entity_info::Entity::Monster(SceneMonsterInfo {
                            monster_id: monster_data.monster_id.0,
                            born_type: MonsterBornType::MonsterBornDefault as i32,
                            block_id: block_id.and_then(|t| Some(t.0)).unwrap_or_default(),
                            group_id: group_id.and_then(|t| Some(t.0)).unwrap_or_default(),
                            config_id: config_id.and_then(|t| Some(t.0)).unwrap_or_default(),
                            affix_list: monster_data.affix_list.0.clone(),
                            init_pose_id: monster_data.pose_id.0,
                            pose_id: monster_data.pose_id.0,
                            title_id: monster_data.title_id.0,
                            special_name_id: monster_data.special_name_id.0,
                            weapon_list: monster_data
                                .monster_equipment
                                .0
                                .iter()
                                .filter_map(|equip_entity| {
                                    let Ok(weapon_data) = weapon_query.get(*equip_entity) else {
                                        return None;
                                    };
                                    Some(SceneWeaponInfo {
                                        entity_id: weapon_data.entity_id.0,
                                        gadget_id: weapon_data.gadget_id.0,
                                        ability_info: Some(AbilitySyncStateInfo::default()),
                                        ..Default::default()
                                    })
                                })
                                .collect(),
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                )
                .collect(),
        },
    );
}

pub fn run_if_monster_entities_appeared(
    monsters: Query<MonsterQueryReadOnly, Added<Visible>>,
) -> bool {
    !monsters.is_empty()
}

pub fn spawn_monster_entity(
    protocol_version: String,
    commands: &mut Commands,
    entity_counter: &mut ResMut<EntityCounter>,
    position: VectorBin,
    rotation: VectorBin,
    monster_id: u32,
    level: u32,
    pose_id: u32,
    title_id: u32,
    special_name_id: u32,
    drop_tag: Option<String>,
    chest_drop_id: u32,
) -> Option<(u32, Entity, f32, f32)> {
    let monster_excel_config_collection_clone =
        std::sync::Arc::clone(monster_excel_config_collection::get());

    let gadget_excel_config_collection_clone =
        std::sync::Arc::clone(gadget_excel_config_collection::get());

    let Some(config) = monster_excel_config_collection_clone.get(&monster_id) else {
        debug!("monster config for id {monster_id} not found");
        return None;
    };

    let mut fight_properties = create_fight_properties_by_monster_config(config);
    for grow_curve in config.prop_grow_curves.iter() {
        fight_properties.apply_grow_curve(level, grow_curve, GrowCurveConfigType::Monster);
    }
    fight_properties.apply_base_values();

    let cur_hp = fight_properties.get_property(FightPropType::FIGHT_PROP_CUR_HP);
    let max_hp = fight_properties.get_property(FightPropType::FIGHT_PROP_MAX_HP);

    let mut title_id = title_id;
    let mut special_name_id = special_name_id;
    if title_id == 0 && special_name_id == 0 {
        match &config.describe {
            None => {}
            Some(describe) => {
                title_id = describe.title_id;
                special_name_id = describe.special_name_id.unwrap_or(0);
            }
        }
    }

    let protocol_entity_id = to_protocol_entity_id(
        protocol_version.as_str(),
        ProtEntityType::ProtEntityMonster,
        entity_counter.inc(),
    );
    let entity_id = protocol_entity_id.0;

    let monster_equipment = config
        .equips
        .iter()
        .filter_map(|equip_id| {
            if *equip_id == 0 {
                return None;
            }
            let Some(config) = gadget_excel_config_collection_clone.get(&equip_id) else {
                debug!("gadget config for id {} not found", equip_id);
                return None;
            };

            debug!("monster_id:{} equip:{}", monster_id, equip_id);

            let weapon_entity = commands
                .spawn(WeaponBundle {
                    weapon_id: WeaponID(0),
                    entity_id: to_protocol_entity_id(
                        protocol_version.as_str(),
                        ProtEntityType::ProtEntityWeapon,
                        entity_counter.inc(),
                    ),
                    level: Level(0),
                    guid: Guid(0),
                    gadget_id: GadgetID(config.id),
                    affix_map: AffixMap(HashMap::default()),
                    promote_level: WeaponPromoteLevel(0),
                })
                .id();

            Some(weapon_entity)
        })
        .collect::<Vec<_>>();

    let monster_entity = commands.spawn(MonsterBundle {
        monster_id: MonsterID(monster_id),
        entity_id: protocol_entity_id,
        level: Level(level),
        monster_equipment: MonsterEquipment(monster_equipment),
        affix_list: AffixList(config.affix.clone()),
        pose_id: PoseId(pose_id),
        title_id: TitleId(title_id),
        special_name_id: SpecialNameId(special_name_id),
        drop_tag: DropTag(drop_tag),
        chest_drop_id: ChestDropId(chest_drop_id),
        transform: Transform { position, rotation },
        fight_properties,
        instanced_abilities: InstancedAbilities::default(),
        instanced_modifiers: InstancedModifiers::default(),
        global_ability_values: GlobalAbilityValues::default(),
        life_state: LifeState::Alive,
    });

    Some((entity_id, monster_entity.id(), cur_hp, max_hp))
}

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel;
use nod_krai_gi_entity::avatar::CurrentTeam;
use nod_krai_gi_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker, IndexInSceneTeam},
    common::ToBeRemovedMarker,
    int_prop_pair,
    weapon::WeaponQueryReadOnly,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    scene_entity_info, AbilitySyncStateInfo, AnimatorParameterValueInfo,
    AnimatorParameterValueInfoPair, AvatarExcelInfo, AvatarSkillInfo, AvatarSkillInfoNotify,
    EntityAuthorityInfo, EntityClientData, EntityClientExtraInfo, EntityRendererChangedInfo,
    FightPropPair, MotionInfo, ProtEntityType, SceneAvatarInfo, SceneEntityInfo, SceneTeamAvatar,
    SceneTeamUpdateNotify, SceneWeaponInfo, Vector,
};

pub fn notify_scene_team_update(
    mut scene_team_update_events: MessageReader<SceneTeamUpdateEvent>,
    avatar_query: Query<
        (
            AvatarQueryReadOnly,
            &IndexInSceneTeam,
            Option<&CurrentPlayerAvatarMarker>,
        ),
        (With<CurrentTeam>, Without<ToBeRemovedMarker>),
    >,
    weapon_query: Query<WeaponQueryReadOnly>,
    players: Res<Players>,
    message_output: Res<MessageOutput>,
) {
    let avatar_skill_depot_excel_config_collection_clone =
        std::sync::Arc::clone(excel::avatar_skill_depot_excel_config_collection::get());

    for _ in scene_team_update_events.read() {
        message_output.send_to_all(
            "SceneTeamUpdateNotify",
            SceneTeamUpdateNotify {
                scene_team_avatar_list: avatar_query
                    .iter()
                    .sort::<&IndexInSceneTeam>()
                    .filter_map(|(avatar_data, _, is_cur)| {
                        let Ok(weapon_data) = weapon_query.get(avatar_data.equipment_weapon.weapon) else {
                            tracing::debug!(
                                "weapon data {} doesn't exist",
                                avatar_data.equipment_weapon.weapon
                            );
                            return None;
                        };

                        let Some(skill_depot_data) =
                            avatar_skill_depot_excel_config_collection_clone
                                .get(&avatar_data.skill_depot.0)
                                .cloned()
                        else {
                            tracing::debug!(
                                "avatar skill depot config {} doesn't exist",
                                avatar_data.skill_depot.0
                            );
                            return None;
                        };

                        let Some(player_info) = players.get(avatar_data.owner_player_uid.0) else {
                            return None;
                        };
                        let Some(ref player_avatar_bin) = player_info.avatar_bin else {
                            return None;
                        };
                        let Some(ref player_scene_bin) = player_info.scene_bin else {
                            return None;
                        };
                        let Some(avatar_bin) = player_avatar_bin.avatar_map.get(&avatar_data.guid.0) else {
                            tracing::debug!("avatar guid {} doesn't exist", avatar_data.guid.0);
                            return None;
                        };
                        Some(SceneTeamAvatar {
                            is_on_scene: true,
                            is_player_cur_avatar: is_cur.is_some(),
                            is_reconnect: false,
                            avatar_guid: avatar_data.guid.0,
                            weapon_guid: weapon_data.guid.0,
                            entity_id: avatar_data.entity_id.0,
                            weapon_entity_id: weapon_data.entity_id.0,
                            avatar_info: None,
                            scene_avatar_info: None,
                            scene_id: player_scene_bin.my_cur_scene_id,
                            player_uid: avatar_data.owner_player_uid.0,
                            server_buff_list: Vec::with_capacity(0),
                            ability_control_block: Some(avatar_data.ability.build_control_block()),
                            avatar_ability_info: Some(AbilitySyncStateInfo::default()),
                            weapon_ability_info: Some(AbilitySyncStateInfo::default()),
                            scene_entity_info: Some(SceneEntityInfo {
                                entity_type: ProtEntityType::ProtEntityAvatar.into(),
                                entity_id: avatar_data.entity_id.0,
                                name: String::with_capacity(0),
                                motion_info: Some(MotionInfo {
                                    pos: Some(player_scene_bin.my_prev_pos.unwrap_or_default().into()),
                                    rot: Some(player_scene_bin.my_prev_rot.unwrap_or_default().into()),
                                    speed: Some(Vector::default()),
                                    ..Default::default()
                                }),
                                prop_list: vec![
                                    int_prop_pair!(PROP_LEVEL, avatar_data.level.0),
                                    int_prop_pair!(PROP_BREAK_LEVEL, avatar_data.break_level.0),
                                ],
                                fight_prop_list: avatar_data
                                    .fight_properties
                                    .0
                                    .iter()
                                    .map(|(k, v)| FightPropPair {
                                        prop_type: *k as u32,
                                        prop_value: *v,
                                    })
                                    .collect(),
                                life_state: *avatar_data.life_state as u32,
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
                                    born_pos: Some(Vector::default()),
                                    client_extra_info: Some(EntityClientExtraInfo {
                                        skill_anchor_position: Some(Vector::default()),
                                    }),
                                    ..Default::default()
                                }),
                                tag_list: Vec::with_capacity(0),
                                server_buff_list: Vec::with_capacity(0),
                                entity: Some(scene_entity_info::Entity::Avatar(SceneAvatarInfo {
                                    uid: avatar_data.owner_player_uid.0,
                                    avatar_id: avatar_data.avatar_id.0,
                                    guid: avatar_data.guid.0,
                                    peer_id: avatar_data.control_peer.0,
                                    equip_id_list: vec![weapon_data.weapon_id.0],
                                    skill_depot_id: avatar_data.skill_depot.0,
                                    talent_id_list: if avatar_data.core_proud_skill_level.0 as usize
                                        > skill_depot_data.talents.len()
                                    {
                                        skill_depot_data.talents
                                    } else {
                                        skill_depot_data.talents
                                            [0..avatar_data.core_proud_skill_level.0 as usize]
                                            .to_vec()
                                    },
                                    core_proud_skill_level: avatar_data.core_proud_skill_level.0,
                                    weapon: Some(SceneWeaponInfo {
                                        guid: weapon_data.guid.0,
                                        entity_id: weapon_data.entity_id.0,
                                        gadget_id: weapon_data.gadget_id.0,
                                        item_id: weapon_data.weapon_id.0,
                                        level: weapon_data.level.0,
                                        promote_level: weapon_data.promote_level.0,
                                        affix_map: weapon_data.affix_map.0.clone(),
                                        ability_info: Some(AbilitySyncStateInfo::default()),
                                        renderer_changed_info: Some(
                                            EntityRendererChangedInfo::default(),
                                        ),
                                        ..Default::default()
                                    }),
                                    reliquary_list: avatar_bin.get_scene_reliquary_info_list(),
                                    inherent_proud_skill_list: avatar_data
                                        .inherent_proud_skill_list
                                        .0
                                        .clone(),
                                    skill_level_map: avatar_data.skill_level_map.0.clone(),
                                    proud_skill_extra_level_map: HashMap::with_capacity(0),
                                    server_buff_list: Vec::with_capacity(0),
                                    team_resonance_list: Vec::with_capacity(0),
                                    wearing_flycloak_id: avatar_data.appearance.flycloak_id,
                                    born_time: avatar_data.born_time.0,
                                    costume_id: avatar_data.appearance.costume_id,
                                    trace_effect_id: avatar_data.appearance.trace_effect_id,
                                    weapon_skin_id: 0,
                                    cur_vehicle_info: None,
                                    excel_info: Some(AvatarExcelInfo::default()),
                                    anim_hash: 0,
                                    ..Default::default()
                                })),
                                ..Default::default()
                            }),
                        })
                    })
                    .collect(),
                is_in_mp: false,
            },
        );
        avatar_query.iter().for_each(|(avatar_data, _, _)| {
            message_output.send(
                avatar_data.owner_player_uid.0,
                "AvatarSkillInfoNotify",
                AvatarSkillInfoNotify {
                    guid: avatar_data.guid.0,
                    skill_map: avatar_data
                        .skill_extra_charge_map
                        .0
                        .iter()
                        .map(|(k, v)| {
                            (
                                *k,
                                AvatarSkillInfo {
                                    max_charge_count: *v,
                                    ..Default::default()
                                },
                            )
                        })
                        .collect(),
                },
            );
        });
    }
}

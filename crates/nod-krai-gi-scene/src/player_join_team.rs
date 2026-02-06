use crate::common::ScenePeerManager;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::{avatar_excel_config_collection, weapon_excel_config_collection};
use nod_krai_gi_entity::avatar::{AvatarQueryReadOnly, CurrentTeam};
use nod_krai_gi_entity::{
    ability::Ability,
    avatar::{
        AvatarAppearance, AvatarBundle, AvatarID, BornTime, ControlPeer, CurrentPlayerAvatarMarker,
        Equipment, IndexInSceneTeam, InherentProudSkillList, SkillDepot, SkillExtraChargeMap,
        SkillLevelMap,
    },
    common::*,
    transform::Transform,
    util::to_protocol_entity_id,
    weapon::{AffixMap, PromoteLevel, WeaponBundle, WeaponID},
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_persistence::{player_information::ItemInformation, Players};
use nod_krai_gi_proto::ProtEntityType;

pub fn player_join_team(
    mut events: MessageReader<PlayerJoinTeamEvent>,
    mut commands: Commands,
    players: Res<Players>,
    peer_mgr: Res<ScenePeerManager>,
    mut entity_counter: ResMut<EntityCounter>,
    mut scene_team_update_events: MessageWriter<SceneTeamUpdateEvent>,
    avatars: Query<(Entity, AvatarQueryReadOnly)>,
) {
    let is_empty = events.is_empty();

    for event in events.read() {
        let weapon_excel_config_collection_clone =
            std::sync::Arc::clone(weapon_excel_config_collection::get());

        let avatar_excel_config_collection_clone =
            std::sync::Arc::clone(avatar_excel_config_collection::get());

        let uid = event.player_uid;
        let player_info = players.get(uid);

        for (idx, to_spawn_guid) in event.avatar_guid_list.iter().enumerate() {
            match avatars
                .iter()
                .find(|(_, data)| data.guid.0 == *to_spawn_guid && data.owner_player_uid.0 == uid)
            {
                Some((avatar_entity, _)) => {
                    commands
                        .entity(avatar_entity)
                        .insert(IndexInSceneTeam(idx as u8))
                        .insert(CurrentTeam)
                        .insert(Transform {
                            position: player_info.world_position.position.into(),
                            rotation: player_info.world_position.rotation.into(),
                        });

                    if *to_spawn_guid == event.appear_avatar_guid {
                        commands
                            .entity(avatar_entity)
                            .insert(Visible)
                            .insert(CurrentPlayerAvatarMarker);
                    }
                }
                None => {
                    let to_spawn = player_info
                        .avatar_module
                        .avatar_map
                        .get(to_spawn_guid)
                        .unwrap();

                    let ItemInformation::Weapon {
                        weapon_id,
                        level,
                        exp: _,
                        promote_level,
                        affix_map,
                        is_locked: _,
                    } = player_info.item_map.get(&to_spawn.weapon_guid).unwrap();

                    let weapon_config =
                        weapon_excel_config_collection_clone.get(weapon_id).unwrap();

                    let weapon_entity = commands
                        .spawn(WeaponBundle {
                            weapon_id: WeaponID(*weapon_id),
                            entity_id: to_protocol_entity_id(
                                ProtEntityType::ProtEntityWeapon,
                                entity_counter.inc(),
                            ),
                            level: Level(*level),
                            guid: Guid(to_spawn.weapon_guid),
                            gadget_id: GadgetID(weapon_config.gadget_id),
                            affix_map: AffixMap(affix_map.clone()),
                            promote_level: PromoteLevel(*promote_level),
                        })
                        .id();

                    let mut avatar_entity = commands.spawn(AvatarBundle {
                        avatar_id: AvatarID(to_spawn.avatar_id),
                        entity_id: to_protocol_entity_id(
                            ProtEntityType::ProtEntityAvatar,
                            entity_counter.inc(),
                        ),
                        guid: Guid(to_spawn.guid),
                        control_peer: ControlPeer(peer_mgr.get_peer_id_by_uid(uid)),
                        skill_depot: SkillDepot(to_spawn.skill_depot_id),
                        core_proud_skill_level: CoreProudSkillLevel(
                            to_spawn.core_proud_skill_level,
                        ),
                        level: Level(to_spawn.level),
                        break_level: BreakLevel(to_spawn.break_level),
                        owner_player_uid: OwnerPlayerUID(player_info.uid),
                        fight_properties: create_fight_props_with_weapon(
                            avatar_excel_config_collection_clone
                                .get(&to_spawn.avatar_id)
                                .unwrap(),
                            to_spawn.cur_hp,
                            to_spawn.level,
                            to_spawn.break_level,
                            weapon_config,
                            *level,
                        ),
                        instanced_abilities: InstancedAbilities::default(),
                        instanced_modifiers: InstancedModifiers::default(),
                        global_ability_values: GlobalAbilityValues::default(),
                        life_state: LifeState::Alive,
                        equipment: Equipment {
                            weapon: weapon_entity,
                        },
                        appearance: AvatarAppearance {
                            flycloak_id: to_spawn.wearing_flycloak_id,
                            costume_id: to_spawn.costume_id,
                            trace_effect_id: to_spawn.trace_effect_id,
                        },
                        transform: Transform {
                            position: player_info.world_position.position.into(),
                            rotation: player_info.world_position.rotation.into(),
                        },
                        ability: Ability::new_for_avatar(
                            to_spawn.avatar_id,
                            to_spawn.open_configs.clone(),
                        ),
                        born_time: BornTime(to_spawn.born_time),
                        index_in_scene_team: IndexInSceneTeam(idx as u8),
                        inherent_proud_skill_list: InherentProudSkillList(
                            to_spawn.inherent_proud_skill_list.clone(),
                        ),
                        skill_level_map: SkillLevelMap(to_spawn.skill_level_map.clone()),
                        skill_extra_charge_map: SkillExtraChargeMap(
                            to_spawn.skill_extra_charge_map.clone(),
                        ),
                    });

                    avatar_entity.insert(CurrentTeam);

                    if *to_spawn_guid == event.appear_avatar_guid {
                        avatar_entity
                            .insert(Visible)
                            .insert(CurrentPlayerAvatarMarker);
                    }
                }
            }
        }
    }

    if !is_empty {
        scene_team_update_events.write(SceneTeamUpdateEvent);
    }
}

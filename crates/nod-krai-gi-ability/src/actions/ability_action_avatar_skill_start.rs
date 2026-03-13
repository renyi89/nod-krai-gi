use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::{EntityById, OwnerProtocolEntityID, SkillCDMap, SkillCDInfo};
use nod_krai_gi_event::ability::*;

use crate::util::eval_option;

pub fn ability_action_avatar_skill_start_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionAvatarSkillStartEvent>,
    mut skill_cd_query: Query<&mut SkillCDMap>,
    abilities_query: Query<(
        Option<&OwnerProtocolEntityID>,
        &nod_krai_gi_entity::common::InstancedAbilities,
    )>,
) {
    for AbilityActionAvatarSkillStartEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entities,
    ) in events.read()
    {
        let Ok((owner_protocol_entity_id, abilities)) = abilities_query.get(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionAvatarSkillStartEvent] Failed to get entity components for {}",
                    ability_entity
                );
            }
            continue;
        };

        let Some(ability) = abilities.list.get(*ability_index as usize).cloned() else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionAvatarSkillStartEvent] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let owner_entity;

        match owner_protocol_entity_id {
            None => {
                owner_entity = Some(*ability_entity);
            }
            Some(owner_protocol_entity_id) => match owner_protocol_entity_id.0 {
                None => {
                    owner_entity = Some(*ability_entity);
                }
                Some(owner_protocol_entity_id) => match index.0.get(&owner_protocol_entity_id) {
                    None => {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!(
                                    "[AbilityActionAvatarSkillStartEvent] owner_protocol_entity_id.0 {} not found",owner_protocol_entity_id);
                        }
                        continue;
                    }
                    Some(temp_entity) => {
                        owner_entity = Some(*temp_entity);
                    }
                },
            },
        }

        match owner_entity {
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!("[AbilityActionAvatarSkillStartEvent] owner_entity.is_none ");
                }
                continue;
            }
            Some(_owner_entity) => {
                let skill_id = action.skill_id;
                let cd_ratio = eval_option(&ability, None, &action.cd_ratio, 1.0);
                let cost_stamina_ratio = eval_option(&ability, None, &action.cost_stamina_ratio, 0.0);

                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[AbilityActionAvatarSkillStartEvent] AvatarSkillStart for {} targets: skill_id={}, cd_ratio={}, cost_stamina_ratio={}",
                        target_entities.len(),
                        skill_id,
                        cd_ratio,
                        cost_stamina_ratio
                    );
                }

                for target_entity in target_entities {
                    let Ok(mut skill_cd_map) = skill_cd_query.get_mut(*target_entity) else {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!(
                                "[AbilityActionAvatarSkillStartEvent] No SkillCDMap found for entity {}",
                                target_entity
                            );
                        }
                        continue;
                    };

                    if skill_id > 0 {
                        start_skill_cd(&mut skill_cd_map, skill_id, cd_ratio);
                    }
                }
            }
        }
    }
}

fn start_skill_cd(skill_cd_map: &mut SkillCDMap, skill_id: u32, cd_ratio: f32) {
    let skill_cd_info = skill_cd_map.0.entry(skill_id).or_insert_with(SkillCDInfo::default);
    
    if cd_ratio != 1.0 {
        let current_cd = skill_cd_info.pass_cd_time as f32;
        let new_cd = (current_cd * cd_ratio) as u32;
        skill_cd_info.pass_cd_time = new_cd;
    }
    
    if GAME_SERVER_CONFIG.plugin.ability_log {
        tracing::debug!(
            "[start_skill_cd] Started skill {} CD: {} (ratio={})",
            skill_id,
            skill_cd_info.pass_cd_time,
            cd_ratio
        );
    }
}

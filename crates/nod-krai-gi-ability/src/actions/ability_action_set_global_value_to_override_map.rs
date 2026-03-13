use crate::util::resolve_target_entity_by_str;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::avatar::{CurrentPlayerAvatarMarker, CurrentTeam};
use nod_krai_gi_entity::common::{
    EntityById, GlobalAbilityValues, InstancedAbilities, OwnerProtocolEntityID, ProtocolEntityID,
};
use nod_krai_gi_entity::team::TeamEntityMarker;
use nod_krai_gi_event::ability::*;

pub fn ability_action_set_global_value_to_override_map_event(
    mut events: MessageReader<AbilityActionSetGlobalValueToOverrideMapEvent>,
    mut abilities_query: Query<&mut InstancedAbilities>,
    global_values_query: Query<&GlobalAbilityValues>,
    entity_by_id: Res<EntityById>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for AbilityActionSetGlobalValueToOverrideMapEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let global_value_key = action.global_value_key;
        let override_map_key = action.override_map_key;
        let ability_formula = action.ability_formula;
        let is_from_owner = action.is_from_owner;

        if global_value_key.is_empty() || override_map_key.is_empty() {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_global_value_to_override_map_event] Missing required keys: global_value_key={}, override_map_key={}",
                    global_value_key,
                    override_map_key
                );
            }
            continue;
        }

        let source_entity = if is_from_owner {
            resolve_target_entity_by_str(
                "Owner",
                *target_entity,
                Some(*target_entity),
                &entity_by_id,
                &entity_query,
            )
        } else {
            Some(*target_entity)
        };

        let Some(source_entity) = source_entity else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_global_value_to_override_map_event] Failed to resolve source entity"
                );
            }
            continue;
        };

        let global_value = match global_values_query.get(source_entity) {
            Ok(global_values) => global_values
                .0
                .get(&global_value_key.into())
                .copied()
                .unwrap_or(0.0),
            Err(_) => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[ability_action_set_global_value_to_override_map_event] Failed to get global values for entity {}",
                        source_entity
                    );
                }
                continue;
            }
        };

        let final_value = calc_ability_formula(ability_formula.as_str(), global_value);

        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_global_value_to_override_map_event] Failed to get abilities for entity {}",
                    ability_entity
                );
            }
            continue;
        };

        if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
            ability
                .ability_specials
                .insert(override_map_key, final_value);

            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_global_value_to_override_map_event] Setting global value {} to override map key {} with value {} (formula: {}, raw: {})",
                    global_value_key,
                    override_map_key,
                    final_value,
                    ability_formula,
                    global_value
                );
            }
        }
    }
}

fn calc_ability_formula(formula: &str, value: f32) -> f32 {
    match formula {
        "DummyThrowSpeed" => value * 30.0 / (0.9424778f32.sin() * 100.0) - 1.0,
        _ => value,
    }
}

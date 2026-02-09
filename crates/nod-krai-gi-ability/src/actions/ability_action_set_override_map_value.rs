use crate::util::eval_option;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::FightProperties;
use nod_krai_gi_event::ability::*;

pub fn ability_action_set_override_map_value_event(
    mut events: MessageReader<AbilityActionSetOverrideMapValueEvent>,
    fight_props_query: Query<&FightProperties>,
    mut abilities_query: Query<&mut nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for AbilityActionSetOverrideMapValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        _target_entity,
    ) in events.read()
    {
        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionSetOverrideMapValueEvent] Failed to get entity components for {}",
                    ability_entity
                );
            }
            continue;
        };

        let Ok(fight_props) = fight_props_query.get(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionSetOverrideMapValueEvent] owner_entity props not found"
                );
            }
            continue;
        };

        let Some(ability) = abilities.list.get_mut(*ability_index as usize) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!("[AbilityActionSetOverrideMapValueEvent] Ability not found for index: {} entity: {}", ability_index, ability_entity);
            }
            continue;
        };

        let override_map_key = action.override_map_key.unwrap_or("".into());

        if override_map_key.is_empty() {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!("[AbilityActionSetOverrideMapValueEvent] Missing override_map_key");
            }
            continue;
        };

        // Calculate value using action.value
        let value = eval_option(ability, Some(&fight_props), &action.value, 0.0);

        // Set value to override map
        ability.ability_specials.insert(override_map_key, value);
        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[AbilityActionSetOverrideMapValueEvent] Setting override map value {} to {}",
                override_map_key,
                value
            );
        }
    }
}

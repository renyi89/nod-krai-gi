use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::InstancedAbilities;
use nod_krai_gi_event::ability::*;
use rand::Rng;

pub fn ability_action_set_random_override_map_value_event(
    mut events: MessageReader<AbilityActionSetRandomOverrideMapValueEvent>,
    mut abilities_query: Query<&mut InstancedAbilities>,
) {
    for AbilityActionSetRandomOverrideMapValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        _target_entity,
    ) in events.read()
    {
        let override_map_key = action.override_map_key;
        let value_range_min = action.value_range_min;
        let value_range_max = action.value_range_max;

        if override_map_key.is_empty() {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_random_override_map_value_event] Missing override_map_key"
                );
            }
            continue;
        }

        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_random_override_map_value_event] Failed to get abilities for entity {}",
                    ability_entity
                );
            }
            continue;
        };

        let mut rng = rand::thread_rng();
        let random_value = if value_range_min < value_range_max {
            rng.gen_range(value_range_min..value_range_max)
        } else {
            rng.gen_range(0.0..1.0)
        };

        if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
            ability.ability_specials.insert(override_map_key, random_value);
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_random_override_map_value_event] Setting random override map key {} to value {} (range: {} - {})",
                    override_map_key,
                    random_value,
                    value_range_min,
                    value_range_max
                );
            }
        }
    }
}

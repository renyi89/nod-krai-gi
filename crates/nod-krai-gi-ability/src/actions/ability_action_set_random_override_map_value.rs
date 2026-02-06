use bevy_ecs::prelude::*;
use nod_krai_gi_event::ability::*;
use rand::Rng;

pub fn ability_action_set_random_override_map_value_event(
    mut events: MessageReader<AbilityActionSetRandomOverrideMapValueEvent>,
    mut abilities_query: Query<&mut nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for AbilityActionSetRandomOverrideMapValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        _target_entity,
    ) in events.read()
    {
        let override_map_key = action.override_map_key.as_deref().unwrap_or("");

        if override_map_key.is_empty() {
            tracing::debug!(
                "[AbilityActionSetRandomOverrideMapValueEvent] Missing override_map_key"
            );
            continue;
        }

        // Get abilities from ability entity (ability_index only applies to ability_entity)
        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            tracing::debug!(
                "[AbilityActionSetRandomOverrideMapValueEvent] Failed to get abilities for entity {}",
                ability_entity
            );
            continue;
        };

        // Generate random value
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..1.0);

        // Set random value to override map
        if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
            ability
                .ability_specials
                .insert(override_map_key.to_string(), random_value);
            tracing::debug!(
                "[AbilityActionSetRandomOverrideMapValueEvent] Setting random override map value {} to {}",
                override_map_key,
                random_value
            );
        }
    }
}

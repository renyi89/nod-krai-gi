use crate::util::eval_option;
use bevy_ecs::prelude::*;

use nod_krai_gi_entity::common::{FightProperties, InstancedAbilities};
use nod_krai_gi_event::ability::ExecuteActionEvent;

pub fn ability_action_set_override_map_value_event(
    mut events: MessageReader<ExecuteActionEvent>,
    fight_props_query: Query<&FightProperties>,
    mut abilities_query: Query<&mut InstancedAbilities>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, _ability_data, _target_entity) in
        events.read()
    {
        if action.type_name != "SetOverrideMapValue" {
            continue;
        }

        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            tracing::debug!(target: "ability",
                "[ability_action_set_override_map_value_event] Failed to get entity components for {}",
                ability_entity
            );
            continue;
        };

        let Ok(fight_props) = fight_props_query.get(*ability_entity) else {
            tracing::debug!(target: "ability",
                "[ability_action_set_override_map_value_event] owner_entity props not found"
            );
            continue;
        };

        let Some(ability) = abilities.list.get_mut(*ability_index as usize) else {
            tracing::debug!(target: "ability", "[ability_action_set_override_map_value_event] Ability not found for index: {} entity: {}", ability_index, ability_entity);
            continue;
        };

        let override_map_key = action.override_map_key;

        if override_map_key.is_empty() {
            tracing::debug!(target: "ability",
                "[ability_action_set_override_map_value_event] Missing override_map_key"
            );
            continue;
        };

        // Calculate value using action.value
        let value = eval_option(ability, Some(&fight_props), &action.value, 0.0);

        // Set value to override map
        ability.ability_specials.insert(override_map_key, value);
        tracing::debug!(target: "ability",
            "[ability_action_set_override_map_value_event] Setting override map value {} to {}",
            override_map_key,
            value
        );
    }
}

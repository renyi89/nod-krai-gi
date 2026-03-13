use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_event::ability::AbilityActionClearGlobalValueEvent;
use nod_krai_gi_entity::common::{InstancedAbilities, GlobalAbilityValues};

pub fn ability_action_clear_global_value_event(
    mut events: MessageReader<AbilityActionClearGlobalValueEvent>,
    abilities_query: Query<&InstancedAbilities>,
    mut global_values_query: Query<&mut GlobalAbilityValues>,
) {
    for AbilityActionClearGlobalValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };
        let Some(_ability) = ability else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_clear_global_value_event] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let key = action.key.as_str();

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_clear_global_value_event] Clear global value: key={}",
                key
            );
        }

        if let Ok(mut global_values) = global_values_query.get_mut(*target_entity) {
            global_values.0.remove(&key.into());
        }
    }
}

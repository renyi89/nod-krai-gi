use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_event::ability::AbilityActionSetGlobalValueEvent;
use nod_krai_gi_entity::common::{InstancedAbilities, GlobalAbilityValues};

use crate::util::{eval_option};

pub fn ability_action_set_global_value_event(
    mut events: MessageReader<AbilityActionSetGlobalValueEvent>,
    abilities_query: Query<&InstancedAbilities>,
    mut global_values_query: Query<&mut GlobalAbilityValues>,
) {
    for AbilityActionSetGlobalValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read() {
        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };
        let Some(ability) = ability else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_set_global_value_event] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let key = action.key.as_str();
        let use_limit_range = action.use_limit_range;

        let value = eval_option(&ability, None, &action.value, 0.0);
        let max_value = eval_option(&ability, None, &action.max_value, 0.0);
        let min_value = eval_option(&ability, None, &action.min_value, 0.0);

        let mut final_value = value;
        if use_limit_range {
            final_value = final_value.max(min_value).min(max_value);
        }

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_set_global_value_event] Set global value: key={}, value={}, use_limit_range={}, max_value={}, min_value={}",
                key,
                final_value,
                use_limit_range,
                max_value,
                min_value
            );
        }

        if let Ok(mut global_values) = global_values_query.get_mut(*target_entity) {
            global_values.0.insert(key.into(), final_value);
        }
    }
}
use bevy_ecs::prelude::*;

use nod_krai_gi_entity::common::{GlobalAbilityValues, InstancedAbilities};
use nod_krai_gi_event::ability::ExecuteActionEvent;

pub fn ability_action_clear_global_value_event(
    mut events: MessageReader<ExecuteActionEvent>,
    abilities_query: Query<&InstancedAbilities>,
    mut global_values_query: Query<&mut GlobalAbilityValues>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, _ability_data, target_entity) in
        events.read()
    {
        if action.type_name != "ClearGlobalValue" {
            continue;
        }

        let target_entity = target_entity.unwrap_or(*ability_entity);

        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };
        let Some(_ability) = ability else {
            tracing::debug!(target: "ability",
                "[ability_action_clear_global_value_event] Ability not found for index: {} entity: {}",
                ability_index,
                ability_entity
            );
            continue;
        };

        let key = action.key.as_str();

        tracing::debug!(target: "ability",
            "[ability_action_clear_global_value_event] Clear global value: key={}",
            key
        );

        if let Ok(mut global_values) = global_values_query.get_mut(target_entity) {
            global_values.0.remove(&key.into());
        }
    }
}

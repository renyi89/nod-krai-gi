use crate::util::eval_option;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;

#[derive(Message)]
pub struct AbilityActionSetOverrideMapValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);
pub fn ability_action_set_override_map_value_event(
    mut events: MessageReader<AbilityActionSetOverrideMapValueEvent>,
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
            tracing::debug!(
                "[AbilityActionSetOverrideMapValueEvent] Failed to get entity components for {}",
                ability_entity
            );
            continue;
        };
        let Some(ability) = abilities.list.get_mut(*ability_index as usize) else {
            tracing::debug!("[AbilityActionSetOverrideMapValueEvent] Ability not found for index: {} entity: {}", ability_index, ability_entity);
            continue;
        };

        let override_map_key = action.override_map_key.as_deref().unwrap_or("");

        if override_map_key.is_empty() {
            tracing::debug!("[AbilityActionSetOverrideMapValueEvent] Missing override_map_key");
            continue;
        };

        // Calculate ratio using action.ratio
        let ratio = eval_option(ability, None, &action.ratio, 0.0);

        // Set value to override map
        ability
            .ability_specials
            .insert(override_map_key.to_string(), ratio);
        tracing::debug!(
            "[AbilityActionSetOverrideMapValueEvent] Setting override map value {} to {}",
            override_map_key,
            ratio
        );
    }
}

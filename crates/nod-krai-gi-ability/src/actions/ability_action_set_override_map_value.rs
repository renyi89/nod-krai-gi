use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;
use nod_krai_gi_entity::common::{InstancedAbility};
use crate::util::eval_option;

#[derive(Message)]
pub struct AbilityActionSetOverrideMapValueEvent(
    pub InstancedAbility,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
    pub Entity,
);
pub fn ability_action_set_override_map_value_event(
    mut events: MessageReader<AbilityActionSetOverrideMapValueEvent>,
    entities: Query<&nod_krai_gi_entity::common::FightProperties>,
) {
    for AbilityActionSetOverrideMapValueEvent(ability, action, _ability_data, entity, _target_entity) in
        events.read()
    {
        let override_map_key = action.override_map_key.as_deref().unwrap_or("");

        if override_map_key.is_empty() {
            tracing::debug!(
                "[AbilityActionSetOverrideMapValueEvent] Missing override_map_key"
            );
            continue;
        }

        // Get fight properties from entity
        let Ok(_fight_props) = entities.get(*entity) else {
            tracing::debug!(
                "[AbilityActionSetOverrideMapValueEvent] Failed to get fight properties for entity {}",
                entity
            );
            continue;
        };

        // Calculate ratio using action.ratio
        let ratio = eval_option(ability, &action.ratio, 0.0);

        // Set value to override map
        // TODO: Implement setting value to override map
        tracing::debug!(
            "[AbilityActionSetOverrideMapValueEvent] Setting override map value {} to {}",
            override_map_key,
            ratio
        );
    }
}

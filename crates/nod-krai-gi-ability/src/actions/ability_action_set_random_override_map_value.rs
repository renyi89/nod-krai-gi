use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;
use nod_krai_gi_entity::common::{InstancedAbility};
use rand::Rng;

#[derive(Message)]
pub struct AbilityActionSetRandomOverrideMapValueEvent(
    pub InstancedAbility,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
    pub Entity,
);
pub fn ability_action_set_random_override_map_value_event(
    mut events: MessageReader<AbilityActionSetRandomOverrideMapValueEvent>,
) {
    for AbilityActionSetRandomOverrideMapValueEvent(_ability, action, _ability_data, _entity, _target_entity) in
        events.read()
    {
        let override_map_key = action.override_map_key.as_deref().unwrap_or("");

        if override_map_key.is_empty() {
            tracing::debug!(
                "[AbilityActionSetRandomOverrideMapValueEvent] Missing override_map_key"
            );
            continue;
        }

        // Generate random value
        // TODO: Implement random value generation based on action parameters
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..1.0);

        // Set random value to override map
        // TODO: Implement setting value to override map
        tracing::debug!(
            "[AbilityActionSetRandomOverrideMapValueEvent] Setting random override map value {} to {}",
            override_map_key,
            random_value
        );
    }
}

use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;

#[derive(Message)]
pub struct AbilityActionSetGlobalValueToOverrideMapEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);
pub fn ability_action_set_global_value_to_override_map_event(
    mut events: MessageReader<AbilityActionSetGlobalValueToOverrideMapEvent>,
    mut abilities_query: Query<&mut nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for AbilityActionSetGlobalValueToOverrideMapEvent(ability_index, ability_entity, action, _ability_data, _target_entity) in
        events.read()
    {
        let global_value_key = action.global_value_key.as_deref().unwrap_or("");
        let override_map_key = action.override_map_key.as_deref().unwrap_or("");
        let ability_formula = action.ability_formula.as_deref().unwrap_or("");

        if global_value_key.is_empty() || override_map_key.is_empty() {
            tracing::debug!(
                "[AbilityActionSetGlobalValueToOverrideMapEvent] Missing required keys: global_value_key={}, override_map_key={}",
                global_value_key,
                override_map_key
            );
            continue;
        }

        // Get abilities from ability entity (ability_index only applies to ability_entity)
        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            tracing::debug!(
                "[AbilityActionSetGlobalValueToOverrideMapEvent] Failed to get abilities for entity {}",
                ability_entity
            );
            continue;
        };

        // Get global value (TODO: Implement proper global value retrieval)
        let mut global_value = 0.0;

        // Special handling for DummyThrowSpeed
        if ability_formula == "DummyThrowSpeed" {
            global_value = global_value * 30.0 / (0.9424778f32.sin() * 100.0) - 1.0;
        }

        // Set value to override map
        if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
            ability.ability_specials.insert(override_map_key.to_string(), global_value);
            tracing::debug!(
                "[AbilityActionSetGlobalValueToOverrideMapEvent] Setting global value {} to override map key {} with value {}",
                global_value_key,
                override_map_key,
                global_value
            );
        }
    }
}

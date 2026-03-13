use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::{FightProperties, InstancedAbilities, InstancedModifiers};
use nod_krai_gi_event::ability::AbilityActionRemoveModifierEvent;
use common::string_util::InternString;

pub fn ability_action_remove_modifier_event(
    mut events: MessageReader<AbilityActionRemoveModifierEvent>,
    mut abilities_query: Query<(&mut InstancedAbilities, &mut InstancedModifiers, Option<&mut FightProperties>)>,
) {
    for event in events.read() {
        let ability_entity = event.1;
        let action = &event.2;
        let target_entity = event.4;
        let modifier_name = action.modifier_name.as_str();

        let Ok((mut abilities, mut modifiers, fight_props)) = abilities_query.get_mut(ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_remove_modifier_event] Entity {} not found or has no InstancedAbilities",
                    ability_entity
                );
            }
            continue;
        };

        let modifier_name_intern = InternString::from(modifier_name);
        let mut found = false;

        for ability in &mut abilities.list {
            if let Some(&modifier_id) = ability.modifiers.get(&modifier_name_intern) {
                if let Some(modifier) = modifiers.get(&modifier_id) {
                    if !modifier.property_deltas.is_empty() {
                        if let Some(mut fight_props) = fight_props {
                            for (prop_type, delta) in &modifier.property_deltas {
                                fight_props.change_property(*prop_type, -delta);
                                if GAME_SERVER_CONFIG.plugin.ability_log {
                                    tracing::debug!(
                                        "[ability_action_remove_modifier_event] Reverted property {:?} by {} for entity {:?}",
                                        prop_type,
                                        delta,
                                        target_entity
                                    );
                                }
                            }
                            fight_props.flush_property();
                        }
                    }

                    modifiers.remove(&modifier_id);
                    ability.modifiers.remove(&modifier_name_intern);

                    if GAME_SERVER_CONFIG.plugin.ability_log {
                        tracing::debug!(
                            "[ability_action_remove_modifier_event] Removed modifier: {} (id: {}) from entity {}",
                            modifier_name,
                            modifier_id,
                            target_entity
                        );
                    }
                    found = true;
                    break;
                }
            }
        }

        if !found {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_remove_modifier_event] Modifier {} not found on entity {}",
                    modifier_name,
                    target_entity
                );
            }
        }
    }
}

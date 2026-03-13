use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_data::ability::get_ability_data;
use nod_krai_gi_entity::common::InstancedAbilities;
use nod_krai_gi_event::ability::{AbilityActionTriggerAbilityEvent, ExecuteActionEvent};
use common::string_util::InternString;

pub fn ability_action_trigger_ability_event(
    mut events: MessageReader<AbilityActionTriggerAbilityEvent>,
    mut abilities_query: Query<&mut InstancedAbilities>,
    mut execute_action_events: MessageWriter<ExecuteActionEvent>,
) {
    for AbilityActionTriggerAbilityEvent(
        _ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let ability_name = action.ability_name.as_str();

        if ability_name.is_empty() {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_trigger_ability_event] Empty ability name for entity {}",
                    ability_entity
                );
            }
            continue;
        }

        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_trigger_ability_event] Failed to get abilities for entity {}",
                    ability_entity
                );
            }
            continue;
        };

        let ability_name_intern = InternString::from(ability_name);
        let new_ability_index = if let Some((idx, _)) = abilities.find_or_add_by_ability_name(ability_name_intern, 0) {
            idx
        } else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_trigger_ability_event] Failed to add ability {} for entity {}",
                    ability_name,
                    ability_entity
                );
            }
            continue;
        };

        let Some(ability_data) = get_ability_data(&ability_name_intern) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_trigger_ability_event] Ability data not found for {}",
                    ability_name
                );
            }
            continue;
        };

        for action in &ability_data.on_ability_start {
            execute_action_events.write(ExecuteActionEvent(
                new_ability_index,
                *ability_entity,
                action.clone(),
                Vec::new(),
                Some(*target_entity),
            ));
        }

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_trigger_ability_event] Triggered ability: {} on entity {} (new_ability_index: {})",
                ability_name,
                target_entity,
                new_ability_index
            );
        }
    }
}

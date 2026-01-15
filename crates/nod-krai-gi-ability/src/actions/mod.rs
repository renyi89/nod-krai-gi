pub(crate) mod ability_action_heal_hp;
pub(crate) mod ability_action_lose_hp;

use crate::server_invoke::ExecuteActionEvent;
use crate::actions::ability_action_heal_hp::AbilityActionHealHPEvent;
use crate::actions::ability_action_lose_hp::AbilityActionLoseHPEvent;
use bevy_ecs::prelude::*;

pub fn execute_action_system(
    mut events: MessageReader<ExecuteActionEvent>,
    mut heal_hp_events: MessageWriter<AbilityActionHealHPEvent>,
    mut lose_hp_events: MessageWriter<AbilityActionLoseHPEvent>,
) {
    for ExecuteActionEvent(ability, action, ability_data, entity, target_entity) in events.read() {
        let type_name = action.type_name.as_deref().unwrap_or("");

        match type_name {
            "HealHP" => match target_entity {
                Some(target_entity) => {
                    heal_hp_events.write(AbilityActionHealHPEvent(
                        ability.clone(),
                        action.clone(),
                        ability_data.clone(),
                        *entity,
                        *target_entity,
                    ));
                }
                _ => {}
            },
            "LoseHP" => match target_entity {
                Some(target_entity) => {
                    lose_hp_events.write(AbilityActionLoseHPEvent(
                        ability.clone(),
                        action.clone(),
                        ability_data.clone(),
                        *entity,
                        *target_entity,
                    ));
                }
                _ => {}
            },
            _ => {
                tracing::debug!("Unhandled action type: {}", type_name);
            }
        }
    }
}

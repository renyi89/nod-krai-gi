pub(crate) mod ability_action_heal_hp;
pub(crate) mod ability_action_lose_hp;
pub(crate) mod ability_action_set_global_value_to_override_map;
pub(crate) mod ability_action_get_hp_paid_debts;
pub(crate) mod ability_action_set_override_map_value;
pub(crate) mod ability_action_set_random_override_map_value;

use crate::server_invoke::ExecuteActionEvent;
use crate::actions::ability_action_heal_hp::AbilityActionHealHPEvent;
use crate::actions::ability_action_lose_hp::AbilityActionLoseHPEvent;
use crate::actions::ability_action_set_global_value_to_override_map::AbilityActionSetGlobalValueToOverrideMapEvent;
use crate::actions::ability_action_get_hp_paid_debts::AbilityActionGetHPPaidDebtsEvent;
use crate::actions::ability_action_set_override_map_value::AbilityActionSetOverrideMapValueEvent;
use crate::actions::ability_action_set_random_override_map_value::AbilityActionSetRandomOverrideMapValueEvent;
use bevy_ecs::prelude::*;

pub fn execute_action_system(
    mut events: MessageReader<ExecuteActionEvent>,
    mut heal_hp_events: MessageWriter<AbilityActionHealHPEvent>,
    mut lose_hp_events: MessageWriter<AbilityActionLoseHPEvent>,
    mut set_global_value_to_override_map_events: MessageWriter<AbilityActionSetGlobalValueToOverrideMapEvent>,
    mut get_hp_paid_debts_events: MessageWriter<AbilityActionGetHPPaidDebtsEvent>,
    mut set_override_map_value_events: MessageWriter<AbilityActionSetOverrideMapValueEvent>,
    mut set_random_override_map_value_events: MessageWriter<AbilityActionSetRandomOverrideMapValueEvent>,
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
            "SetGlobalValueToOverrideMap" => {
                set_global_value_to_override_map_events.write(AbilityActionSetGlobalValueToOverrideMapEvent(
                    ability.clone(),
                    action.clone(),
                    ability_data.clone(),
                    *entity,
                    target_entity.unwrap_or(*entity),
                ));
            },
            "GetHPPaidDebts" => {
                get_hp_paid_debts_events.write(AbilityActionGetHPPaidDebtsEvent(
                    ability.clone(),
                    action.clone(),
                    ability_data.clone(),
                    *entity,
                    target_entity.unwrap_or(*entity),
                ));
            },
            "SetOverrideMapValue" => {
                set_override_map_value_events.write(AbilityActionSetOverrideMapValueEvent(
                    ability.clone(),
                    action.clone(),
                    ability_data.clone(),
                    *entity,
                    target_entity.unwrap_or(*entity),
                ));
            },
            "SetRandomOverrideMapValue" => {
                set_random_override_map_value_events.write(AbilityActionSetRandomOverrideMapValueEvent(
                    ability.clone(),
                    action.clone(),
                    ability_data.clone(),
                    *entity,
                    target_entity.unwrap_or(*entity),
                ));
            },
            _ => {
                tracing::debug!("Unhandled action type: {}", type_name);
            }
        }
    }
}

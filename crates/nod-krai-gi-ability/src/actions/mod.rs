pub(crate) mod ability_action_heal_hp;
pub(crate) mod ability_action_lose_hp;
pub(crate) mod ability_action_set_global_value_to_override_map;
pub(crate) mod ability_action_get_hp_paid_debts;
pub(crate) mod ability_action_set_override_map_value;
pub(crate) mod ability_action_set_random_override_map_value;
pub(crate) mod ability_action_add_hp_debts;
pub(crate) mod ability_action_reduce_hp_debts;

use crate::server_invoke::ExecuteActionEvent;
use crate::actions::ability_action_heal_hp::AbilityActionHealHPEvent;
use crate::actions::ability_action_lose_hp::AbilityActionLoseHPEvent;
use crate::actions::ability_action_set_global_value_to_override_map::AbilityActionSetGlobalValueToOverrideMapEvent;
use crate::actions::ability_action_get_hp_paid_debts::AbilityActionGetHPPaidDebtsEvent;
use crate::actions::ability_action_set_override_map_value::AbilityActionSetOverrideMapValueEvent;
use crate::actions::ability_action_set_random_override_map_value::AbilityActionSetRandomOverrideMapValueEvent;
use crate::actions::ability_action_add_hp_debts::AbilityActionAddHPDebtsEvent;
use crate::actions::ability_action_reduce_hp_debts::AbilityActionReduceHPDebtsEvent;
use bevy_ecs::prelude::*;

pub fn execute_action_system(
    mut events: MessageReader<ExecuteActionEvent>,
    mut heal_hp_events: MessageWriter<AbilityActionHealHPEvent>,
    mut lose_hp_events: MessageWriter<AbilityActionLoseHPEvent>,
    mut set_global_value_to_override_map_events: MessageWriter<AbilityActionSetGlobalValueToOverrideMapEvent>,
    mut get_hp_paid_debts_events: MessageWriter<AbilityActionGetHPPaidDebtsEvent>,
    mut set_override_map_value_events: MessageWriter<AbilityActionSetOverrideMapValueEvent>,
    mut set_random_override_map_value_events: MessageWriter<AbilityActionSetRandomOverrideMapValueEvent>,
    mut add_hp_debts_events: MessageWriter<AbilityActionAddHPDebtsEvent>,
    mut reduce_hp_debts_events: MessageWriter<AbilityActionReduceHPDebtsEvent>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in events.read() {
        let type_name = action.type_name.as_deref().unwrap_or("");

        match type_name {
            "HealHP" => match target_entity {
                Some(target_entity) => {
                    heal_hp_events.write(AbilityActionHealHPEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
                        *target_entity,
                    ));
                }
                _ => {}
            },
            "LoseHP" => match target_entity {
                Some(target_entity) => {
                    lose_hp_events.write(AbilityActionLoseHPEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
                        *target_entity,
                    ));
                }
                _ => {}
            },
            "SetGlobalValueToOverrideMap" => {
                set_global_value_to_override_map_events.write(AbilityActionSetGlobalValueToOverrideMapEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target_entity.unwrap_or(*ability_entity),
                ));
            },
            "GetHPPaidDebts" => {
                get_hp_paid_debts_events.write(AbilityActionGetHPPaidDebtsEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target_entity.unwrap_or(*ability_entity),
                ));
            },
            "SetOverrideMapValue" => {
                set_override_map_value_events.write(AbilityActionSetOverrideMapValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target_entity.unwrap_or(*ability_entity),
                ));
            },
            "SetRandomOverrideMapValue" => {
                set_random_override_map_value_events.write(AbilityActionSetRandomOverrideMapValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target_entity.unwrap_or(*ability_entity),
                ));
            },
            "AddHPDebts" => match target_entity {
                Some(target_entity) => {
                    add_hp_debts_events.write(AbilityActionAddHPDebtsEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
                        *target_entity,
                    ));
                }
                _ => {}
            },
            "ReduceHPDebts" => match target_entity {
                Some(target_entity) => {
                    reduce_hp_debts_events.write(AbilityActionReduceHPDebtsEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
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
